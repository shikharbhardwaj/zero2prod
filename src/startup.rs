use std::net::TcpListener;

use actix_files as fs;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key,
    dev::Server,
    web::{self, Data},
    App, HttpRequest, HttpServer,
};
use actix_web_flash_messages::{storage::CookieMessageStore, FlashMessagesFramework};
use actix_web_lab::middleware::from_fn;
use secrecy::{ExposeSecret, Secret};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing_actix_web::TracingLogger;
use utoipa::{
    openapi::security::{Http, HttpAuthScheme, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    authentication::reject_anonymous_users,
    configuration::{DatabaseSettings, Settings, SignupSettings},
    domain,
    email_client::EmailClient,
    routes::health_check,
    routes::{
        admin_dashboard, change_password, change_password_form, confirm, home, log_out, login,
        login_form, newsletter_issue_form, publish_newsletter, signup, signup_form, subscribe,
    },
};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);
        let email_client = configuration.email_client.client();

        let addr = configuration.application.get_listen_addr();

        let listener = TcpListener::bind(addr).expect("Failed to bind to a local port.");

        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
            email_client,
            configuration.application.base_url,
            configuration.application.hmac_secret,
            configuration.redis_uri,
            configuration.application.signup,
        )
        .await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub struct ApplicationBaseUrl(pub String);

async fn run(
    listener: TcpListener,
    connection: PgPool,
    email_client: EmailClient,
    base_url: String,
    hmac_secret: Secret<String>,
    redis_uri: Secret<String>,
    signup_settings: SignupSettings,
) -> Result<Server, anyhow::Error> {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            crate::routes::health_check,
            crate::routes::subscribe,
            crate::routes::confirm,
            crate::routes::publish_newsletter,
            crate::routes::login,
            crate::routes::signup,
        ),
        components(
            schemas(domain::SubscriptionRequest),
            schemas(domain::SubscriberName),
            schemas(domain::SubscriberEmail),
            schemas(crate::routes::NewsletterRequestBody),
            schemas(crate::routes::LoginFormData),
            schemas(crate::routes::SignupFormData),
        ),
        tags(
            (name = "zero2prod", description = "Newsletter app built following the Rust: Zero to Production book.")
        ),
        modifiers(&SecurityAddon)
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap();

            components.add_security_scheme(
                "http_basic",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Basic)),
            );
        }
    }

    let openapi = ApiDoc::openapi();

    let connection = web::Data::new(connection);
    let email_client = web::Data::new(email_client);
    let base_url = web::Data::new(ApplicationBaseUrl(base_url));
    let signup_settings = web::Data::new(signup_settings);

    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());

    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();

    let redis_store = RedisSessionStore::new(redis_uri.expose_secret()).await?;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(message_framework.clone())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .wrap(TracingLogger::default())
            .service(health_check)
            .service(subscribe)
            .service(confirm)
            .service(home)
            .service(login)
            .service(login_form)
            .service(signup)
            .service(signup_form)
            .service(
                web::scope("/admin")
                    .wrap(from_fn(reject_anonymous_users))
                    .service(admin_dashboard)
                    .service(change_password_form)
                    .service(change_password)
                    .service(newsletter_issue_form)
                    .service(publish_newsletter)
                    .service(log_out),
            )
            .service(fs::Files::new("/assets", "./static/assets"))
            .service(fs::Files::new("/images", "./static/images"))
            .service(web::resource("/favicon.ico").route(web::get().to(favicon)))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .app_data(connection.clone())
            .app_data(email_client.clone())
            .app_data(base_url.clone())
            .app_data(signup_settings.clone())
            .app_data(Data::new(HmacSecret(hmac_secret.clone())))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

#[derive(Clone)]
pub struct HmacSecret(pub Secret<String>);

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

async fn favicon(_req: HttpRequest) -> actix_web::error::Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("./static/favicon.ico")?)
}

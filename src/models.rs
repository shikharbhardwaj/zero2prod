use serde::Deserialize;
use utoipa::ToSchema;

/// Subcription request from a user.
#[derive(Deserialize, ToSchema, Clone, Debug)]
pub(super) struct SubscribeRequest {
    #[schema(example = "John Doe")]
    pub name: String,

    #[schema(example = "jonhdoe@example.com")]
    pub email: String,
}

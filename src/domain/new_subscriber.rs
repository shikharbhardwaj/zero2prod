use utoipa::ToSchema;

use crate::domain::SubscriberName;

use super::SubscriberEmail;

/// Subcription request from a user.
#[derive(ToSchema, Clone, Debug)]
pub struct SubscriptionRequest {
    #[schema(example = "John Doe")]
    pub name: SubscriberName,

    #[schema(example = "jonhdoe@example.com")]
    pub email: SubscriberEmail,
}

use garde::Validate;
use serde::Deserialize;

pub mod handler;

#[derive(Debug, Deserialize, Validate)]
pub struct Subscription {
    #[garde(length(min = 1, max = 256))]
    name: String,
    #[garde(email)]
    email: String,
}

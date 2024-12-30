use serde::Deserialize;

pub mod handler;

#[derive(Debug, Deserialize)]
pub struct Subscription {
    name: String,
    email: String,
}

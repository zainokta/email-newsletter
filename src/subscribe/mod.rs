use serde::Deserialize;

pub mod handler;

#[derive(Deserialize)]
pub struct Subscription {
    name: Option<String>,
    email: Option<String>,
}

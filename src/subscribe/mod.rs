use serde::Deserialize;

pub mod subscription;

#[derive(Deserialize)]
pub struct Subscription {
    name: Option<String>,
    email: Option<String>,
}

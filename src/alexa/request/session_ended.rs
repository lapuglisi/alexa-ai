use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionEndedRequest {
  pub name: String,
}

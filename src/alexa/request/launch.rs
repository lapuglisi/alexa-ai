use crate::alexa::request::{self, GenericRequest, GenericRequestImpl};
use serde::{Deserialize, Serialize, ser::Error};

#[derive(Debug, Deserialize, Serialize)]
pub struct LaunchRequest {
  pub base: GenericRequest,
}

impl GenericRequestImpl for LaunchRequest {
  fn request_type() -> request::RequestType {
    request::RequestType::LaunchRequest
  }

  fn id(&self) -> String {
    self.base.id.to_owned()
  }

  fn timestamp(&self) -> String {
    self.base.timestamp.to_owned()
  }

  fn locale(&self) -> String {
    self.base.locale.to_owned()
  }
}

impl std::fmt::Display for LaunchRequest {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match serde_json::to_string(&self.base) {
      Ok(s) => write!(f, "{}", s),
      Err(e) => Err(std::fmt::Error::custom(e))?,
    }
  }
}

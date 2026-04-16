use serde::{Deserialize, Serialize};

type GenericField = serde_json::Value;

pub mod api;
mod intent;
mod launch;
mod presets;

#[derive(Debug, Deserialize, Serialize)]
enum RequestType {
  LaunchRequest,
  IntentRequest,
  CanCagarIntentRequest,
  SessionEndedRequest,
  Generic,
}

impl std::fmt::Display for RequestType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl std::str::FromStr for RequestType {
  type Err = RequestType;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "LaunchRequest" => Ok(Self::LaunchRequest),
      "IntentRequest" => Ok(Self::IntentRequest),
      "SessionEndedRequest" => Ok(Self::SessionEndedRequest),
      _ => {
        log::info!("RequestType::FromStr({}) fell back to Generic", s);
        Ok(Self::Generic)
      }
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GenericRequest {
  pub request_type: RequestType,
  pub id: String,
  pub timestamp: String,
  pub locale: String,
}

pub trait GenericRequestImpl {
  fn request_type() -> RequestType {
    RequestType::Generic
  }

  fn id(&self) -> String {
    String::new()
  }

  fn timestamp(&self) -> String {
    String::new()
  }

  fn locale(&self) -> String {
    "pt-BR".into()
  }
}

impl Default for GenericRequest {
  fn default() -> Self {
    Self {
      request_type: RequestType::Generic,
      id: String::new(),
      timestamp: String::new(),
      locale: String::from("pt-BR"),
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
struct _AlexaRequestUser {
  #[serde(rename = "userId")]
  id: String,
  #[serde(rename = "accessToken")]
  token: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct _SessionApplication {
  #[serde(rename = "applicationId")]
  id: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct _RequestSession {
  new: bool,
  #[serde(rename = "sessionId")]
  session_id: String,
  application: _SessionApplication,
  attributes: GenericField,
  user: _AlexaRequestUser,
}

#[derive(Debug, Deserialize, Serialize)]
struct _ContextSystemDevice {
  #[serde(rename = "deviceId")]
  id: String,
  #[serde(rename = "supportedInterfaces")]
  supported_ifs: GenericField,
  #[serde(rename = "persistentEndpointId")]
  persistent_endpoint: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct _ContextSystem {
  device: _ContextSystemDevice,
  user: _AlexaRequestUser,

  person: Option<GenericField>,
  unit: Option<GenericField>,

  #[serde(rename = "apiEndpoint")]
  api_endpoint: String,

  #[serde(rename = "apiAccessToken")]
  api_access_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct _RequestContext {
  #[serde(rename = "System")]
  system: _ContextSystem,

  #[serde(rename = "Advertising")]
  advertising: Option<GenericField>,

  #[serde(rename = "AudioPlayer")]
  audio_player: Option<GenericField>,
}

#[derive(Debug, Deserialize, Serialize)]
struct _AlexaRequest {
  body: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AlexaApiRequest {
  version: String,
  session: _RequestSession,
  context: _RequestContext,
  request: GenericField,
}

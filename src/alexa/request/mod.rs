use serde::{Deserialize, Serialize};

type GenericField = serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct GenericRequest {
  #[serde(rename = "type")]
  request_type: String,

  #[serde(rename = "requestId")]
  id: String,

  timestamp: String,

  locale: String,
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
  /*request: _AlexaRequest,*/
}

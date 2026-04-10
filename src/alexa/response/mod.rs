use serde::{Deserialize, Serialize};
/*
{
  "outputSpeech": {
    "type": "PlainText",
    "text": "Plain text string to speak",
    "playBehavior": "REPLACE_ENQUEUED"
  },
  "card": {
    "type": "Standard",
    "title": "Title of the card",
    "text": "Text content for a standard card",
    "image": {
      "smallImageUrl": "https://url-to-small-card-image...",
      "largeImageUrl": "https://url-to-large-card-image..."
    }
  },
  "reprompt": {
    "outputSpeech": {
      "type": "PlainText",
      "text": "Plain text string to speak",
      "playBehavior": "REPLACE_ENQUEUED"
    }
  },
  "directives": [
    {
      "type": "InterfaceName.Directive"
      (...properties depend on the directive type)
    }
  ],
  "shouldEndSession": true
}
*/

#[derive(Debug, Deserialize, Serialize)]
enum OutputSpeechType {
  SSML,
  PlainText,
  Unknown,
}

impl std::fmt::Display for OutputSpeechType {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let out = match self {
      Self::SSML => "SSML",
      Self::PlainText => "PlainText",
      Self::Unknown => "Unknown",
    };

    write!(f, "{}", out)
  }
}

// response implementation
#[derive(Debug, Deserialize, Serialize)]
pub struct OutputSpeech {
  #[serde(rename = "type")]
  speech_type: OutputSpeechType, // SSML or PlainText

  ssml: Option<String>,
  text: Option<String>,

  #[serde(rename = "playBehavior")]
  play_behavior: Option<String>,
}

impl OutputSpeech {
  pub fn new() -> Self {
    Self {
      speech_type: OutputSpeechType::Unknown,
      ssml: None,
      text: None,
      play_behavior: None,
    }
  }

  pub fn with_ssml(mut self, ssml: &str) -> Self {
    self.speech_type = OutputSpeechType::SSML;
    self.ssml = Some(ssml.into());
    self
  }

  pub fn with_text(mut self, text: &str) -> Self {
    self.speech_type = OutputSpeechType::PlainText;
    self.text = Some(text.into());
    self
  }
}

#[derive(Debug, Deserialize, Serialize)]
struct ResponseReprompt {
  #[serde(rename = "outputSpeech")]
  output_speech: OutputSpeech,
}

impl ResponseReprompt {
  fn new(o: OutputSpeech) -> Self {
    Self { output_speech: o }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AlexaResponse {
  #[serde(rename = "outputSpeech")]
  pub output_speech: OutputSpeech,

  reprompt: Option<ResponseReprompt>,

  #[serde(rename = "shouldEndSession")]
  pub should_end_session: bool,

  #[serde(rename = "sessionAttributes")]
  pub session_attributes: Option<()>,
}

impl Default for AlexaResponse {
  fn default() -> Self {
    Self {
      output_speech: OutputSpeech::new(),
      reprompt: None,
      should_end_session: false,
      session_attributes: None,
    }
  }
}

impl AlexaResponse {
  pub fn new_with_text(text: &str) -> Self {
    Self::default().with_output_speech(OutputSpeech::new().with_text(text))
  }

  pub fn new_with_ssml(ssml: &str) -> Self {
    Self::default().with_output_speech(OutputSpeech::new().with_ssml(ssml))
  }

  pub fn with_output_speech(mut self, o: OutputSpeech) -> Self {
    self.output_speech = o;
    self
  }

  pub fn with_reprompt_ssml(mut self, ssml: &str) -> Self {
    self.reprompt = Some(ResponseReprompt::new(OutputSpeech::new().with_ssml(ssml)));
    self
  }

  pub fn with_reprompt_text(mut self, text: &str) -> Self {
    self.reprompt = Some(ResponseReprompt::new(OutputSpeech::new().with_text(text)));
    self
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse {
  pub version: String,
  pub response: AlexaResponse,
}

impl Default for ApiResponse {
  fn default() -> Self {
    Self {
      version: "1.0".into(),
      response: AlexaResponse::default(),
    }
  }
}

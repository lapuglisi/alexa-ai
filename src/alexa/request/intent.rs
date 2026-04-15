use crate::alexa::request::GenericRequestImpl;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
pub enum IntentConfirmationStatus {
  Confirmed,
  Denied,
  None,
}

impl FromStr for IntentConfirmationStatus {
  type Err = Box<dyn std::error::Error>;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let c = match s {
      "CONFIRMED" => Self::Confirmed,
      "DENIED" => Self::Denied,
      "NONE" => Self::None,
      _ => {
        return Err(format!("unknown confirmation status: {}", s))?;
      }
    };

    Ok(c)
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IntentRequestSlotValue {
  resolutions: Option<serde_json::Value>,

  #[serde(rename = "type")]
  slot_type: String,

  value: Option<String>,

  values: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IntentRequestSlot {
  #[serde(rename = "confirmationStatus")]
  pub confirmation_status: Option<String>,

  pub name: Option<String>,

  pub resolutions: Option<serde_json::Value>,

  #[serde(rename = "slotValue")]
  pub slot_value: IntentRequestSlotValue,

  pub source: Option<String>,

  pub value: Option<String>,
}

impl IntentRequestSlot {
  pub fn from_value(value: Option<&serde_json::Value>) -> Option<Self> {
    if value.is_none() {
      return None;
    }

    let value = value.unwrap().to_owned();

    match serde_json::from_value::<IntentRequestSlot>(value) {
      Ok(v) => Some(v),
      Err(e) => {
        log::error!("could not get intent slot: {}", e);
        None
      }
    }
  }

  pub fn value(&self) -> Option<String> {
    let value = self.slot_value.value.clone();
    match value {
      Some(v) => Some(v),
      None => self.value.clone(),
    }
  }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IntentRequestIntent {
  pub name: String,

  #[serde(rename = "confirmationStatus")]
  pub confirmation_status: String,
  pub slots: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IntentRequest {
  #[serde(rename = "type")]
  pub intent_type: String,

  #[serde(rename = "requestId")]
  pub request_id: String,
  pub timestamp: String,

  #[serde(rename = "dialogState")]
  pub dialog_state: String,

  pub locale: String,
  pub intent: IntentRequestIntent,
}

impl GenericRequestImpl for IntentRequest {}

impl IntentRequest {
  pub fn get_slot(&self, slot: &str) -> Option<IntentRequestSlot> {
    let intent = self.intent.clone();

    let slots = match intent.slots {
      Some(s) => s,
      None => {
        return None;
      }
    };

    IntentRequestSlot::from_value(slots.get(slot))
  }
}

use crate::alexa::{
  request::{self, AlexaApiRequest, RequestType, intent::{self, IntentRequest, IntentRequestSlot}},
  response::{AlexaApiResponse, AlexaResponse},
};
use reqwest::{Client};
use std::{error::Error, time::Duration};
use std::str::FromStr;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct AiApiRequest {
	prompt: String,
	n_predict: u64,
	temperature: f64,
	model: String
}


pub async fn alexa_request(request: AlexaApiRequest) -> AlexaApiResponse {
  log::info!("got request: {:?}", request.request);

  let rt = match request.request.get("type") {
    Some(v) => {
      let s = v.as_str().unwrap_or("");
      RequestType::from_str(s).unwrap()
    }
    None => RequestType::Generic,
  };

  // TODO: implement each request type
  let response = match rt {
    RequestType::LaunchRequest => handle_launch_request(request),
    RequestType::SessionEndedRequest => AlexaApiResponse::default(),
		RequestType::IntentRequest => {
			handle_intent_request(request).await
				.unwrap_or(AlexaApiResponse::default())
		},
    _ => AlexaApiResponse::default(),
  };

  response
}

fn handle_launch_request(_: AlexaApiRequest) -> AlexaApiResponse {
	let ssml = "Bem vindo ao Sero Miners enterprises. O que você busca hoje?";

	let response = AlexaResponse::new_with_ssml(ssml).with_reprompt_ssml("Ss respostas válidas são: Busco algo ou busque algo.");
	
	AlexaApiResponse::default()
    .with_response(AlexaResponse::new_with_ssml(ssml).should_end_session(false))
}

async fn handle_intent_request(request: AlexaApiRequest) -> Result<AlexaApiResponse, Box<dyn Error>> {
	log::info!("handling IntentRequest: {}", serde_json::to_string_pretty(&request.request).unwrap_or("sem change de convert".into()));
	let mut response = AlexaResponse::default();

	let intent: intent::IntentRequest = match serde_json::from_value(request.request) {
		Ok(r) => r,
		Err(e) => {
			log::error!("could not get intent request: {}", e);
			return Err(e)?;
		}
	};

	response = match intent.intent.name.as_str() {
		"GetAICompletion" => {
			let s = hangle_get_ai_completion(intent).await
				.inspect_err(|e| log::error!("error handling intent: {}", e))
				.expect("could not handle intent request");

			log::info!("got response from rsrag: {}", s);

			
			response.with_text(&s).should_end_session(false)
		},
		_ => {
			response.with_text("quero fazer alugma coisa lokas")
		}
	};

	Ok(AlexaApiResponse::default().with_response(response))

}

async fn hangle_get_ai_completion(intent: IntentRequest) -> Result<String, Box<dyn Error>> {

	let a = intent.get_slot("user_query").expect("user_query not provided");
	let b = intent.get_slot("query_type").expect("query_type not providede");

	let query_type = b.value().expect("query_type must not be empty");
	let user_query = a.value().expect("user_query must not be empty");

	let query = AiApiRequest {
		model: "default".into(),
		prompt: user_query,
		n_predict: match query_type.as_str() {
			"simples" => 128,
			"normal" => 256,
			"expert" => 512,
			_ => 64
		},
		temperature: 0.2
	};
	
	let body = serde_json::to_string(&query)?;

	log::info!("sending query to rsrag: {}", body);

	let client = Client::new()
		.post("http://localhost:9091/api/completion")
		.body(body)
		.header("Content-Type", "application/json")
		.timeout(Duration::from_secs(5))
		.send()
		.await?;

	Ok(client.text().await?)
}

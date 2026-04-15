use crate::alexa::{
  request::{
    self, AlexaApiRequest, RequestType,
    intent::{self, IntentRequest, IntentRequestSlot},
  },
  response::{AlexaApiResponse, AlexaResponse},
};
use reqwest::Client;
use serde_json::json;
use std::str::FromStr;
use std::{error::Error, time::Duration};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct AiApiRequest {
  prompt: String,
  n_predict: u64,
  temperature: f64,
  model: String,
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
    RequestType::SessionEndedRequest => AlexaApiResponse::default().with_response(
      AlexaResponse::new_with_ssml("Aquele abraço então caro colega!").should_end_session(true),
    ),
    RequestType::IntentRequest => handle_intent_request(request)
      .await
      .unwrap_or(AlexaApiResponse::default()),
    _ => AlexaApiResponse::default().with_response(
      AlexaResponse::new_with_ssml(format!("quero comer bananas: {}", rt.to_string()).as_str())
        .should_end_session(true),
    ),
  };

  response
}

fn handle_launch_request(_: AlexaApiRequest) -> AlexaApiResponse {
  let ssml = "Bem vindo ao busco resposta chega hoje frete grátis enterprises. O que queres que eu responda?";

  let response = AlexaResponse::new_with_ssml(ssml)
    .with_reprompt_ssml("Para perguntar, diga: 'do doce'; 'quero saber'; 'perguntar' ou 'pergunta', e em seguida a sua pergunta enquanto dúvida a nível de questão.")
    .should_end_session(false);

  AlexaApiResponse::default().with_response(response)
}

async fn handle_intent_request(
  request: AlexaApiRequest,
) -> Result<AlexaApiResponse, Box<dyn Error>> {
  log::info!(
    "handling IntentRequest: {}",
    serde_json::to_string_pretty(&request.request).unwrap_or("sem change de convert".into())
  );
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
      let s = match hangle_get_ai_completion(intent).await {
        Ok(v) => {
          log::info!("got response from rsrag: {}", v);

          let json = serde_json::from_str::<serde_json::Value>(&v)?;
          let result = json
            .get("choices")
            .expect("no choices found")
            .get(0)
            .expect("no choices found")
            .get("message")
            .expect("no message found in response")
            .get("content")
            .expect("no content found in response")
            .to_string()
            .replace("\\\"", "'")
						.replace("\\n", "\n");

          result
        }
        Err(e) => {
          format!("{}", e)
        }
      };

      log::info!("got result from API response: {}", s);

      response.with_text(&s).should_end_session(false)
    }
    "AMAZON.StopIntent" => response
      .with_text(
        "Obrigado por utilizar os serviços da busco chega hoje facilities enterprises frete grátis!",
      )
      .should_end_session(true),
    _ => response
      .with_text("quero fazer alguma coisa chega hoje busco nearby")
      .should_end_session(true),
  };

  Ok(AlexaApiResponse::default().with_response(response))
}

async fn hangle_get_ai_completion(intent: IntentRequest) -> Result<String, Box<dyn Error>> {
  let a = intent
    .get_slot("user_query")
    .expect("não sei o que responder se não me perguntam nada.");

  let user_query = a
    .value()
    .expect("como que respondo uma pergunta que não existe?");

  let lower = user_query.to_lowercase();
  let re = regex::Regex::new("(ão|ao)+[!\\.]?$").unwrap();

  if re.is_match(&lower) {
    Err(format!("Então... meu pau na sua mão."))?
  }

  if regex::Regex::new("(cala|calar).*(boca)")
    .unwrap()
    .is_match(&lower)
  {
    Err("calar a boquinha já morreu, senta com força e pega no meu.")?
  }

  let json = json!({
    "model":  "default",
    "prompt": get_full_query(&user_query, &intent.locale),
    "n_predict": -1,
    "temperature": 0.1,
    "db_limit": 1,
    "threshold": 0.9,
    "rag_strategy": "main-colbert",
    "db_collection": "alexa-rsrag-text-main",
    "system_prompt": "You are a help assistant who provides concise and precise answers."
  });

  let body = serde_json::to_string(&json)?;

  log::info!("sending query to rsrag: {}", body);

  let client = Client::new()
    .post("http://localhost:9091/api/completion")
    .body(body)
    .header("Content-Type", "application/json")
    .send()
    .await
    .map_err(|_| format!("A inteligência artificial não quier responder sua pergunta agora."))?;

  Ok(client.text().await.map_err(|_| {
    format!(
      "A IA não souve responder a sua pergunta '{}'. Porque ela não é tão inteligente assim.",
      user_query
    )
  })?)
}

fn get_full_query(input: &str, locale: &str) -> String {
  let extra = match locale.to_lowercase().as_str() {
    "pt-br" => "(responda em portugues)",
    "en-us" => "(answer in english)",
    _ => "(answer in the original language)",
  };

  format!("{} {}", input, extra)
}

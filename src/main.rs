use actix_web::{App, HttpResponse, HttpServer, Responder, get, post};
use env_logger::{Builder, Env};
use serde::{Deserialize, Serialize};
use std::{
  env::{self, Args},
  error::Error,
  str::FromStr,
};

// Constants
const ALEXA_AI_DEFAULT_HOST: &str = "0.0.0.0";
const ALEXA_AI_DEFAULT_PORT: u16 = 9090;

#[derive(Debug, Deserialize, Serialize)]
struct AlexaRequest {
  version: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AlexaResponse {
  version: String,
  response: AlexaReponseData,
}

#[derive(Debug, Deserialize, Serialize)]
struct AlexaReponseData {
  #[serde(rename = "outputSpeech")]
  output_speech: AlexaResponseOutputSpeech,
  #[serde(rename = "shouldEndSession")]
  should_end_session: bool,
  #[serde(rename = "sessionAttributes")]
  session_attrs: Option<AlexaResponseSessionAttrs>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AlexaResponseOutputSpeech {
  #[serde(rename = "type")]
  reponse_type: String,
  ssml: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AlexaResponseSessionAttrs {
  index: String,
}

#[post("/")]
async fn alexa_main(payload: actix_web::web::Json<AlexaRequest>) -> HttpResponse {
  let resp = AlexaResponse {
    version: "1.0".into(),
    response: AlexaReponseData {
      output_speech: AlexaResponseOutputSpeech {
        reponse_type: "SSML".into(),
        ssml: "<speak>Quero comer bananas e peidar todo o dia.</speak>".into(),
      },
      should_end_session: false,
      session_attrs: None,
    },
  };

  log::info!("got payload: {:?}", payload.0);

  HttpResponse::Ok().json(resp)
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut http_host: String = String::from(ALEXA_AI_DEFAULT_HOST);
  let mut http_port: u16 = ALEXA_AI_DEFAULT_PORT;

  // initialize logger
  env_logger::init_from_env(Env::new().filter("ALEXA_AI_LOG").default_filter_or("info"));

  let mut iter = env::args();
  while let Some(arg) = iter.next() {
    match arg.as_ref() {
      "--host" | "-h" => {
        if let Some(h) = iter.next() {
          http_host = h;
        }
      }
      "--port" | "-p" => {
        if let Some(p) = iter.next() {
          http_port = p.parse::<u16>().unwrap_or(ALEXA_AI_DEFAULT_PORT);
        }
      }
      _ => {}
    }
  }

  let addr = format!("{}:{}", http_host, http_port);

  log::info!("http_host: {}", http_host);
  log::info!("http_port: {}", http_port);
  log::info!("address: {}", addr);

  let address: std::net::SocketAddr = std::net::SocketAddr::from_str(&addr)?;

  log::info!("using address: {}", address);

  let server = HttpServer::new(|| App::new().service(alexa_main))
    .workers(4)
    .bind(address)?;

  log::info!("listening on {}:{}", http_host, http_port);

  match server.run().await {
    Ok(_) => Ok(()),
    Err(e) => {
      log::error!("server error: {}", e);
      Err(e)?
    }
  }
}

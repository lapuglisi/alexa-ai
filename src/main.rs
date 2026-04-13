use actix_web::{App, HttpResponse, HttpServer, post};
use env_logger::Env;
use std::{env, error::Error, str::FromStr};

mod alexa;
use alexa::request;

// Constants
const ALEXA_AI_DEFAULT_HOST: &str = "0.0.0.0";
const ALEXA_AI_DEFAULT_PORT: u16 = 9090;
const ALEXA_AI_MAX_WORKERS: usize = 4;

#[post("/")]
async fn alexa_main(payload: String) -> HttpResponse {
  let request: request::AlexaApiRequest = match serde_json::from_str(&payload) {
    Ok(r) => r,
    Err(e) => {
      log::error!("error while parsing request: {}", e);
      return HttpResponse::InternalServerError().body("asdasd");
    }
  };

  let response = alexa::request::api::alexa_request(request).await;

  HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let mut http_host: String = String::from(ALEXA_AI_DEFAULT_HOST);
  let mut http_port: u16 = ALEXA_AI_DEFAULT_PORT;
  let mut workers: usize = ALEXA_AI_MAX_WORKERS;

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
      "--workers" => {
        if let Some(w) = iter.next() {
          workers = w.parse::<usize>().unwrap_or(ALEXA_AI_MAX_WORKERS);
        }
      }
      _ => {}
    }
  }

  let addr = format!("{}:{}", http_host, http_port);
  let address: std::net::SocketAddr = std::net::SocketAddr::from_str(&addr)?;

  log::info!("address ..... {}", address);
  log::info!("workers ..... {}", workers);

  let server = HttpServer::new(|| App::new().service(alexa_main))
    .workers(workers)
    .bind(address)?;

  println!("listening on {}:{}", http_host, http_port);
  log::info!("listening on {}:{}", http_host, http_port);

  match server.run().await {
    Ok(_) => Ok(()),
    Err(e) => {
      log::error!("server error: {}", e);
      Err(e)?
    }
  }
}

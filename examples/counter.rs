use actix_state_guards::UseStateGuardOnApp;

use std::fmt::Display;
use std::sync::Mutex;

use actix_web::App;
use actix_web::HttpServer;
use actix_web::Responder;
use actix_web::ResponseError;
use actix_web::{get, web};

#[derive(Debug)]
pub struct CounterError();

impl Display for CounterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Error: Counter is over 100")
    }
}

impl ResponseError for CounterError {}

#[get("/count")]
async fn count(counter: web::Data<Mutex<u32>>) -> impl Responder {
    let mut counter = counter.lock().unwrap();
    *counter += 1;
    counter.to_string()
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Mutex::new(0u32)))
            .use_state_guard(
                |counter: web::Data<Mutex<u32>>| async move {
                    if *counter.lock().unwrap() < 100 {
                        Ok(())
                    } else {
                        // by returning the error case of the result enum we signal that this
                        // request shall not be allowed to pass on to the scope wrapped
                        Err(CounterError())
                    }
                },
                web::scope("").service(count),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?)
}

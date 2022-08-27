use diesel_usage_controller::diesel_usage_controller;
use dotenv::dotenv;
use probability_of_fail_controller::probability_of_fail_controller;
use std::collections::HashMap;
use validate_env::validate_env;
use warp::Filter;

#[path = "./utils/validateEnv.rs"]
mod validate_env;

#[path = "./controllers/dieselUsageController.rs"]
mod diesel_usage_controller;

#[path = "./controllers/probabilityOfFailController.rs"]
mod probability_of_fail_controller;

// https://github.com/blurbyte/restful-rust
// fuel usage: l/100\*distance
// release with .exe
// table of contents
// https://blog.knoldus.com/containerize-rust-application-with-docker/

// TODO: make available on lan
// TODO: Docker

#[tokio::main]
async fn main() {
    // Load variables from .env file.
    dotenv().ok();

    // Check if variables from .env exists.
    let variables = validate_env();

    // GET: /calculateDieselUsageForDistance
    let calculate_diesel_usage_for_distance = warp::get()
        // Path
        .and(warp::path("calculateDieselUsageForDistance"))
        // Params
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::path::end())
        .and_then(diesel_usage_controller);

    // GET /probabilityOfUnitInjectorFail
    let probability_of_unit_injector_fail = warp::get()
        // Path
        .and(warp::path("probabilityOfUnitInjectorFail"))
        // Params
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::path::end())
        .and_then(probability_of_fail_controller);

    // Get all routes.
    let routes = calculate_diesel_usage_for_distance.or(probability_of_unit_injector_fail);

    println!("[server]: Listening on port {}.", variables["PORT"]);

    // TODO: Support for custom IP rather than hardcoded.
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

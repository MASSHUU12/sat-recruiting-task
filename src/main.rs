use dotenv::dotenv;
use routes::routes;
use std::collections::HashMap;
use validate_env::validate_env;

#[path = "./utils/validateEnv.rs"]
mod validate_env;

#[path = "./api/routes.rs"]
mod routes;

#[tokio::main]
async fn main() {
    // Load variables from .env file.
    dotenv().ok();

    // Check if variables from .env exists, and return them (or defaults) to variable.
    let variables: HashMap<String, String> = validate_env();

    // Create Vec holding splitted IP.
    let ip: Vec<u8> = Vec::from_iter(variables["IP"].split(".").map(|x| x.parse().unwrap()));

    println!(
        "[server]: Listening on: {}:{}.",
        variables["IP"], variables["PORT"]
    );

    // Run server.
    warp::serve(routes())
        .run((
            [ip[0], ip[1], ip[2], ip[3]],
            variables["PORT"].parse::<u16>().unwrap(),
        ))
        .await;
}

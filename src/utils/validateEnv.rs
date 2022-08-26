use std::env;

/**
 * Validates .env variables.
 *
 */
pub fn validate_env() -> String {
    match env::var("PORT") {
        Ok(var) => return var,
        Err(_) => {
            println!("PORT is not specified in .env file, using default port: 3030.");
            return "3030".to_string();
        }
    }
}

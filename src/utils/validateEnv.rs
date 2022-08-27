use std::{collections::HashMap, env};

/**
 * Validates .env variables.
 *
 * @returns - HaspMap<String, String>
 */
pub fn validate_env() -> HashMap<String, String> {
    let mut variables = HashMap::new();

    // Validate PORT variable and return its contents.
    let port = match env::var("PORT") {
        Ok(var) => {
            // Try to parse string into i32, or return 0.
            let num = var.parse::<i32>().unwrap_or_default();

            #[allow(unused_assignments)]
            let mut temp_var: String = String::new();

            // Checks whether PORT has been turned into an integer.
            if num == 0 {
                println!("Specified PORT is not a valid variable, using default: 3030.");
                temp_var = "3030".to_string();
            } else {
                temp_var = var;
            }
            temp_var
        }
        Err(_) => {
            println!("Variable PORT is not specified in .env file, using default: 3030.");
            "3030".to_string()
        }
    };

    // Validate IP variable and return its contents.
    let ip = match env::var("IP") {
        Ok(var) => var,
        Err(_) => {
            println!("Variable IP is not specified in .env file, using default: 127.0.0.1");
            "127.0.0.1".to_string()
        }
    };

    // Insert variables into HasMap.
    variables.insert("PORT".to_string(), port);
    variables.insert("IP".to_string(), ip);

    // Return variables.
    return variables;
}

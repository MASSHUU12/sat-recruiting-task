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
                println!("[server]: Specified PORT is not a valid variable, using default: 3030.");
                temp_var = "3030".to_string();
            } else {
                temp_var = var;
            }
            temp_var
        }
        Err(_) => {
            println!("[server]: Variable PORT is not specified in .env file, using default: 3030.");
            "3030".to_string()
        }
    };

    // Validate IP variable and return its contents.
    let ip = match env::var("IP") {
        Ok(var) => {
            let split = Vec::from_iter(var.split(".").map(String::from));
            let mut temp_var = var;

            for ip_fragment in split {
                if ip_fragment.parse::<u16>().unwrap_or(9999) == 9999 {
                    println!("[server]: The server can't run on the specified IP address, using default: 127.0.0.1");
                    temp_var = "127.0.0.1".to_string()
                }
            }
            temp_var
        }
        Err(_) => {
            println!(
                "[server]: Variable IP is not specified in .env file, using default: 127.0.0.1"
            );
            "127.0.0.1".to_string()
        }
    };

    // Insert variables into HasMap.
    variables.insert("PORT".to_string(), port);
    variables.insert("IP".to_string(), ip);

    // Return variables.
    return variables;
}

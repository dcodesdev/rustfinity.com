use challenges::{challenges_json, get_max_id};

fn main() {
    let arg = std::env::args().nth(1).unwrap_or("".to_string());

    match arg.as_str() {
        "count" => {
            let challenges = challenges_json().expect("Failed to read challenges.json");

            println!("Total challenges: {}", challenges.len());
        }
        _ => {
            let max_id = get_max_id();

            println!("Max id is: {}", max_id);
        }
    }
}

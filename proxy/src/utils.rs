use rand::{distributions::Alphanumeric, Rng};

// This converts a long string into one that's shortened.
// alongstringlikethis would become alongs...kethis
pub fn make_string_fixed_length(string: String, length: usize) -> String {
    if string.len() <= length {
        string
    } else {
        let part_size = length - 3 / 2;
        format!(
            "{}...{}",
            string[0..part_size].to_string(),
            string[string.len() - part_size..].to_string()
        )
    }
}

pub fn rainbowfy(message: String) -> String {
    let mut return_message = String::new();
    let rainbow_characters = "c6eab5";
    for (i, cha) in message.chars().enumerate() {
        match cha {
            ' ' => return_message.push(cha),
            _ => {
                return_message.push('&');
                return_message.push(
                    rainbow_characters
                        .chars()
                        .nth(i % rainbow_characters.len())
                        .unwrap(),
                );
                return_message.push(cha);
            }
        }
    }
    return_message
}

pub fn generate_connection_id() -> String {
    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    rand_string
}

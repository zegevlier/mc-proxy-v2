use rand::{distributions::Alphanumeric, Rng};

pub fn generate_connection_id() -> String {
    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    rand_string
}

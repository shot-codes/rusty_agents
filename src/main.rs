use rusty_agents::parse_level;
use std::io;

fn main() {
    eprintln!("Running rusty_agents 🤖");
    println!("rusty_agents"); // Send name to server
    let server_messages = io::BufReader::new(io::stdin());
    let state = parse_level(server_messages);
    dbg!(state);
}

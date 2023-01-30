use chat_formatting::chat::Chat;

fn main() {
    for line in std::io::stdin().lines() {
        if let Err(err) = line {
            eprintln!("Error parsing a line: {:?}", err);
            std::process::exit(1);
        }
        let line = line.unwrap();

        match serde_json::from_str::<Chat>(&line) {
            Ok(chat) => println!("{:?}", chat),
            Err(err) => eprintln!("Failed to parse: {err:?}"),
        }
    }
}

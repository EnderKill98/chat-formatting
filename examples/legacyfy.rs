use chat_formatting::{
    chat::{Chat, TextFormatter},
    translator::Translator,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let translator = if args.len() > 1 {
        Translator::from_translation_content(
            &std::fs::read_to_string(&args[1]).expect("Read file with translations"),
        )
        .expect("Load file with translations")
    } else {
        Default::default()
    };

    for line in std::io::stdin().lines() {
        if let Err(err) = line {
            eprintln!("Error parsing a line: {:?}", err);
            std::process::exit(1);
        }
        let line = line.unwrap();

        let chat = if let Ok(chat) = serde_json::from_str::<Chat>(&line) {
            chat
        } else {
            Chat::Legacy(line)
        };
        println!("{}", chat.to_legacy_string(&translator));
    }
}

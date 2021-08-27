extern crate omegalul;
use std::thread;

use ::std::*;
use omegalul::server::{get_random_server, ChatEvent, Server};

#[tokio::main]
async fn main() {
    if let Some(server_name) = get_random_server().await {
        println!("Connecting to {} server", server_name);

        let server = &mut Server::new(server_name.as_str(), vec!["hors".to_string()]);
        let chat = &mut server.start_chat().await;

        if let Some(chat) = chat {
            let cloned_chat = chat.clone();

            thread::spawn(move || {
                let cloned_chat = cloned_chat.clone();
                let runtime = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .unwrap();

                runtime.block_on(async {
                    loop {
                        cloned_chat.clone().send_message(&get_input()).await;
                    }
                });
            });

            loop {
                let event = chat.fetch_event().await;

                match event {
                    ChatEvent::Message(message) => println!("Incoming... \"{}\"", &message),
                    ChatEvent::Disconnected => println!("The user has disconnected... mean."),
                    ChatEvent::Typing => println!("The user is typing... how exciting!"),
                    _ => (),
                }
            }
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read line");

    return input.trim().to_string();
}
extern crate omegalul;
use std::{collections::HashMap, thread};

use ::std::*;
use omegalul::server::{get_random_server, ChatEvent, Server};

#[tokio::main]
async fn main() {
    if let Some(server_name) = get_random_server().await {
        println!("Connecting to {} server", server_name);

        let server = &mut Server::new(
            server_name.as_str(),
            vec!["hors".to_string()],
        );

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
                        let commands = &mut HashMap::<&str, Box<dyn Fn()>>::new();

                        commands.insert(
                            "disconnect",
                            Box::new(|| {
                                let cloned_chat = cloned_chat.clone();

                                runtime.spawn(async move {
                                    cloned_chat.clone().disconnect().await;

                                    println!("Disconnected, quitting program.");
                                    std::process::exit(0);
                                });
                            }),
                        );

                        commands.insert(
                            "info",
                            Box::new(|| println!("client id: {}", cloned_chat.client_id)),
                        );

                        let input = &get_input();
                        let command = commands.get(input.as_str());

                        match command {
                            Some(function) => (function)(),
                            None => cloned_chat.clone().send_message(input).await,
                        }
                    }
                });
            });

            loop {
                let event = chat.fetch_event().await;

                match event {
                    ChatEvent::Message(message) => println!("Incoming... \"{}\"", &message),
                    ChatEvent::StrangerDisconnected => {
                        println!("The user has disconnected... mean.")
                    }
                    ChatEvent::Typing => println!("The user is typing... how exciting!"),
                    ChatEvent::Connected => println!("You have matched with someone!"),
                    ChatEvent::CommonLikes(likes) => {
                        println!("Oh, you 2 seem to have some things in common! {:?}", likes)
                    }
                    ChatEvent::Waiting => {
                        println!("You are currently waiting for a person to match with.")
                    }
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

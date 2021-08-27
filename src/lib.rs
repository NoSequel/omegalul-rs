pub mod id;
pub mod server;

pub mod omegalul {}

#[cfg(test)]
mod tests {
    use crate::server::{get_random_server, ChatEvent, Server};

    #[tokio::test]
    async fn attempt_connect() {
        if let Some(server_name) = get_random_server().await {
            println!("Connecting to {} server", server_name);

            let server = &mut Server::new(server_name.as_str(), vec!["hors".to_string(), "discord".to_string()]);
            let chat = &mut server.start_chat().await;

            if let Some(chat) = chat {
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
}

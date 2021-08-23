pub mod id;
pub mod server;

pub mod omegalul {}

#[cfg(test)]
mod tests {

    use crate::server::{get_random_server, Server};
    use reqwest::Client;

    #[tokio::test]
    async fn attempt_connect() {
        let client = Client::new();

        if let Some(server_name) = get_random_server(client).await {
            println!("Connecting to {} server", server_name);

            let server = &mut Server::new(server_name.as_str());
            let chat = &mut server.start_chat().await;

            if let Some(chat) = chat {
                loop {
                    chat.fetch_event().await;
                }
            }
        }
    }
}

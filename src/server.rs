use crate::id::*;
use reqwest::Client;

#[derive(Clone)]
pub struct Server {
    name: String,
    client: Client,
}

impl Server {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            client: Client::new(),
        }
    }

    pub async fn start_chat(&mut self) -> Option<Chat> {
        let random_id = generate_random_id();
        let omegle_url = format!("{}.omegle.com", self.name);

        let response = self
            .client
            .post(format!(
                "https://{}/start?rcs=1&firstevents=1&m=0&randid={}",
                omegle_url, random_id
            ))
            .send()
            .await;

        if let Ok(response) = response {
            let json_response = json::parse(&response.text().await.unwrap());

            if let Ok(json) = json_response {
                return Some(Chat::new(
                    json["clientID"].clone().as_str().unwrap(),
                    self.clone(),
                ));
            }
        }

        return None;
    }
}

pub struct Chat {
    client_id: String,
    server: Server,
}

impl Chat {
    pub fn new(client_id: &str, server: Server) -> Self {
        return Self {
            client_id: client_id.to_string(),
            server: server,
        };
    }

    pub async fn fetch_event(&mut self) -> ChatEvent {
        let omegle_url = format!("{}.omegle.com", self.server.name);

        let response = self
            .server
            .client
            .post(format!("https://{}/events", omegle_url))
            .body(format!("id={}", self.client_id.clone()))
            .send()
            .await;

        if let Ok(response) = response {
            println!("{:?}", response);

            //if let Ok(json) = json::parse(response.text().await.unwrap().as_str()) {
            //    println!("{}", json)
            //}
        }

        return ChatEvent::None;
    }
}

pub enum ChatEvent {
    Message(String),
    Disconnected,
    Typing,
    None,
}

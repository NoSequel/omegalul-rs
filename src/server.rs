use crate::id::*;
use json::JsonValue;
use reqwest::Client;

use rand::seq::SliceRandom;

pub async fn get_random_server(client: Client) -> Option<String> {
    let servers = get_servers(client).await;

    if let Some(servers) = servers {
        match servers {
            JsonValue::Array(array) => {
                let random = array.choose(&mut rand::thread_rng());

                if let Some(random) = random {
                    return Some(random.as_str().unwrap().to_string());
                }
            }
            _ => {}
        }
    }

    return None;
}

pub async fn get_servers(client: Client) -> Option<JsonValue> {
    let request = client.get("https://omegle.com/status").send().await;

    if let Ok(request) = request {
        return Some(
            json::parse(request.text().await.unwrap().as_str()).unwrap()["servers"].clone(),
        );
    }

    return None;
}

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
            .post(format!(
                "https://{}/events?id={}",
                omegle_url, self.client_id
            ))
            .send()
            .await;

        if let Ok(response) = response {
            if let Ok(body) = response.text().await {
                if let Ok(json) = json::parse(body.as_str()) {
                    println!("{}", json)
                }
            }
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

use crate::id::*;
use itertools::Itertools;
use json::JsonValue;
use reqwest::Client;

use rand::seq::SliceRandom;

pub async fn get_random_server() -> Option<String> {
    let servers = get_servers().await;

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

pub async fn get_servers() -> Option<JsonValue> {
    let client = Client::new();
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
    interests: Vec<String>,
    client: Client,
}

impl Server {
    pub fn new(name: &str, interests: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            interests: interests,
            client: Client::new(),
        }
    }

    pub async fn start_chat(&mut self) -> Option<Chat> {
        let random_id = generate_random_id();
        let omegle_url = format!("{}.omegle.com", self.name);
        let interests = self.interests.iter().cloned().intersperse(",".to_owned()).collect::<String>();

        println!("{}", interests);

        let response = self
            .client
            .post(format!(
                "https://{}/start?caps=recaptcha2,t&firstevents=1&spid=&randid={}&lang=en&topics=[{}]",
                omegle_url, random_id, interests
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
        let server = &mut self.server;
        let omegle_url = format!("{}.omegle.com", server.name);
        let pair = [("id", self.client_id.clone())];

        let response = server
            .client
            .post(format!("https://{}/events", omegle_url))
            .form(&pair)
            .send()
            .await;

        if let Ok(response) = response {
            let json: Result<Vec<Vec<String>>, _> = response.json().await;

            return match &json {
                Ok(result) => {
                    let message = &result[0];
                    let event = &message[0];

                    return match event.as_str() {
                        "gotMessage" => ChatEvent::Message(message[1..message.len()].concat()),
                        "typing" => ChatEvent::Typing,
                        "strangerDisconnected" => ChatEvent::Disconnected,
                        _ => ChatEvent::None,
                    };
                }
                Err(_err) => ChatEvent::None,
            };
        }

        return ChatEvent::None;
    }

    pub async fn send_message(&mut self, message: &str) {
        let server = &mut self.server;
        let omegle_url = format!("{}.omegle.com", server.name);

        let pair = [("id", self.client_id.clone()), ("msg", message.to_owned())];

        let response = server
            .client
            .post(format!("https://{}/send", omegle_url))
            .form(&pair)
            .send()
            .await;

        match response {
            Err(error) => println!("{:?}", error),
            _ => (),
        }
    }
}

#[derive(Debug)]
pub enum ChatEvent {
    Message(String),
    Disconnected,
    Typing,
    None,
}

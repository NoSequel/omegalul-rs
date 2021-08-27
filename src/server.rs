use crate::id::*;
use json::JsonValue;
use reqwest::Client;

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

pub async fn get_random_server() -> Option<String> {
    let servers = get_servers().await;

    if let Some(servers) = servers {
        if let JsonValue::Array(array) = servers {
            return match array.choose(&mut rand::thread_rng()) {
                Some(random) => Some(random.as_str().unwrap().to_string()),
                None => None,
            };
        }
    }

    return None;
}

pub async fn get_servers() -> Option<JsonValue> {
    let client = Client::new();
    let request = client.get("https://omegle.com/status").send().await;

    return match request {
        Ok(request) => {
            Some(json::parse(request.text().await.unwrap().as_str()).unwrap()["servers"].clone())
        }
        Err(_error) => None,
    };
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

        let mut interests_str = "".to_owned();

        for i in 0..self.interests.len() {
            interests_str.push_str(&format!("\"{}\"", self.interests[i]));

            if i != self.interests.len() - 1 {
                interests_str.push(',');
            }
        }

        let response = self
            .client
            .post(format!(
                "https://{}/start?caps=recaptcha2,t&firstevents=1&spid=&randid={}&lang=en&topics=[{}]",
                omegle_url, random_id, interests_str
            ))
            .send()
            .await;

        if let Ok(response) = response {
            return match json::parse(&response.text().await.unwrap()) {
                Ok(json) => Some(Chat::new(
                    json["clientID"].clone().as_str().unwrap(),
                    self.clone(),
                )),
                Err(_error) => None,
            };
        }

        return None;
    }
}

#[derive(Clone)]
pub struct Chat {
    pub client_id: String,
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
            let response_text = response.text().await;

            if let Ok(response_text) = response_text {
                let json_response = json::parse(&response_text);

                return match json_response {
                    Ok(json_response) => {
                        let response_array = as_array(&json_response);

                        for event in response_array {
                            let array = as_array(&event);
                            let event_name = event[0].as_str().unwrap().to_owned();

                            return match event_name.as_str() {
                                "gotMessage" => ChatEvent::Message(array[1].as_str().unwrap().to_owned()),
                                "connected" => ChatEvent::Connected,
                                "commonLikes" => ChatEvent::CommonLikes(as_array(&array[1]).iter().map(|x| x.as_str().unwrap().to_owned()).collect()),
                                "waiting" => ChatEvent::Waiting,
                                "typing" => ChatEvent::Typing,
                                "stoppedTyping" => ChatEvent::StoppedTyping,
                                "strangerDisconnected" => ChatEvent::StrangerDisconnected,
                                _ => ChatEvent::None
                            };
                        }
                        
                        return ChatEvent::None;
                    },
                    Err(_err) => ChatEvent::None
                };
            }
        }

        return ChatEvent::None;
    }

    pub async fn send_message(self, message: &str) {
        let server = self.server;
        let omegle_url = format!("{}.omegle.com", server.name);

        let pair = [("id", self.client_id.clone()), ("msg", message.to_owned())];

        handle_simple_post::<&str, String>(
            server.client.clone(),
            &format!("https://{}/send", omegle_url),
            &pair,
        )
        .await;
    }

    pub async fn disconnect(&mut self) {
        let server = &mut self.server;
        let omegle_url = format!("{}.omegle.com", server.name);

        let pair = [("id", self.client_id.clone())];

        handle_simple_post::<&str, String>(
            server.client.clone(),
            &format!("https://{}/disconnect", omegle_url),
            &pair,
        )
        .await;
    }
}

async fn handle_simple_post<K: Serialize, V: Serialize>(
    client: Client,
    url: &str,
    pair: &[(K, V)],
) {
    let response = client.post(format!("{}", url)).form(&pair).send().await;

    if let Err(error) = response {
        println!("{:?}", error);
    }
}

fn as_array(value: &JsonValue) -> Vec<JsonValue> {
    match value {
        JsonValue::Array(array) => array.to_vec(),
        _ => vec![],
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChatEvent {
    #[serde(rename(deserialize = "gotMessage"))]
    Message(String),
    #[serde(rename(deserialize = "commonLikes"))]
    CommonLikes(Vec<String>),
    #[serde(rename(deserialize = "connected"))]
    Connected,
    #[serde(rename(deserialize = "strangerDisconnected"))]
    StrangerDisconnected,
    #[serde(rename(deserialize = "typng"))]
    Typing,
    #[serde(rename(deserialize = "stoppedTyping"))]
    StoppedTyping,
    #[serde(rename(deserialize = "waiting"))]
    Waiting,
    None,
}

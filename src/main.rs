pub mod id;
mod server;

use futures::executor;
use json::JsonValue;
use reqwest::Client;

use server::*;

use rand::seq::SliceRandom;

#[tokio::main]
async fn main() {
    let client = Client::new();

    if let Some(server_name) = get_random_server(client) {
        let server = &mut Server::new(server_name.as_str());
        let chat = &mut executor::block_on(server.start_chat());

        if let Some(chat) = chat {
            executor::block_on(chat.fetch_event());
        }
    }
}

fn get_random_server(client: Client) -> Option<String> {
    let servers = executor::block_on(get_servers(client));

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

async fn get_servers(client: Client) -> Option<JsonValue> {
    let request = client.get("https://omegle.com/status").send().await;

    if let Ok(request) = request {
        return Some(
            json::parse(request.text().await.unwrap().as_str()).unwrap()["servers"].clone(),
        );
    }

    return None;
}

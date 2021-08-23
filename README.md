# omegalul-rs
``omegalul-rs`` is a work-in-progress opensource library for building [Omegle](https://omegle.com) clients. 

# Features
### Current Features
* Fetching random server from omegle status servers
* Starting a chat on the server
* Fetching current chat event (currently not working properly)

### Missing Features (adding soon)
* Sending chat messages
* Adding interests

# Usages
```rs
    #[tokio::main]
    async fn main() {
        // get a random server to connect with, this server
        // is grabbed from the https://omegle.com/status path.
        if let Some(server_name) = get_random_server().await {
            println!("Connecting to {} server", server_name);

            // just create a new server - this is a simple struct and 
            // the Server::new(&str) method does nothing besides creating
            // a new object of the struct.
            let server = &mut Server::new(server_name.as_str());

            // start the chat, this sends a POST message  
            // and connects to the current omegle server
            let chat = &mut server.start_chat().await;

            if let Some(chat) = chat {
                // indefinite loop to always get new events
                // from the current omegle chat.
                loop {
                    // fetch the current event, this may return 
                    // a ChatEvent::None value - this means
                    // there is no current event going on.
                    let event = chat.fetch_event().await;

                    match event {
                        ChatEvent::Message(message) => println!("{}", &message),
                        ChatEvent::Disconnected => println!("The user has disconnected... mean."),
                        ChatEvent::Typing => println!("The user is typing... how exciting!"),
                        _ => ()
                    }
                }
            }
        }
    }
```
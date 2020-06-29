use log::info;

use std::format;
use std::time::Duration;

use naia_client::{find_my_ip_address, ClientEvent, Config, NaiaClient};

use naia_example_shared::{manifest_load, AuthEvent, ExampleEntity, ExampleEvent, StringEvent};

const SERVER_PORT: &str = "14191";

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        const SERVER_IP_ADDRESS: &str = "192.168.1.9"; // Put your Server's IP Address here!, can't easily find this automatically from the browser
    } else {
        const SERVER_IP_ADDRESS: &str = find_my_ip_address::get();
    }
}

pub struct App {
    client: NaiaClient<ExampleEvent, ExampleEntity>,
}

impl App {
    pub fn new() -> App {
        info!("Naia Client Example Started");

        let server_socket_address = format!("{}:{}", SERVER_IP_ADDRESS, SERVER_PORT);

        let mut config = Config::default();
        config.heartbeat_interval = Duration::from_secs(4);

        let auth = ExampleEvent::AuthEvent(AuthEvent::new("charlie", "12345"));

        App {
            client: NaiaClient::new(
                &server_socket_address,
                manifest_load(),
                Some(config),
                Some(auth),
            ),
        }
    }

    pub fn update(&mut self) {
        match self.client.receive() {
            Ok(event) => {
                match event {
                    ClientEvent::Connection => {
                        info!("Client connected to: {}", self.client.server_address());
                    }
                    ClientEvent::Disconnection => {
                        info!("Client disconnected from: {}", self.client.server_address());
                    }
                    ClientEvent::Event(event_type) => match event_type {
                        ExampleEvent::StringEvent(string_event) => {
                            let message = string_event.message.get();
                            info!("Client received event: {}", message);

                            if let Some(count) = self.client.get_sequence_number() {
                                let new_message: String = "Client Packet (".to_string()
                                    + count.to_string().as_str()
                                    + ")";
                                info!("Client send: {}", new_message);

                                let string_event = StringEvent::new(new_message);
                                self.client.send_event(&string_event);
                            }
                        }
                        _ => {}
                    },
                    ClientEvent::CreateEntity(local_key) => {
                        if let Some(entity) = self.client.get_entity(local_key) {
                            match entity {
                                ExampleEntity::PointEntity(point_entity) => {
                                    info!(
                                        "creation of point entity with key: {}, x: {}, y: {}",
                                        local_key,
                                        point_entity.as_ref().borrow().x.get(),
                                        point_entity.as_ref().borrow().y.get()
                                    );
                                }
                            }
                        }
                    }
                    ClientEvent::UpdateEntity(local_key) => {
                        if let Some(entity) = self.client.get_entity(local_key) {
                            match entity {
                                ExampleEntity::PointEntity(point_entity) => {
                                    info!(
                                        "update of point entity with key: {}, x: {}, y: {}",
                                        local_key,
                                        point_entity.as_ref().borrow().x.get(),
                                        point_entity.as_ref().borrow().y.get()
                                    );
                                }
                            }
                        }
                    }
                    ClientEvent::DeleteEntity(local_key) => {
                        info!("deletion of point entity with key: {}", local_key);
                    }
                    ClientEvent::None => {
                        //info!("Client non-event");
                    }
                }
            }
            Err(err) => {
                info!("Client Error: {}", err);
            }
        }
    }
}
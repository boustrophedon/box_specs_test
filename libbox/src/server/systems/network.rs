use std::io::{self, Read, Write};

use std::net::{TcpListener, TcpStream};

use std::num::Wrapping;

use rustc_serialize::json;

use specs::{MessageQueue, RunArg, System, World};

use server::{ServerConfig, ServerSystemContext};

use common::{Message, NetworkMessage};

struct ClientConnection {
    pub stream: TcpStream,
    pub client_id: u16,
}

impl ClientConnection {
    pub fn new(stream: TcpStream, client_id: u16) -> ClientConnection {
        stream.set_nodelay(true).unwrap();
        stream.set_nonblocking(true).unwrap();
        ClientConnection {
            stream: stream,
            client_id: client_id,
        }
    }
}

pub struct NetworkSystem {
    connected_clients: Vec<ClientConnection>, // hashmap may be better
    listener: TcpListener,
    current_id: Wrapping<u16>,
}

impl NetworkSystem {
    pub fn new(cfg: ServerConfig) -> NetworkSystem {
        let listener = TcpListener::bind(cfg.server_address).unwrap();
        listener.set_nonblocking(true).unwrap();
        NetworkSystem {
            connected_clients: Vec::new(),
            listener: listener,
            current_id: Wrapping(0u16),
        }
    }

    fn handle_new_connection(&mut self, stream: TcpStream) {
        // TODO do a search for unused client id and err if we don't have one
        // but actually we should just reject new connections if we are full
        self.connected_clients.push(ClientConnection::new(stream, self.current_id.0));
        self.current_id += Wrapping(1);
    }

    fn handle_incoming_connections(&mut self) {
        loop {
            let stream = self.listener.accept();
            match stream {
                Ok(s) => {
                    self.handle_new_connection(s.0);
                }
                Err(_) => {
                    break;
                    // don't know what to do here yet.
                }
            }
        }
    }

    fn handle_incoming_messages(&mut self) {
        let mut buf = String::new();
        for client in &mut self.connected_clients {
            match client.stream.read_to_string(&mut buf) {
                Ok(_) => {},
                Err(error) => {
                    // apparently with nonblocking sockets, it returns WouldBlock when the read is
                    // successful?
                    if error.kind() == io::ErrorKind::WouldBlock ||
                       error.kind() == io::ErrorKind::TimedOut {}
                    else {
                        // actual error
                        println!("{:?}", error);
                        continue;
                    }
                },
            }

            if buf.len() < 1 {
                continue;
            }

            let msg: NetworkMessage;
            match json::decode(&buf) {
                Ok(m) => {msg = m},
                Err(error) => {
                    println!("error decoding message: {:?}", error);
                    println!("{}", buf);
                    continue;
                }
            }

            // TODO lots of validation here, and attach client id to messages somehow
            use common::NetworkMessage::*;
            match msg {
                GameMessage(_) => (),
                Connect(version) => {
                    if version.0.as_str() != env!("CARGO_PKG_VERSION") {
                        println!("client {} sent wrong version string", client.client_id);
                        // TODO send disconnect
                    }
                    else {
                        println!("sending motd to client {}", client.client_id);
                        let message = NetworkMessage::Motd("drink your ovaltine".to_owned());
                        let message = json::encode(&message).unwrap();
                        client.stream.write(message.as_bytes()).unwrap();
                    }
                },
                Motd(_) => (),
                Disconnect(_) => {
                    // close connection
                }
            }

            buf.clear();

        }
    }
}

impl System<Message, ServerSystemContext> for NetworkSystem {
    fn run(&mut self, arg: RunArg, _: MessageQueue<Message>, _: ServerSystemContext) {
        let _ = arg.fetch(|_| {});

        self.handle_incoming_connections();
        self.handle_incoming_messages();
    }

    fn handle_message(&mut self, _: &mut World, msg: &Message) {
        match *msg {
            _ => (),
        }
    }
}

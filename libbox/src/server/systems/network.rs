use std::io::{self, Read, Write};

use std::net::{TcpListener, TcpStream};

use std::num::Wrapping;

use nalgebra::Point3;

use specs::{MessageQueue, RunArg, System, World};

use server::{ServerConfig, ServerSystemContext};

use common::{decode_messages, Message, NetworkMessage};
use common::ClientID;

struct ClientConnection {
    pub stream: TcpStream,
    pub client_id: ClientID,
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
    current_id: Wrapping<ClientID>,
}

impl NetworkSystem {
    pub fn new(cfg: ServerConfig) -> NetworkSystem {
        let listener = TcpListener::bind(cfg.server_address).unwrap();
        listener.set_nonblocking(true).unwrap();
        NetworkSystem {
            connected_clients: Vec::new(),
            listener: listener,
            current_id: Wrapping(0),
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

    fn handle_incoming_messages(&mut self, mq: MessageQueue<Message>) {
        let mut messages_buf = String::new();
        let mut send_buf = Vec::new();

        for client in &mut self.connected_clients {
            match client.stream.read_to_string(&mut messages_buf) {
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

            let messages_result = decode_messages(&messages_buf); 

            let messages: Vec<NetworkMessage>;
            match messages_result {
                Ok(m) => { messages = m },
                Err(error) => {
                    println!("error decoding message from client {}: {:?}", client.client_id, error);
                    println!("{}", &messages_buf);
                    continue;
                }
            }

            for msg in messages {
                handle_message(msg, client, &mq, &mut send_buf);
            }

            messages_buf.clear();
        }
    }

    fn send_game_message_to_clients(&mut self, msg: &Message) {
        let netmsg = NetworkMessage::GameMessage(msg.clone());
        let mut buf = Vec::new();
        netmsg.encode_as_bytes(&mut buf);
        for client in &mut self.connected_clients {
            client.stream.write(buf.as_slice()).unwrap();
        }
    }

}

fn handle_message(msg: NetworkMessage, client: &mut ClientConnection, mq: &MessageQueue<Message>, send_buf: &mut Vec<u8>) {
    // TODO lots of validation here
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

                let netmsg = NetworkMessage::Connected(client.client_id, "drink your ovaltine".to_owned());
                netmsg.encode_as_bytes(send_buf);
                client.stream.write(send_buf.as_slice()).unwrap();
                send_buf.clear();

                mq.send(Message::SpawnBox(
                         Point3::new((7*client.client_id%11) as f32, (4*client.client_id%23) as f32, 0.0), client.client_id
                        )
                );
            }
        },
        Connected(_, _) => (),
        Disconnect(_) => {
            // close connection
        }
    }
}

impl System<Message, ServerSystemContext> for NetworkSystem {
    fn run(&mut self, arg: RunArg, mq: MessageQueue<Message>, _: ServerSystemContext) {
        let _ = arg.fetch(|_| {});

        self.handle_incoming_connections();
        self.handle_incoming_messages(mq);
    }

    fn handle_message(&mut self, _: &mut World, msg: &Message) {
        // TODO write code that checks message variant and determine whether to send to all clients
        // or specific ones
        // for now we send to all
        self.send_game_message_to_clients(msg);
    }
}

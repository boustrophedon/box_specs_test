use std::io::{self, Read, Write};

use std::net::TcpStream;

use rustc_serialize::json;

use specs::{MessageQueue, RunArg, System, World};

use client::{ClientConfig, ClientSystemContext};

use common::{Message, NetworkMessage, Version};

#[derive(Copy, Clone, Debug)]
enum ConnectionState {
    Connected,
    Connecting,
    Disconnected,
}

struct ServerConnection {
    pub stream: Option<TcpStream>,
    pub connection_state: ConnectionState,
}

impl ServerConnection {
    pub fn new(connect: io::Result<TcpStream>) -> ServerConnection {
        let stream: Option<TcpStream>;
        let state: ConnectionState;
        match connect {
            Ok(s) => {
                s.set_nodelay(true).unwrap();
                s.set_nonblocking(true).unwrap();
                stream = Some(s); 
                state = ConnectionState::Connecting;
            },
            Err(error) => {
                println!("Connecting to server failed, {:?}", error);
                stream = None;
                state = ConnectionState::Disconnected;
            }
        }

        ServerConnection {
            stream: stream,
            connection_state: state,
        }
    }
}

pub struct NetworkSystem {
    current_server: ServerConnection,
}

impl NetworkSystem {
    pub fn new(cfg: ClientConfig) -> NetworkSystem {
        let connection = TcpStream::connect(cfg.server_address);

        let server_connection = ServerConnection::new(connection); 

        let mut sys = NetworkSystem {
            current_server: server_connection,
        };
        let version = env!("CARGO_PKG_VERSION").to_owned();
        sys.send_connect(version);
        sys
    }

    pub fn send_connect(&mut self, version: String) {
        // TODO: don't call unwrap, actually handle connection errors, close, etc
        let stream = self.current_server.stream.as_mut().unwrap();

        let connect = NetworkMessage::Connect(Version(version));
        let connect_encoded = json::encode(&connect).unwrap();

        stream.write(connect_encoded.as_bytes()).unwrap();
    }

    pub fn handle_server_message(&mut self, msgq: MessageQueue<Message>, connecting: bool) {
        // TODO: don't call unwrap, actually handle connection errors, close, etc
        let stream = self.current_server.stream.as_mut().unwrap();

        let mut buf = String::new();
        match stream.read_to_string(&mut buf) {
            Ok(_) => (),
            Err(error) => {
                // apparently with nonblocking sockets, it returns WouldBlock when the read is
                // successful?
                if error.kind() == io::ErrorKind::WouldBlock ||
                   error.kind() == io::ErrorKind::TimedOut {}
                else {
                    // actual error
                    println!("{:?}", error);
                    return;
                }
            },
        }

        if buf.len() < 1 {
            return;
        }

        let msg: NetworkMessage;
        match json::decode(&buf) {
            Ok(m) => {msg = m},
            Err(error) => {
                println!("error decoding message: {:?}", error);
                println!("{}", buf);
                return;
            }
        }

        use common::NetworkMessage::*;
        match msg {
            GameMessage(message) => {
                // ignore messages while we're still connecting
                if connecting {
                    return;
                }
                msgq.send(message);
            },
            Connect(_) => (), // only used by server
            Motd(motd) => {
                println!("Connected to server");
                println!("Message of the day: {}", motd); 
                self.current_server.connection_state = ConnectionState::Connected;
            },
            Disconnect(_) => {
                // close connection
            }
        }
    }
}

impl System<Message, ClientSystemContext> for NetworkSystem {
    fn run(&mut self, arg: RunArg, msg: MessageQueue<Message>, _: ClientSystemContext) {
        let _ = arg.fetch(|_| {});

        let state = self.current_server.connection_state;
        match state {
            ConnectionState::Connected => {
                self.handle_server_message(msg, false);
            },
            ConnectionState::Connecting => {
                self.handle_server_message(msg, true);
            },
            ConnectionState::Disconnected => (),
        }
    }

    fn handle_message(&mut self, _: &mut World, msg: &Message) {
        match *msg {
            _ => (),
        }
    }
}

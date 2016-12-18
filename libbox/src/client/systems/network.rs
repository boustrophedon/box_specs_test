use std::io::{self, Read, Write};

use std::net::TcpStream;

use specs::{MessageQueue, RunArg, System, World};

use client::{ClientConfig, ClientSystemContext};

use common::{decode_messages, Message, NetworkMessage, Version};
use common::resources::MyClientId;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
        let mut buf = Vec::new();
        connect.encode_as_bytes(&mut buf);
        stream.write(buf.as_slice()).unwrap();
    }

    pub fn handle_server_message(&mut self, mq: MessageQueue<Message>, my_id: &mut MyClientId) {
        let mut messages_buf = String::new();
        // need scope so stream borrow doesn't conflict with handle_message borrow
        {
            // TODO: don't call unwrap, actually handle connection errors, close, etc
            let stream = self.current_server.stream.as_mut().unwrap();

            match stream.read_to_string(&mut messages_buf) {
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
        }

		let messages_result = decode_messages(&messages_buf);

		let messages: Vec<NetworkMessage>;
		match messages_result {
			Ok(m) => { messages = m },
			Err(error) => {
				println!("error decoding message from server: {:?}", error);
				println!("{}", &messages_buf);
				return;
			}
		}

		for msg in messages {
			self.handle_message(msg, my_id, &mq);
		}

    }

    fn handle_message(&mut self, msg: NetworkMessage, my_id: &mut MyClientId, mq: &MessageQueue<Message>) {
        use common::NetworkMessage::*;
        match msg {
            GameMessage(message) => {
                // ignore messages while we're still connecting
                if self.current_server.connection_state == ConnectionState::Connecting {
                    return;
                }
                mq.send(message);
            },
            Connect(_) => (), // only used by server
            Connected(clientid, motd) => {
                println!("Connected to server");
                println!("client id: {}", clientid);
                println!("Message of the day: {}", motd); 
                *my_id = MyClientId::new(clientid);
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
        let mut my_id = arg.fetch(|w| {w.write_resource::<MyClientId>()});

        let state = self.current_server.connection_state;
        match state {
            ConnectionState::Connected => {
                self.handle_server_message(msg, &mut *my_id);
            },
            ConnectionState::Connecting => {
                self.handle_server_message(msg, &mut *my_id);
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

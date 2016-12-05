use std::net::{TcpListener, TcpStream};

use std::num::Wrapping;

use specs::{MessageQueue, RunArg, System, World};

use server::{ServerConfig, ServerSystemContext};

use common::Message;

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

    //fn handle_connection(&mut self, stream: TcpStream) {
    //    // TODO do a search for unused client id and err if we don't have one
    //    // but actually we should just reject new connections if we are full
    //}

    fn handle_incoming_connections(&mut self) {
        loop {
            let stream = self.listener.accept();
            match stream {
                Ok(s) => {
                    self.connected_clients.push(ClientConnection::new(s.0, self.current_id.0));
                    self.current_id += Wrapping(1);
                    //self.handle_connection(s);
                }
                Err(_) => {
                    break;
                    // don't know what to do here yet.
                }
            }
        }
    }

    fn do_echo(&mut self) {
        let mut buf = String::new();
        for client in &mut self.connected_clients {
            use std::io::{Read, Write};
            buf.clear();
            client.stream.read_to_string(&mut buf);
            if buf.len()>0 {println!("{}", buf);}
            write!(client.stream, "{}", buf).unwrap();
        }
    }

}

impl System<Message, ServerSystemContext> for NetworkSystem {
    fn run(&mut self, arg: RunArg, _: MessageQueue<Message>, _: ServerSystemContext) {
        let _ = arg.fetch(|_| {});

        self.handle_incoming_connections();
        self.do_echo();
    }

    fn handle_message(&mut self, _: &mut World, msg: &Message) {
        match *msg {
            _ => (),
        }
    }
}

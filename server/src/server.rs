use std::{
    io::{self, ErrorKind},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

use shared::Codec;

pub struct Server {
    pub listener: TcpListener,
    pub clients: Vec<Codec>,
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

//const MSG_SIZE: usize = 64;

impl Server {
    pub fn initialize(address: &str) -> Result<Server, io::Error> {
        let listener = TcpListener::bind(address)?;
        listener.set_nonblocking(true)?;

        let clients = vec![];
        let (tx, rx) = mpsc::channel::<String>();

        Ok(Server {
            listener,
            clients,
            tx,
            rx,
        })
    }

    pub fn accept_new_connection(&mut self) {
        if let Ok((socket, addr)) = self.listener.accept() {
            println!("Client {} connected!", addr);
            self.handle_connection(socket, addr);
        }
    }

    fn handle_connection(&mut self, socket: TcpStream, addr: SocketAddr) {
        let tx = self.tx.clone();
        self.clients.push(
            Codec::new(socket.try_clone().expect("failed to clone client."))
                .expect("failed creating codec."),
        );

        // reading messages and sending them via tx in another thread
        let _th = thread::spawn(move || loop {
            //let mut buff = vec![0; MSG_SIZE];
            let mut codec_copy = Codec::new(socket.try_clone().expect("failed to clone client."))
                .expect("failed creating codec copy.");
            //match socket.read_exact(&mut buff) {
            match codec_copy.read_message() {
                Ok(msg) => {
                    //let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                    //let msg = String::from_utf8(msg).expect("Invalid utf8 message.");
                    println!("{}: {:?}", addr, msg);

                    tx.send(msg).expect("Failed to send msg to the rx.");
                }
                Err(ref error) if error.kind() == ErrorKind::WouldBlock => (),
                Err(_) => {
                    println!("closing connection with {}", addr);
                    break;
                }
            }

            thread::sleep(Duration::from_millis(100));
        });
    }

    pub fn share_messages(&mut self) {
        if let Ok(msg) = self.rx.try_recv() {
            for client in &mut self.clients {
                //let mut buff = msg.clone().into_bytes();
                //buff.resize(MSG_SIZE, 0);
                client
                    .send_message(&msg)
                    .expect("failed writing to client.");
                //client.write_all(&buff).expect("Failed writing to client.");
            }
        }

        thread::sleep(Duration::from_millis(100));
    }
}

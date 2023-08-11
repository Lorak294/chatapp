use std::io;

use shared::{Message, LOCAL_ADDRESS};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::mpsc,
};

#[tokio::main]
async fn main() {
    // connecting to a remote host
    let mut socket = TcpStream::connect(LOCAL_ADDRESS).await.unwrap();
    let myaddress = socket.local_addr().unwrap();
    let (tx, mut rx) = mpsc::channel::<Message>(10);

    tokio::spawn(async move {
        loop {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
            tokio::select! {
                // prinitng received message
                res = reader.read_line(&mut line) => {
                    let bytes_read = res.unwrap();
                        if bytes_read == 0 {
                            break;
                        }
                        let msg = Message::deserialize(line.clone());
                        msg.print();
                        line.clear();
                }

                // sending message transmited from stdin
                res = rx.recv() => {

                    let msg = res.unwrap();
                    writer.write_all(msg.serialize().as_bytes()).await.unwrap();
                }
            }
        }
    });

    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).unwrap();
        print!("\r");
        let msg_str = buff.trim().to_string();
        let msg = Message::UserMessage(msg_str.clone(), myaddress);

        if msg_str == ":quit" || tx.send(msg).await.is_err() {
            break;
        }
    }
    println!("goodbye :D");
}

use std::io;

use shared::{Message, LOCAL_ADDRESS};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::mpsc,
};

// function to get connection address from the user
fn read_host_address() -> String {
    println!("Enter server address. (for local harcoded address just type \":local\"):");
    let mut buff = String::new();
    io::stdin().read_line(&mut buff).unwrap();
    let msg_str = buff.trim();
    if msg_str == ":local" {
        return String::from(LOCAL_ADDRESS);
    }
    String::from(msg_str)
}

#[tokio::main]
async fn main() {
    // connecting to a remote host
    let mut socket;
    loop {
        let res = TcpStream::connect(&read_host_address()).await;
        match res {
            Ok(s) => {
                socket = s;
                break;
            }
            Err(_) => {
                println!("Unable to connect to given address, try again.")
            }
        }
    }

    //let mut socket = TcpStream::connect(LOCAL_ADDRESS).await.unwrap();
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

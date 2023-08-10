use std::{
    io::{self, ErrorKind, Read, Write},
    net::TcpStream,
    sync::mpsc::{self, TryRecvError},
    thread,
    time::Duration,
};

const LOCAL: &str = "127.0.0.1:8080";
const MSG_SIZE: usize = 64;

fn sleep() {
    thread::sleep(Duration::from_millis(100));
}

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect.");
    client
        .set_nonblocking(true)
        .expect("failed to initiate non-blocking mode.");

    let (tx, rx) = mpsc::channel::<String>();

    let _th = thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];

        match client.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                println!("message received: {:?}", msg);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection with server terminated.");
                client
                    .shutdown(std::net::Shutdown::Both)
                    .expect("failed shutting down the connection.");
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("writing to socket failed.");
                println!("message sent: {}", msg);
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        sleep();
    });

    println!("Write a message:");
    loop {
        let mut buff = String::new();
        io::stdin()
            .read_line(&mut buff)
            .expect("reading form stdin failed.");
        let msg = buff.trim().to_string();

        if msg == ":quit" || tx.send(msg).is_err() {
            break;
        }
    }
    println!("goodbye :D");
}

use shared::{Message, LOCAL_ADDRESS};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(LOCAL_ADDRESS).await.unwrap();
    //let (tx, _rx) = broadcast::channel::<String>(10);
    let (tx, _rx) = broadcast::channel::<Message>(10);

    loop {
        // blocking accept
        let (mut socket, address) = listener.accept().await.expect("failed during accepting.");
        println!("Accepted new connection from {}", address);
        // channel setup
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        // spawning new task to handle the connection
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    res = reader.read_line(&mut line) => {
                        let bytes_read = res.unwrap();
                        if bytes_read == 0 {
                            break;
                        }
                        //tx.send(line.clone()).unwrap();
                        //tx.send(Message::deserialize(line.clone())).unwrap();
                        let message = Message::UserMessage(line.clone(),address);
                        message.print();
                        tx.send(message).unwrap();

                        line.clear();
                    }

                    res = rx.recv() => {
                        let msg = res.unwrap();
                        writer.write_all(msg.serialize().as_bytes()).await.unwrap();
                    }

                }
            }
        });
    }
}

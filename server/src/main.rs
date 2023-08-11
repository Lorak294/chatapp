use shared::{Message, LOCAL_ADDRESS};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(LOCAL_ADDRESS).await.unwrap();
    println!(
        "Server running on address: {}",
        listener.local_addr().unwrap()
    );
    //let (tx, _rx) = broadcast::channel::<String>(10);
    let (tx, _rx) = broadcast::channel::<Message>(10);

    loop {
        // blocking accept
        let (mut socket, address) = listener.accept().await.expect("failed during accepting.");
        // channel setup
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        // spawning new task to handle the connection
        tokio::spawn(async move {
            // reader and writer setup
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            // sending message about new connection
            let sys_msg = Message::SystemMessage(format!("{} joined the server.", address));
            sys_msg.print();
            tx.send(sys_msg).unwrap();

            loop {
                tokio::select! {
                    // reading incomimng messeages
                    res = reader.read_line(&mut line) => {
                        match res {
                            // connection has been terminated
                            Err(_) | Ok(0) => {
                                // sending system message abut disconnected client
                                let sys_msg = Message::SystemMessage(format!("{} disconnected.", address));
                                sys_msg.print();
                                tx.send(sys_msg).unwrap();
                                break;
                            },

                            // some data has been received from the client
                            Ok(_) => {
                                // forwarding read messeages to the channel
                                let message = Message::deserialize(line.clone());
                                message.print();
                                tx.send(message).unwrap();
                                line.clear();
                            }
                        }
                    }

                    // forawrding messeages from the channel to all connected endpoints
                    res = rx.recv() => {
                        let msg = res.unwrap();
                        writer.write_all(msg.serialize().as_bytes()).await.unwrap();
                    }

                }
            }
        });
    }
}

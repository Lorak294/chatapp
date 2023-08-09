use server::Server;

mod server;

const LOCAL: &str = "127.0.0.1:6000";

fn main() {
    let mut server = Server::initialize(LOCAL).expect("err initializing the server.");

    loop {
        server.accept_new_connection();
        server.share_messages();
    }
}

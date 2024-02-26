use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1234").expect("failed to bind");

    println!("En attente d'un client...");
    match listener.accept() {
        Ok((client, addr)) => {
            println!("Nouveau client [adresse : {}]", addr);
        }
        _ => {
            println!("Un client a tenté de se connecter...")
        }
    }
}

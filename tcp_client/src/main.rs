use std::{io, thread};

mod net;
mod ui;

fn main() {
    println!("Tentative de connexion au serveur...");

    let mut stream = net::connect("127.0.0.1:1234", false);
    let mut client = stream.try_clone().expect("Erreur : Impossible de cloner le socket");

    thread::spawn(move || loop {
        let msg = net::handle_read(&mut stream);
        println!("Recieved : {}", msg);
    });
    
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("Reading from stdin failed");
        net::handle_write(&mut client, &buff);
    }
}

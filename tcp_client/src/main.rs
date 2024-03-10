use std::{io::{self, Read, Write}, net::TcpStream, str::from_utf8, thread};

mod net;
mod ui;

fn main() {
    println!("Tentative de connexion au serveur...");

    let mut stream = TcpStream::connect("127.0.0.1:1234").expect("Erreur : Impossible de se connecter au serveur distant");
    stream.set_nonblocking(false).expect("Erreur : Impossible de rendre le socket non-bloquant");

    let mut client = stream.try_clone().expect("Error cloning stream");
    // Read
    thread::spawn(move || loop {
        let mut data = [0 as u8; 2048];
        match stream.read(&mut data) {
            Ok(_) => {
                let msg = from_utf8(&data).unwrap();
                println!("Message recieved : {}", msg);
            }
            Err(e) => {
                println!("Failed to recieve data : {}", e);
            }
        }
    }); 

    // Write
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("Reading from stdin failed");

        let msg = buff.trim().to_string();
        client.write(msg.as_bytes());
    }
}

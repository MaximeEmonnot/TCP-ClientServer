use std::{io::{Read, Write}, net::TcpStream};

pub fn connect(address : &str, non_blocking : bool) -> TcpStream {
    let stream = TcpStream::connect(address).expect("Erreur : Impossible de se connecter au serveur distant");
    stream.set_nonblocking(non_blocking).expect("Erreur : Impossible de modifier l'aspect non-bloquant");
    return stream;
}

pub fn handle_read(stream : &mut TcpStream) -> String {
    let mut data = [0 as u8; 2048];
    match stream.read(&mut data) {
        Ok(_) => {
            return String::from_utf8((&data).to_vec()).unwrap();
        }
        Err(e) => {
            println!("Erreur : Impossible de récupérer les données : {}", e);
        }
    }
    return String::new();
}

pub fn handle_write(stream : &mut TcpStream, msg : &String) {
    let _ = stream.write(msg.trim().as_bytes());
}
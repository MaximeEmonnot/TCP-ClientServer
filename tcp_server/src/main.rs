use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, str::from_utf8, sync::{Arc, Mutex}, thread};

fn handle_client(mut stream: TcpStream, address : &str, clients : Arc<Mutex<Vec<std::net::TcpStream>>>)
{
    loop {
        let mut buffer = [0; 2048];
        let bytes_read = stream.read(&mut buffer).expect("Failed to read from socket");
        if bytes_read == 0 {
            println!("Client disconnected : {}", address);
            return;
        }
        println!("Recieved : {}", from_utf8(&buffer).unwrap());

        let mut clients = clients.lock().unwrap();
        for client in clients.iter_mut() {
            client.write_all(&buffer[..bytes_read]).expect("Failed to write to socket");
        }
    }
}

fn main() {
    let listener : TcpListener = TcpListener::bind("127.0.0.1:1234").unwrap();
    let clients = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming()
    {
        match stream {
            Ok(stream) => {
                let address : String = match stream.peer_addr() {
                    Ok(addr) => format!("[adresse : {}]", addr),
                    Err(_) => "inconnue".to_owned()
                };

                println!("Nouveau client : {}", address);
                let clients = Arc::clone(&clients);
                clients.lock().unwrap().push(stream.try_clone().expect("Failed to clone socket"));

                thread::spawn(move|| {
                    handle_client(stream, &*address, clients)
                });
            }
            Err(e) =>
            {
                println!("La connexion du client a échoué : {}", e);
            }
        }
    }
}

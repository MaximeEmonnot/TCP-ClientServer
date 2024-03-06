use std::{net::{TcpListener, TcpStream}, thread};

fn handle_client(mut stream: TcpStream)
{
    // TODO : Reception et renvoi de message
}

fn main() {
    let listener : TcpListener = TcpListener::bind("127.0.0.1:1234").unwrap();

    println!("En attente d'un client...");
    for stream in listener.incoming()
    {
        match stream {
            Ok(stream) => {
                let address : String = match stream.peer_addr() {
                    Ok(addr) => format!("[adresse : {}]", addr),
                    Err(_) => "inconnue".to_owned()
                };

                println!("Nouveau client : {}", address);

                thread::spawn(move|| {
                    handle_client(stream)
                });
            }
            Err(e) =>
            {
                println!("La connexion du client a échoué : {}", e);
            }
        }
    }
}

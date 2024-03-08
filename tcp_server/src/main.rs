use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, thread};

fn handle_client(mut stream: TcpStream, address : &str)
{
    let mut msg : Vec<u8> = Vec::new();
    loop {
        let mut buf = &mut [0; 10];

        match stream.read(buf){
            Ok(received) => {
                if received < 1 {
                    println!("Client disconnected {}", address);
                    return;
                }
                let mut x = 0;

                for c in buf {
                    if x >= received {
                        break;
                    }
                    x += 1;
                    if *c == '\n' as u8 {
                        println!("Message reçu {} : {}", address, String::from_utf8(msg).unwrap());
                        stream.write(b"ok\n");
                        msg = Vec::new();
                    } else {
                        msg.push(*c);
                    }
                }
            }
            Err(_) => {
                println!("Client disconnected : {}", address);
                return;
            }
        }
    }
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
                    handle_client(stream, &*address)
                });
            }
            Err(e) =>
            {
                println!("La connexion du client a échoué : {}", e);
            }
        }
    }
}

use std::{io::{stdin, Read, Write}, net::TcpStream};

pub fn connect(address: &str) {
    // L'objet TcpStream est globalement important et à des méthodes read write il faudra se renseigner.
    // la méthode connect prends en paramètre ip:port mais fonctionne aussi comme suit => (ip,port) (pas tout mais en 2 paramètres quoi)
    match TcpStream::connect(address) {
        Ok(stream) => {
            println!("Salut les PD !");
            client_routine(stream);
        }
        Err(e) => {
            println!("PAS CONNECTE {}", e);
        }
    }
}

fn get_entry() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf);
    return buf.replace("\n", "").replace("\r", "");
}

fn client_routine(mut stream: TcpStream) {
    let stdout = std::io::stdout();
    let mut io = stdout.lock();
    let mut buf = &mut[0; 3];

    println!("Enter 'quit' when you want to leave");

    loop {
        write!(io, ">");

        io.flush();
        match &*get_entry() {
            "quit" => {
                println!("Bye !");
                return;
            }
            line => {
                write!(stream, "{}\n", line);
                match stream.read(buf){
                    Ok(received) => {
                        if received < 1{
                            println!("Perte de la connexion avec le serveur");
                            return;
                        }
                    }
                    Err(_) => {
                        println!("Perte de la connexion avec le serveur");
                        return;
                    }
                }
                println!("Réponse du serveur : {:?}", buf);
            }
        }
    }
}

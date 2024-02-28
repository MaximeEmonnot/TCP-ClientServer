use std::net::TcpStream;

pub fn connect(address: &str) {
    // L'objet TcpStream est globalement important et à des méthodes read write il faudra se renseigner.
    // la méthode connect prends en paramètre ip:port mais fonctionne aussi comme suit => (ip,port) (pas tout mais en 2 paramètres quoi)
    match TcpStream::connect(address) {
        Ok(_) => {
            println!("Salut les PD !");
            client_routine();
        }
        Err(e) => {
            println!("PAS CONNECTE {}", e);
        }
    }
}

fn client_routine() {
    loop {
        println!("PD");
    }
}

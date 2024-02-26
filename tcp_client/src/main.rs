use std::net::TcpStream;

fn main() {
    println!("Tentative de connexion au serveur...");
    // L'objet TcpStream est globalement important et à des méthodes read write il faudra se renseigner.
    // la méthode connect prends en paramètre ip:port mais fonctionne aussi comme suit => (ip,port) (pas tout mais en 2 paramètres quoi)
    match TcpStream::connect("127.0.0.1:1234") {
        Ok(_) => {
            println!("Salut les PD !");
        }
        Err(e) => {
            println!("PAS CONNECTE {}", e);
        }
    }
}

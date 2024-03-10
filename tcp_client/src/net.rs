use std::{io::{stdin, Read, Write}, net::TcpStream, sync::mpsc};

/*
 * Objectif de la classe : 
 * - Connexion à un serveur via un socket
 * - Permettre la lecture des données provenant du serveur à tout moment
 * - Permettre l'envoi des données à tout moment
 */
/*
struct TcpSocket {
    stream: Option<TcpStream>,
    address: String,
}

impl TcpSocket {
    fn new() -> Self
    {
        Self {
            stream: None,
            address: "".to_string()
        }
    }
    
    fn connect(&mut self, address: &String)
    {
        self.address = address.clone();
        self.stream = TcpStream::connect(address).expect("Erreur : Impossible de se connecter au serveur distant");
    }
    
    fn set_nonblocking(&mut self, value: bool)
    {
        self.stream.set_nonblocking(value).expect("Erreur : Impossible rendre le socket non-bloquant");
    }
    
    fn handle_write(&mut self, msg : &String)
    {
        self.stream.write(msg.as_bytes()).unwrap();
        
        println!("Message envoyé avec succès ! : {}", msg);
    }
    
    fn handle_read(&mut self) -> String
    {
        let mut data = [0 as u8; 6];
        self.stream.read_exact(&mut data).expect("Erreur : Perte de connexion au serveur");
        return String::from_utf8(data.to_vec()).expect("Erreur : Impossible de convertir en String");
    }
}

pub struct Client {
    socket : TcpSocket,
    name : String,
}

impl Client {
    pub fn new(name: String) -> Self
    {
        Self{
            socket: TcpSocket::new(),
            name: name
        }
    }
    
    pub fn run(&self, address: &String)
    {
        self.socket.connect(address);
        self.socket.set_nonblocking(true);
        
        self.socket.handle_write(&"Gros PD".to_string());
    }
}
*/

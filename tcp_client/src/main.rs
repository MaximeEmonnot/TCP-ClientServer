mod net;
mod ui;

fn main() {
    println!("Tentative de connexion au serveur...");
    ui::run("GROS PD");
    net::connect("127.0.0.1:1234");
}

mod net;
mod ui;

fn main() {
    println!("Tentative de connexion au serveur...");
    ui::run::<ui::MyApp>("Client TCP");
}

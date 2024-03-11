use std::{net::TcpStream, sync::Arc, thread};

use eframe::egui;
use egui::mutex::Mutex;

use crate::net;

pub struct ClientGUI {
    ip_address: String,
    name: String,
    is_connected: bool,
    messages: Arc<Mutex<String>>,
    new_message: String,
    stream: Option<TcpStream>,
}

impl Default for ClientGUI {
    fn default() -> Self {
        Self {
            ip_address: "".to_string(),
            name: "".to_string(),
            is_connected: false,
            messages: Arc::new(Mutex::new("".to_string())),
            new_message: "".to_string(),
            stream: None,
        }
    }
}

impl ClientGUI {
    fn handle_read(&mut self) {
        if let Some(ref mut st) = self.stream {
            let messages = Arc::clone(&self.messages);
            let mut client = st
                .try_clone()
                .expect("Erreur : Impossible de cloner le stream");
            thread::spawn(move || loop {
                let msg = net::handle_read(&mut client);
                let mut messages = messages.lock();
                messages.push_str(msg.trim());
                messages.push_str("\n");
            });
        } else {
            println!("Erreur !");
        }
    }

    fn handle_write(&mut self, msg: &String) {
        if let Some(ref mut st) = self.stream {
            net::handle_write(st, msg);
        }
    }
}

impl eframe::App for ClientGUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Si le client est connecté
            if self.is_connected {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label(self.messages.lock().clone());
                });

                ui.allocate_ui_at_rect(
                    egui::Rect {
                        min: egui::Pos2 {
                            x: 0.0,
                            y: ui.available_height() - 50.0,
                        },
                        max: egui::Pos2 {
                            x: ui.available_width(),
                            y: ui.available_height(),
                        },
                    },
                    |ui| {
                        ui.text_edit_singleline(&mut self.new_message);

                        if ui.button("Envoyer").clicked() {
                            let mut data = String::new();
                            data.push_str(&self.name);
                            data.push_str(": ");
                            data.push_str(&self.new_message.trim());
                            self.handle_write(&mut data);

                            self.new_message.clear();
                        }
                    },
                );
            }
            // Sinon...
            else {
                ui.label("Username");
                ui.text_edit_singleline(&mut self.name);
                ui.label("Adresse IP");
                ui.text_edit_singleline(&mut self.ip_address);

                if ui.button("Connexion").clicked() {
                    if !self.name.is_empty() && !self.ip_address.is_empty() {
                        self.stream = Some(net::connect(self.ip_address.trim(), false));
                        self.is_connected = true;
                        self.handle_read();
                    }
                }
            }
        });
    }
}

pub fn run(window_name: &String) {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        window_name,
        options,
        Box::new(|cc| Box::<ClientGUI>::default()),
    );
}

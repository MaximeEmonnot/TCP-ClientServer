//use eframe::egui;

//use crate::net;
/*
pub struct MyApp {
    ip_address: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            ip_address: "".to_owned()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Enter the IP address of the distant server :");
            ui.text_edit_singleline(&mut self.ip_address);
            if ui.button("Connect").clicked() {
                net::connect(&self.ip_address);
            }
        });
    }
}

pub fn run<T: eframe::App + Default + 'static>(window_name: &str)
{
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(window_name, options, Box::new(|cc| Box::<T>::default()));
}
*/
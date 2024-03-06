
use eframe::egui;
struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Heading");
            ui.label("Label");
            ui.text_edit_singleline(&mut self.name);
            if ui.button("Button").clicked() {}
        });
    }
}

pub fn run(window_name: &str)
{
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(window_name, options, Box::new(|cc| Box::<MyApp>::default()));
}
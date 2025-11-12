use eframe::egui;

#[derive(Default)]
struct MyApp {
    counters: [i32; 3],
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Hello from egui!");
                ui.horizontal_centered(|ui| {
                    for (i, counter) in self.counters.iter_mut().enumerate() {
                        ui.vertical(|ui| {
                            ui.label(format!("Counter {}", i + 1));
                            let button = ui.button("Increment");
                            if button.clicked() {
                                *counter += 1;
                            }
                            ui.label(format!("Value: {}", counter));
                            ui.label("Yolo");
                        });
                    }
                })
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My App",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

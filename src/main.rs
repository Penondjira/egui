use eframe::egui::{self, Id};

struct MyApp {
    value1: i32,
    value2: i32,
    value3: i32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            value1: 0,
            value2: 0,
            value3: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Use CentralPanel to take up all available space.
        egui::CentralPanel::default().show(ctx, |_ui| {
            // Apply a centered layout for both horizontal and vertical alignment.
            egui::Area::new(Id::new("centered area"))
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        // Use a horizontal layout for the buttons.
                        // Add some spacing to separate the buttons.
                        ui.spacing_mut().item_spacing = egui::Vec2::new(10.0, 10.0);

                        // Center the vertical layout itself.
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            // First incrementer button
                            ui.horizontal(|ui| {
                                if ui.button("Increment 1").clicked() {
                                    self.value1 += 1;
                                }
                                ui.label(format!("Count: {}", self.value1));
                            });

                            // Second incrementer button
                            ui.horizontal(|ui| {
                                if ui.button("Increment 2").clicked() {
                                    self.value2 += 1;
                                }
                                ui.label(format!("Count: {}", self.value2));
                            });

                            // Third incrementer button
                            ui.horizontal(|ui| {
                                if ui.button("Increment 3").clicked() {
                                    self.value3 += 1;
                                }
                                ui.label(format!("Count: {}", self.value3));
                            });
                        });
                    });
                });
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Centered Incrementers",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

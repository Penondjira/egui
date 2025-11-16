use eframe::egui::{self, pos2, Color32, Id, Rect, Sense};

const PIP_WIDTH: f32 = 100.0;
const PIP_HEIGHT: f32 = 20.0;
const PIP_SPACING: f32 = 10.0;
const ROOM_SPACING: f32 = 50.0;

struct Pip {
    width: f32,
    height: f32,
}

impl Pip {
    fn new() -> Self {
        Self {
            width: PIP_WIDTH,
            height: PIP_HEIGHT,
        }
    }
}

impl eframe::App for Pip {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_ui| {
            egui::Area::new(Id::new("cenetered"))
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.allocate_exact_size(
                        egui::Vec2 {
                            x: (self.width),
                            y: (self.height),
                        },
                        Sense::click(),
                    )
                });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native("variety", options, Box::new(|_cc| Ok(Box::new(Pip::new()))));
}

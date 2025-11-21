use eframe::egui::{self, Align2, Id, Sense, Vec2};

const CORRECT_VALUES: &'static [i32] = &[3, 4, 8, 5];

struct PowerDisplay {
    pip_width: f32,
    pip_height: f32,
    room_spacing: f32,
    pip_spacing: f32,
    room_labels: Vec<String>,
    room_values: Vec<i32>,
    is_correct: bool,
}

impl eframe::App for PowerDisplay {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui
                .add_sized([200.0, 50.0], egui::Button::new("Activate"))
                .clicked()
            {
                let mut is_matching = true;
                for i in 0..self.room_values.len() {
                    if CORRECT_VALUES[i] != self.room_values[i] {
                        is_matching = false;
                    }
                }
                if is_matching {
                    self.is_correct = true;
                }
            }
            if self.is_correct {
                ui.label("Access Granted");
            } else {
                ui.label("Access Denied");
            }
            egui::Area::new(Id::new("central area"))
                .anchor(Align2::CENTER_TOP, Vec2::new(0.0, 50.0))
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing =
                            egui::Vec2::new(self.room_spacing, self.pip_spacing);
                        for i in 0..4 {
                            ui.vertical(|ui| {
                                ui.label(self.room_values[i].to_string());
                                ui.horizontal_top(|ui| {
                                    ui.spacing_mut().item_spacing =
                                        egui::Vec2::new(5.0, self.pip_spacing);
                                    if ui.add_sized([60.0, 60.0], egui::Button::new("^")).clicked()
                                        && self.room_values[i] < 10
                                        && self.is_correct == false
                                    {
                                        self.room_values[i] += 1;
                                    }
                                    if ui.add_sized([60.0, 60.0], egui::Button::new("v")).clicked()
                                        && self.room_values[i] > 1
                                        && self.is_correct == false
                                    {
                                        self.room_values[i] -= 1;
                                    }
                                });
                                ui.label(self.room_labels[i].clone());
                                for _i in 0..self.room_values[i] {
                                    let (pip, _resp) = ui.allocate_exact_size(
                                        Vec2::new(self.pip_width, self.pip_height),
                                        Sense::click(),
                                    );
                                    let _ = ui.painter().rect_filled(
                                        pip,
                                        5,
                                        egui::Color32::from_rgb(255, 255, 255),
                                    );
                                }
                            });
                        }
                    });
                });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_fullscreen(true)
            .with_decorations(false),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Power Puzzle",
        options,
        Box::new(|cc| {
            let mut style = (*cc.egui_ctx.style()).clone();
            style.text_styles = [
                (egui::TextStyle::Body, egui::FontId::proportional(24.0)),
                (egui::TextStyle::Button, egui::FontId::proportional(28.0)),
                (egui::TextStyle::Heading, egui::FontId::proportional(32.0)),
                (egui::TextStyle::Monospace, egui::FontId::proportional(24.0)),
            ]
            .into();
            cc.egui_ctx.set_style(style);
            Ok(Box::new(PowerDisplay {
                pip_width: 100.0,
                pip_height: 20.0,
                room_spacing: 50.0,
                pip_spacing: 10.0,
                room_labels: vec![
                    "Incubators".to_string(),
                    "AI".to_string(),
                    "Rockets".to_string(),
                    "Laboratory".to_string(),
                ],
                room_values: vec![3, 4, 8, 5],
                is_correct: false,
            }))
        }),
    );
}

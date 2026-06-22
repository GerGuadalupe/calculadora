mod buttons;

use super::calculadora::Calculadora;
use eframe::{egui::{self, Vec2}, emath::Numeric};

const RADIO_CORNERS: u8 = 40;
const APLICATION_H: f32 = 800.0;

static VIEWPORT_SIZE: Vec2 = Vec2::new(APLICATION_H * 0.8, APLICATION_H);
static DISPLAY_SIZE: f32 = APLICATION_H * 0.2;

pub fn launch() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(VIEWPORT_SIZE)
            .with_resizable(false),

        ..Default::default()
    };

    eframe::run_native(
        "Calculadora chida",
        native_options,
        Box::new(|_cc| Ok(Box::new(Calculadora::default()))),
    )
}

impl eframe::App for Calculadora {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(egui::Color32::from_rgb(25, 25, 25))
                    .outer_margin(0),
            )
            .show_inside(ui, |ui| {
                egui::Panel::top("panel del display")
                    .frame(
                        egui::Frame::new()
                            .corner_radius(egui::CornerRadius::default().at_least(RADIO_CORNERS))
                            .fill(egui::Color32::from_rgb(25, 150, 75))
                            .outer_margin(egui::Margin::same(25))
                    )
                    .default_size(DISPLAY_SIZE)
                    .show_inside(ui, |ui| {
                        ui.add_sized(
                            ui.available_size(),
                            egui::Label::new(
                                egui::RichText::new(&self.display)
                                    .size(60.0/{(self.display.len() as f32 + 4.0).log(5.0)})
                                    .color(egui::Color32::BLACK),
                            ),
                        )
                    });

                egui::CentralPanel::default()
                    .frame(egui::Frame::new().inner_margin(12.0))
                    .show_inside(ui, |ui| buttons::botones(ui, self));
            });
    }
}

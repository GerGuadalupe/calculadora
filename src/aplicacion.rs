mod buttons;

use super::calculadora::Calculadora;
use eframe::egui::{self, Vec2};

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
        "Calculadora",
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
                                egui::RichText::new(self.display())
                                    .size(60.0/{(self.display().len() as f32 + 4.0).log(10.0)})
                                    .color(egui::Color32::BLACK),
                            ),
                        )
                    });

                egui::CentralPanel::default()
                    .frame(egui::Frame::new().inner_margin(12.0))
                    .show_inside(ui, |ui| buttons::botones(ui, self));
            });

            if let Some(key) = ui.input(|i| input_teclado(i, self)){
                
                ui.input_mut(|i| i.consume_key(egui::Modifiers::default(), key));
            }

            
    }
}

fn input_teclado(i: &egui::InputState, calc: &mut Calculadora) -> Option<egui::Key>{
    for entrada in &i.events{
        if let egui::Event::Key{ key, physical_key: _, pressed, repeat, modifiers } = entrada{
            if !repeat && *pressed{
                if modifiers.is_none(){
                    match key {
                        egui::Key::Num0 => calc.push_in_display("0"),
                        egui::Key::Num1 => calc.push_in_display("1"),
                        egui::Key::Num2 => calc.push_in_display("2"),
                        egui::Key::Num3 => calc.push_in_display("3"),
                        egui::Key::Num4 => calc.push_in_display("4"),
                        egui::Key::Num5 => calc.push_in_display("5"),
                        egui::Key::Num6 => calc.push_in_display("6"),
                        egui::Key::Num7 => calc.push_in_display("7"),
                        egui::Key::Num8 => calc.push_in_display("8"),
                        egui::Key::Num9 => calc.push_in_display("9"),
                        egui::Key::Period => calc.push_in_display("."),

                        egui::Key::Enter => calc.calculate(),
                        egui::Key::Backspace => calc.delete(),

                        egui::Key::Plus => calc.push_in_display(" + "),
                        egui::Key::Minus => calc.push_in_display(" - "),
                        egui::Key::Slash => calc.push_in_display(" / "),
                        _ => {}
                    }
                } 
                else if modifiers.shift_only() && *key == egui::Key::Num6{
                    calc.push_in_display(" ^ ");
                }
            }
        }

        else if let egui::Event::Text(text) = entrada{
            if *text == "*".to_string(){
                calc.push_in_display(" * ");
            }
        }

    }
    None
}
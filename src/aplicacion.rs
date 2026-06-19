use eframe::egui::{self, Vec2};
use super::calculadora::Calculadora;
pub fn launch() -> eframe::Result{
    let native_options = eframe::NativeOptions{
        viewport : egui::ViewportBuilder::default()
            .with_inner_size(Vec2::from((800.0, 1000.0)))
            .with_resizable(false),
        
        ..Default::default()
    };

    eframe::run_native("Calculadora chida",
        native_options,
        Box::new(|_cc| Ok(Box::new(Calculadora::default())))
    )
}

impl eframe::App for Calculadora {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        
    }
}
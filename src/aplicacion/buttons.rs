use crate::calculadora::Calculadora;
use eframe::egui::{self, Vec2};

const BUTTON_CORNERS: u8 = 20;

pub fn botones(ui: &mut egui::Ui, calc: &mut Calculadora) {
    let size = set_button_size(ui.available_size(), 5.0, 4.0);

    egui::Grid::new("tablero de botones")
        .min_col_width(0.0)
        .show(ui, |ui| {
            add_button(ui, calc, "DEL", TipoInput::Del, &size);
            add_button(ui, calc, "^", TipoInput::Operator, &size);
            add_button(ui, calc, "*", TipoInput::Operator, &size);
            add_button(ui, calc, "/", TipoInput::Operator, &size);

            ui.end_row();

            add_button(ui, calc, "7", TipoInput::NumberOrDot, &size);
            add_button(ui, calc, "8", TipoInput::NumberOrDot, &size);
            add_button(ui, calc, "9", TipoInput::NumberOrDot, &size);
            add_button(ui, calc, "+", TipoInput::Operator, &size);

            ui.end_row();

            add_button(ui, calc, "4", TipoInput::NumberOrDot, &size);
            add_button(ui, calc, "5", TipoInput::NumberOrDot, &size);
            add_button(ui, calc, "6", TipoInput::NumberOrDot, &size);
            add_button(ui, calc, "-", TipoInput::Operator, &size);

            ui.end_row();

            add_button(ui, calc, "1", TipoInput::NumberOrDot, &size);
            add_button(ui, calc, "2", TipoInput::NumberOrDot, &size);
            add_button(ui, calc, "3", TipoInput::NumberOrDot, &size);
            add_button(ui, calc, "ANS", TipoInput::Ans, &size);
        });

    ui.horizontal(|ui| {
        add_button(ui, calc, "0", TipoInput::NumberOrDot, &size);
        add_button(ui, calc, ".", TipoInput::NumberOrDot, &size);
        add_button(ui, calc, "=", TipoInput::Ecual, &size);
    });
}

fn set_button_size(total_size: Vec2, rows: f32, columns: f32) -> Vec2 {
    Vec2 {
        x: total_size.x / columns - 6.0,
        y: total_size.y / rows - 2.0,
    }
}

fn add_button(
    ui: &mut egui::Ui,
    calc: &mut Calculadora,
    input: &str,
    tipo_input: TipoInput,
    size: &Vec2,
) {
    let mut size = size.clone();

    if ui
        .add_sized(
            //definiendo el tamaño
            if tipo_input == TipoInput::Ecual {
                size.x *= 2.0;
                size.x += 6.0;
                size
            } else {
                size
            },
            egui::Button::new(
                egui::RichText::new(input).size(40.0).color(
                    if tipo_input == TipoInput::Ecual || tipo_input == TipoInput::Del {
                        egui::Color32::BLACK
                    } else {
                        egui::Color32::WHITE
                    },
                ),
            )
            .corner_radius(BUTTON_CORNERS)
            .fill(
                if tipo_input == TipoInput::Del || tipo_input == TipoInput::Ecual {
                    egui::Color32::ORANGE
                } else {
                    egui::Color32::from_rgb(100, 100, 100)
                },
            ),
        )
        .clicked()
    {

        match tipo_input {
            TipoInput::Del => {
                calc.delete();
            }
            TipoInput::Ecual => calc.calculate(),
            TipoInput::NumberOrDot => calc.push_in_display(input),
            TipoInput::Operator => calc.push_in_display(&format!(" {} ", input)),
            TipoInput::Ans => calc.ans(),
        };
    }
}

#[derive(PartialEq)]
enum TipoInput {
    NumberOrDot,
    Operator,
    Ecual,
    Ans,
    Del,
}

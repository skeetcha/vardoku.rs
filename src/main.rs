use eframe::egui;
use sudokugen::BoardSize;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Vardoku",
        options,
        Box::new(|_| {
            Ok(Box::new(Vardoku::new()))
        })
    )
}

fn board_size_name(bs: &Option<BoardSize>) -> &str {
    match bs {
        Some(BoardSize::FourByFour) => "4x4",
        Some(BoardSize::NineByNine) => "9x9",
        Some(BoardSize::SixteenBySixteen) => "16x16",
        None => ""
    }
}

struct Vardoku {
    board_size: Option<BoardSize>,
    start: bool
}

impl Vardoku {
    fn new() -> Self {
        Self {
            board_size: None,
            start: false
        }
    }

    fn header(&self, s: impl Into<String>) -> egui::RichText {
        egui::RichText::new(s).color(egui::Color32::WHITE).size(32.0)
    }

    fn text(&self, s: impl Into<String>) -> egui::RichText {
        egui::RichText::new(s).color(egui::Color32::WHITE).size(14.0)
    }
}

impl eframe::App for Vardoku {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.start {

            } else {
                ui.vertical_centered(|ui| {
                    ui.label(self.header("Vardoku"));
                    ui.label(self.text("Choose your Board size"));

                    ui.horizontal(|ui| {
                        ui.add_space(340.0);
                        egui::ComboBox::from_id_salt("board_size").selected_text(board_size_name(&self.board_size)).show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.board_size, Some(BoardSize::FourByFour), "4x4");
                            ui.selectable_value(&mut self.board_size, Some(BoardSize::NineByNine), "9x9");
                            ui.selectable_value(&mut self.board_size, Some(BoardSize::SixteenBySixteen), "16x16");
                        });
    
                    });
                    
                    ui.label(self.text("Choose your difficulty"));
                    // combox box
                    let submit_button = ui.button("Submit");

                    if submit_button.clicked() {
                        // do stuff
                    }
                });
            }
        });
    }
}
use eframe::egui;
use rand::seq::SliceRandom;
use sudokugen::{Board, BoardSize, Puzzle};

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

#[derive(PartialEq)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
    Expert,
    Extreme
}

impl ToString for Difficulty {
    fn to_string(&self) -> String {
        match self {
            Self::Easy => "Easy".into(),
            Self::Medium => "Medium".into(),
            Self::Hard => "Hard".into(),
            Self::Expert => "Expert".into(),
            Self::Extreme => "Extreme".into()
        }
    }
}

struct Vardoku {
    board_size: Option<BoardSize>,
    start: bool,
    difficulty: Option<Difficulty>,
    board: Option<Board>
}

impl Vardoku {
    fn new() -> Self {
        Self {
            board_size: None,
            start: false,
            difficulty: None,
            board: None
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
                    
                    ui.horizontal(|ui| {
                        ui.add_space(340.0);
                        egui::ComboBox::from_id_salt("difficulty").selected_text(match &self.difficulty { Some(diff) => diff.to_string(), None => "".into() }).show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.difficulty, Some(Difficulty::Easy), "Easy");
                            ui.selectable_value(&mut self.difficulty, Some(Difficulty::Medium), "Medium");
                            ui.selectable_value(&mut self.difficulty, Some(Difficulty::Hard), "Hard");
                            ui.selectable_value(&mut self.difficulty, Some(Difficulty::Expert), "Expert");
                            ui.selectable_value(&mut self.difficulty, Some(Difficulty::Extreme), "Extreme");
                        });
                    });

                    let submit_button = ui.button("Submit");

                    if submit_button.clicked() {
                        let puzzle = Puzzle::generate(self.board_size.unwrap());
                        let mut board = puzzle.solution().clone();

                        let max_digit = match self.board_size {
                            Some(BoardSize::FourByFour) => 4u8,
                            Some(BoardSize::NineByNine) => 9,
                            Some(BoardSize::SixteenBySixteen) => 16,
                            None => panic!("Shouldn't happen")
                        };

                        let ptr = match self.difficulty {
                            Some(Difficulty::Easy) => 0.51,
                            Some(Difficulty::Medium) => 0.58,
                            Some(Difficulty::Hard) => 0.65,
                            Some(Difficulty::Expert) => 0.73,
                            Some(Difficulty::Extreme) => 0.75,
                            None => panic!("Please select a difficulty")
                        };

                        let mut cell_order = (0..max_digit.pow(2)).map(|val| (val % max_digit, val / max_digit)).collect::<Vec<(u8, u8)>>();
                        cell_order.shuffle(&mut rand::thread_rng());

                        for i in 0..((max_digit.pow(2) as f32 * ptr) as usize) {
                            board.unset(&board.cell_at(cell_order[i].0 as usize, cell_order[i].1 as usize));
                        }

                        self.board = Some(board.to_owned());
                    }
                });
            }
        });
    }
}
use std::fmt::Debug;

use eframe::egui;
use rand::prelude::*;

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

#[derive(PartialEq, Debug)]
enum BoardSize {
    Four,
    Nine,
    Sixteen,
    TwentyFive
}

fn board_size_name(bs: &Option<BoardSize>) -> &str {
    match bs {
        Some(BoardSize::Four) => "4x4",
        Some(BoardSize::Nine) => "9x9",
        Some(BoardSize::Sixteen) => "16x16",
        Some(BoardSize::TwentyFive) => "25x25",
        None => ""
    }
}

struct Vardoku {
    board_size: Option<BoardSize>,
    start: bool,
    board: Option<Board>
}

type Board = Vec<Vec<Cell>>;

#[derive(Clone)]
struct Cell {
    value: Option<i32>,
    notes: Vec<bool>,
    correct_value: i32
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            value: None,
            notes: Vec::new(),
            correct_value: 0
        }
    }
}

impl Vardoku {
    fn new() -> Self {
        Self {
            board_size: None,
            start: false,
            board: None
        }
    }

    fn header(&self, s: impl Into<String>) -> egui::RichText {
        egui::RichText::new(s).color(egui::Color32::WHITE).size(32.0)
    }

    fn text(&self, s: impl Into<String>) -> egui::RichText {
        egui::RichText::new(s).color(egui::Color32::WHITE).size(14.0)
    }

    fn random_gen(&self, num: i32) -> i32 {
        rand::thread_rng().gen_range(0..num)
    }

    fn setup_board(&mut self) {
        let srn = match self.board_size {
            Some(BoardSize::Four) => 4i32,
            Some(BoardSize::Nine) => 9,
            Some(BoardSize::Sixteen) => 16,
            Some(BoardSize::TwentyFive) => 25,
            None => panic!("Should not be happening")
        };

        self.board = Some(vec![vec![Default::default(); (srn * srn) as usize]; (srn * srn) as usize]);

        for i in (0..(srn * srn)).step_by(srn as usize) {
            let mut num: i32;
            let mut nums_used: Vec<i32> = Vec::new();

            for j in 0..srn {
                for k in 0..srn {
                    loop {
                        num = self.random_gen(srn * srn);

                        if nums_used.contains(&num) {
                            break;
                        }

                        if let Some(board) = &mut self.board {
                            board[(i + j) as usize][(i + k) as usize].correct_value = num;
                            board[(i + j) as usize][(i + k) as usize].value = Some(num);
                            nums_used.push(num);
                        } else {
                            panic!("This shouldn't happend");
                        }
                    }
                }
            }
        }
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
                            ui.selectable_value(&mut self.board_size, Some(BoardSize::Four), "4x4");
                            ui.selectable_value(&mut self.board_size, Some(BoardSize::Nine), "9x9");
                            ui.selectable_value(&mut self.board_size, Some(BoardSize::Sixteen), "16x16");
                            ui.selectable_value(&mut self.board_size, Some(BoardSize::TwentyFive), "25x25");
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
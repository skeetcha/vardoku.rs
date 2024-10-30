use eframe::egui;
use rand::seq::SliceRandom;
use sudokugen::{board::CellLoc, Board, BoardSize, Puzzle};
use egui_extras::Size;

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
    board: Option<Board>,
    notes: Option<Vec<Vec<Vec<bool>>>>,
    max_digit: u8,
    selected_cell: Option<CellLoc>,
    input_method: InputMethod,
    solution: Option<Board>
}

#[derive(PartialEq)]
enum InputMethod {
    Value,
    Candidate
}

impl Vardoku {
    fn new() -> Self {
        Self {
            board_size: None,
            start: false,
            difficulty: None,
            board: None,
            notes: None,
            max_digit: 0,
            selected_cell: None,
            input_method: InputMethod::Value,
            solution: None
        }
    }

    fn header(&self, s: impl Into<String>) -> egui::RichText {
        egui::RichText::new(s).color(egui::Color32::WHITE).size(32.0)
    }

    fn text(&self, s: impl Into<String>) -> egui::RichText {
        egui::RichText::new(s).color(egui::Color32::WHITE).size(14.0)
    }

    fn draw_cell_borders(&self, painter: &egui::Painter, rect: egui::Rect, row: usize, col: usize) {
        let light_border = egui::Stroke::new(1.0, egui::Color32::DARK_GRAY);
        let thick_border = egui::Stroke::new(2.0, egui::Color32::BLACK);
        let rem = (self.max_digit as f32).sqrt() as usize;

        let top_border = if row % rem == 0 { thick_border } else { light_border };
        let left_border = if col % rem == 0 { thick_border } else { light_border };
        let bottom_border = if row == (self.max_digit as usize - 1) { thick_border } else { light_border };
        let right_border = if col == (self.max_digit as usize - 1) { thick_border } else { light_border };

        painter.line_segment([rect.left_top(), rect.right_top()], top_border);
        painter.line_segment([rect.left_bottom(), rect.right_bottom()], bottom_border);
        painter.line_segment([rect.left_top(), rect.left_bottom()], left_border);
        painter.line_segment([rect.right_top(), rect.right_bottom()], right_border);
    }

    fn get_box_coords(&self, row: usize, col: usize) -> (usize, usize) {
        (row / (self.max_digit as f32).sqrt() as usize, col / (self.max_digit as f32).sqrt() as usize)
    }
}

impl eframe::App for Vardoku {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let selected_color = egui::Color32::from_rgba_unmultiplied(70, 0, 0, 200);
        let highlighted_color = egui::Color32::from_black_alpha(200);
        let default_color = egui::Color32::from_black_alpha(0);
        let cell_size = egui::vec2(15.0, 15.0);
        ctx.set_debug_on_hover(true);

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.start {
                ctx.set_pixels_per_point(match self.board_size {
                    Some(BoardSize::FourByFour) => 8.0,
                    Some(BoardSize::NineByNine) => 3.5,
                    Some(BoardSize::SixteenBySixteen) => 3.5,
                    None => panic!("question mark")
                });
                let mut grid = egui_grid::GridBuilder::new();

                for _ in 0..self.max_digit {
                    grid = grid.new_row(Size::exact(13.0)).cells(Size::exact(8.0), self.max_digit.into()).with_margin(egui::Margin::ZERO);
                }

                grid.show(ui, |mut grid| {
                    for i in 0..self.max_digit {
                        for j in 0..self.max_digit {
                            if let Some(val) = self.board.as_ref().unwrap().get_at(i as usize, j as usize) {
                                grid.cell(|ui| {
                                    let cell = ui.allocate_exact_size(cell_size, egui::Sense::click());
                                    
                                    let box_coords = self.get_box_coords(i as usize, j as usize);
                                    let selected_box_coords = if let Some(cell) = &self.selected_cell {
                                        self.get_box_coords(cell.line(), cell.col())
                                    } else { (30, 30) };

                                    let bg_color = match &self.selected_cell {
                                        Some(cell) => if *cell == self.board.as_ref().unwrap().cell_at(i as usize, j as usize) {
                                            selected_color
                                        } else if (cell.col() == j as usize) || (cell.line() == i as usize) {
                                            highlighted_color
                                        } else if box_coords == selected_box_coords {
                                            highlighted_color
                                        } else {
                                            default_color
                                        },
                                        None => default_color
                                    };

                                    let text_color = match &self.selected_cell {
                                        Some(cell_loc) => if cell_loc.line() == i as usize && cell_loc.col() == j as usize {
                                            if self.board.as_ref().unwrap().get_at(i as usize, j as usize).unwrap_or(30) == self.solution.as_ref().unwrap().get(cell_loc).unwrap_or(30) {
                                                egui::Color32::WHITE
                                            } else {
                                                egui::Color32::RED
                                            }
                                        } else {
                                            egui::Color32::WHITE
                                        },
                                        None => egui::Color32::WHITE
                                    };

                                    let cell_rect = cell.0.expand(0.5);
                                    let painter = ui.painter();

                                    painter.rect_filled(cell_rect, 0.0, bg_color);
                                    painter.text(cell_rect.center(), egui::Align2::CENTER_CENTER, val.to_string(), egui::TextStyle::Body.resolve(&ui.ctx().style()), text_color);
                                    self.draw_cell_borders(painter, cell_rect, i as usize, j as usize);

                                    if cell.1.clicked() {
                                        self.selected_cell = Some(self.board.as_ref().unwrap().cell_at(i as usize, j as usize));
                                    }
                                });
                            } else {
                                grid.cell(|ui| {
                                    let cell = ui.allocate_exact_size(cell_size, egui::Sense::click());

                                    let box_coords = self.get_box_coords(i as usize, j as usize);
                                    let selected_box_coords = if let Some(cell) = &self.selected_cell {
                                        self.get_box_coords(cell.line(), cell.col())
                                    } else { (30, 30) };

                                    let bg_color = match self.selected_cell {
                                        Some(cell) => if cell == self.board.as_ref().unwrap().cell_at(i as usize, j as usize) {
                                            selected_color
                                        } else if (cell.col() == j as usize) || (cell.line() == i as usize) {
                                            highlighted_color
                                        } else if box_coords == selected_box_coords {
                                            highlighted_color
                                        } else {
                                            default_color
                                        },
                                        None => default_color
                                    };

                                    let cell_rect = cell.0.expand(0.5);
                                    let painter = ui.painter();

                                    painter.rect_filled(cell_rect, 0.0, bg_color);
                                    self.draw_cell_borders(painter, cell_rect, i as usize, j as usize);

                                    let mut note_grid = egui_grid::GridBuilder::new();

                                    for _ in 0..((self.max_digit as f32).sqrt() as u8) {
                                        note_grid = note_grid.new_row(Size::exact(15.0 / (self.max_digit as f32).sqrt())).cells(Size::exact(15.0 / (self.max_digit as f32).sqrt()), (self.max_digit as f32).sqrt() as i32).with_margin(egui::Margin::ZERO);
                                    }

                                    ui.horizontal_centered(|ui| {
                                        note_grid.spacing(0.0, 0.0).show(ui, |mut note_grid| {
                                            for k in 0..self.max_digit {
                                                if *self.notes.as_ref().unwrap().get(i as usize).unwrap().get(j as usize).unwrap().get(k as usize).unwrap() {
                                                    note_grid.cell(|ui| {
                                                        ui.allocate_exact_size(egui::vec2(15.0 / (self.max_digit as f32).sqrt(), 15.0 / (self.max_digit as f32).sqrt()), egui::Sense::hover());
    
                                                        ui.label(egui::RichText::new((k + 1).to_string()).color(egui::Color32::WHITE).size(5.0));
                                                    });
                                                } else {
                                                    note_grid.empty();
                                                }
                                            }
                                        });
                                    });

                                    if cell.1.clicked() {
                                        self.selected_cell = Some(self.board.as_ref().unwrap().cell_at(i as usize, j as usize));
                                    }
                                });
                            }
                        }
                    }
                });

                let value_button = egui::Button::new("Value");
                let vb_resp = ui.add_enabled(self.input_method == InputMethod::Candidate, value_button);
                let candidate_button = egui::Button::new("Candidate");
                let cb_resp = ui.add_enabled(self.input_method == InputMethod::Value, candidate_button);

                if vb_resp.clicked() {
                    self.input_method = InputMethod::Value;
                }

                if cb_resp.clicked() {
                    self.input_method = InputMethod::Candidate;
                }

                ui.input(|input| {
                    if let None = &self.selected_cell {
                        return;
                    }

                    let key_to_num = (0..self.max_digit).map(|val| {
                        match val {
                            0 => (egui::Key::Num1, 1),
                            1 => (egui::Key::Num2, 2),
                            2 => (egui::Key::Num3, 3),
                            3 => (egui::Key::Num4, 4),
                            4 => (egui::Key::Num5, 5),
                            5 => (egui::Key::Num6, 6),
                            6 => (egui::Key::Num7, 7),
                            7 => (egui::Key::Num8, 8),
                            8 => (egui::Key::Num9, 9),
                            9 => (egui::Key::A, 10),
                            10 => (egui::Key::B, 11),
                            11 => (egui::Key::C, 12),
                            12 => (egui::Key::D, 13),
                            13 => (egui::Key::E, 14),
                            14 => (egui::Key::F, 15),
                            15 => (egui::Key::G, 16),
                            _ => panic!("Not supported")
                        }
                    }).collect::<Vec<(egui::Key, u8)>>();

                    for (key, num) in key_to_num {
                        if num > self.max_digit {
                            return;
                        }

                        if input.key_pressed(key) {
                            if self.input_method == InputMethod::Value {
                                let val = self.board.as_mut().unwrap().set(&self.selected_cell.unwrap(), num);

                                if let Some(old_val) = val {
                                    if old_val == num {
                                        self.board.as_mut().unwrap().unset(&self.selected_cell.unwrap());
                                    }
                                }
                            } else {
                                let col = self.selected_cell.as_ref().unwrap().col();
                                let row = self.selected_cell.as_ref().unwrap().line();

                                let val = self.notes.as_mut().unwrap().get_mut(row).unwrap().get_mut(col).unwrap().get_mut(num as usize - 1).unwrap();

                                *val = !*val;
                            }
                            
                            break;
                        }
                    }
                });
            } else {
                ui.vertical_centered(|ui| {
                    ui.label(self.header("Vardoku"));
                    ui.label(self.text("Choose your Board size"));

                    ui.horizontal(|ui| {
                        ui.add_space(340.0);
                        egui::ComboBox::from_id_source("board_size").selected_text(board_size_name(&self.board_size)).show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.board_size, Some(BoardSize::FourByFour), "4x4");
                            ui.selectable_value(&mut self.board_size, Some(BoardSize::NineByNine), "9x9");
                            //ui.selectable_value(&mut self.board_size, Some(BoardSize::SixteenBySixteen), "16x16");
                        });
                    });
                    
                    ui.label(self.text("Choose your difficulty"));
                    
                    ui.horizontal(|ui| {
                        ui.add_space(340.0);
                        egui::ComboBox::from_id_source("difficulty").selected_text(match &self.difficulty { Some(diff) => diff.to_string(), None => "".into() }).show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.difficulty, Some(Difficulty::Easy), "Easy");
                            ui.selectable_value(&mut self.difficulty, Some(Difficulty::Medium), "Medium");
                            ui.selectable_value(&mut self.difficulty, Some(Difficulty::Hard), "Hard");
                            ui.selectable_value(&mut self.difficulty, Some(Difficulty::Expert), "Expert");
                            ui.selectable_value(&mut self.difficulty, Some(Difficulty::Extreme), "Extreme");
                        });
                    });

                    let submit_button = ui.button("Submit");

                    if submit_button.clicked() {
                        let first_instant = std::time::Instant::now();
                        println!("{:?} - Generating puzzle...", std::time::Instant::now().duration_since(first_instant));
                        let puzzle = Puzzle::generate(self.board_size.unwrap());
                        println!("{:?} - Grabbing board...", std::time::Instant::now().duration_since(first_instant));
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

                        println!("{:?} - Removing random values...", std::time::Instant::now().duration_since(first_instant));

                        for i in 0..((max_digit.pow(2) as f32 * ptr) as usize) {
                            board.unset(&board.cell_at(cell_order[i].0 as usize, cell_order[i].1 as usize));
                        }

                        println!("{:?} - Done\nSetting up game.", std::time::Instant::now().duration_since(first_instant));

                        self.board = Some(board.to_owned());
                        self.solution = Some(puzzle.solution().to_owned());
                        self.start = true;
                        self.notes = Some((0..(max_digit as usize)).map(|_| {
                            (0..(max_digit as usize)).map(|_| {
                                (0..max_digit).map(|_| false).collect()
                            }).collect()
                        }).collect());
                        self.max_digit = max_digit;
                    }
                });
            }
        });
    }
}
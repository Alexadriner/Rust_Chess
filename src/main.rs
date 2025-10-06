mod game_objects;
use game_objects::chess_piece;
use crate::game_objects::chess_piece::Chess_piece;

mod gui;
use gui::chess_gui;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Ok(Box::new(chess_gui::App::default()))),
    )
}
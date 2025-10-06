use eframe::egui;
use egui::ColorImage;
use std::collections::HashMap;

pub struct App {
    textures: HashMap<&'static str, egui::TextureHandle>,
}

impl App {
    fn load_image(ctx: &egui::Context, name: &'static str, bytes: &[u8]) -> egui::TextureHandle {
        let image = image::load_from_memory(bytes).unwrap().to_rgba8();
        let size = [image.width() as usize, image.height() as usize];
        let color_image = ColorImage::from_rgba_unmultiplied(size, image.as_raw());
        ctx.load_texture(name, color_image, egui::TextureOptions::default())
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        if self.textures.is_empty() {
            //add image to images here
            let images = [
                ("chess_pieces", include_bytes!("../../textures/chess_pieces.png") as &[u8]),
            ];

            for (name, bytes) in images {
                let texture = Self::load_image(ctx, name, bytes);
                self.textures.insert(name, texture);
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            //render images here
            ui.horizontal(|ui| {
                ui.image(&self.textures["chess_pieces"]);
            });
        });
    }
}
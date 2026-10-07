use eframe::egui::{self/*, Vec2, Pos2*/}; 

mod colour_palette;
use colour_palette::*; 

pub const COLOUR_PALETTE: ColourPalette = ColourPalette{background: 0xFFF0C5, borders: 0x359FA0, points: 0xFF8C52};
 
fn main() -> eframe::Result<()> {
   
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Voronoi Diagram")
            .with_resizable(true), 
            ..Default::default() 
    };   
    eframe::run_native(
        "Voronoi", 
        options, 
        Box::new(|_cc| Ok(Box::new(VoronoiApp {}))),
        )  
}

struct VoronoiApp {}

impl eframe::App for VoronoiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let background_rgb = to_rgb(COLOUR_PALETTE.background());
       
        ui.painter().rect_filled(ui.max_rect(), 0.0, egui_colour(background_rgb));
    }
}

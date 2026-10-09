use eframe::egui::{self, epaint::Pos2, epaint::CircleShape}; 

mod voronoiapp; 
use voronoiapp::*;  

fn main() -> eframe::Result<()> {
   
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Voronoi Diagram")
            .with_resizable(true), 
            ..Default::default() 
    };   

    //Initialising Voronoi App struct
    let seed_number: usize = seed_number(); 
    let seed_coordinates = seed_coordinates(seed_number); 
    let background_colour: u32 = 0xFFF0C5;  
    let border_colour: u32 = 0x359FA0; 
    let seed_colour:u32 = 0xFF8C52;

    let voronoi_app = VoronoiApp::new(seed_number, 
        seed_coordinates, background_colour, border_colour, seed_colour);

    eframe::run_native(
        "Voronoi", 
        options, 
        Box::new(|_cc| 
            Ok(Box::new(voronoi_app))),
        )  
}

impl eframe::App for VoronoiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        //Window background
        let window_frame = ui.max_rect(); 
        let background_rgb = to_rgb(self.background_colour());
        ui.painter().rect_filled(window_frame, 
            0.0, egui_colour(background_rgb));

        let width = window_frame.max.x - window_frame.min.x; 
        let height = window_frame.max.y - window_frame.min.y; 
        //Drawing circles painted with COLOUR_PALETTE.points colour
        for i in 0..self.seed_number() {
            let (x, y) = (0.01*width*self.seed_coordinates[i].0, 
                0.01*height*self.seed_coordinates[i].1);
            let circle_centre = Pos2::new(x, y);
            
            let circle_radius: f32 = 3.0; 
            let circle_colour = egui_colour(to_rgb(self.seed_colour()));
        
            let circle = CircleShape::filled(circle_centre, 
                circle_radius, circle_colour);
            ui.painter().add(circle);
        }
    }
}

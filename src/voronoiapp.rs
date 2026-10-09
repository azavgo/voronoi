use rand::random_range; 

pub struct VoronoiApp {
    seed_number: usize, 
    pub seed_coordinates: Box<Vec<(f32, f32)>>,
    // Colour Palette 
    background_colour: u32, 
    border_colour: u32, 
    seed_colour: u32,
}

impl VoronoiApp {
    pub fn new(seed_number: usize, seed_coordinates: Box<Vec<(f32, f32)>>, 
        background_colour: u32, border_colour: u32, seed_colour:u32) -> Self {
        Self {seed_number, seed_coordinates, background_colour, border_colour, 
        seed_colour}
    }

    pub fn seed_number(&self) -> usize {
        self.seed_number
    }

    pub fn background_colour(&self) -> u32 {
        self.background_colour 
    }
    
    pub fn border_colour(&self) -> u32 {
        self.border_colour
    }

    pub fn seed_colour(&self) -> u32 {
        self.seed_colour
    }
}

// Generates random number of seeds 
pub fn seed_number() -> usize {
    random_range(0..100)
}

// Generates random coordinates for the seeds
// (f32, f32) = (x%, y%) - x and y are coordinates in % from max width and 
// max height of the Window Frame
pub fn seed_coordinates(seed_number: usize) -> Box<Vec<(f32, f32)>> {
    let mut coordinates = vec![(0.0f32, 0.0f32); seed_number];
    coordinates.iter_mut().for_each(|e| {
            e.0 = random_range(0.0..=100.0); 
            e.1 = random_range(0.0..=100.0); 
            });
    
    Box::new(coordinates)
}

pub struct RGB {
    red: u8, 
    green: u8, 
    blue: u8,
}

impl RGB {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self{red, green, blue}
    }

    pub fn red(&self) -> u8 {
        self.red
    }

    pub fn green(&self) -> u8 {

        self.green
    }

    pub fn blue(&self) -> u8 {
        self.blue
    }
}

pub fn to_rgb(colour: u32) -> RGB {
    let red = (colour>>16 & 0xFF) as u8; 
    let green = (colour>>8 & 0xFF) as u8; 
    let blue = (colour & 0xFF) as u8; 

    RGB::new(red, green, blue)
}

pub fn egui_colour(rgb: RGB) -> eframe::egui::Color32 {
    eframe::egui::Color32::from_rgb(rgb.red(), rgb.green(), rgb.blue())
}
   
 

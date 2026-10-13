use rand::random_range; 
use eframe::egui::Color32; 

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
    random_range(2..100)
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

pub fn border_coordinates(seeds: Box<Vec<(f32, f32)>>) -> 
Box<Vec<(f32, f32)>> {
    let mut border_points: Vec<(f32, f32)> = Vec::new();
    let mut distances: Vec<f32> = Vec::new(); 
    let l = seeds.len(); 
    let start: f32 = 0.0;
    let step: f32 = 0.02;
    let total_steps = 5000; 
    for j in 0..= total_steps {
        for k in 0..= total_steps {
            for i in 0..l {
                let x: f32 = start + (j as f32) * step; 
                let y: f32 = start + (k as f32) * step;  
                border_points.push((x, y)); 
                let d = (x - seeds[i].0) * 
                        (x - seeds[i].0) + 
                        (y - seeds[i].1) * 
                        (y - seeds[i].1);
                distances.push(d); 
            }
            distances.sort_unstable_by(|a, b| a.total_cmp(b));
            if distances[1] - distances[0] > 2.0 * step {
                border_points.pop(); 
            } 
            distances.clear(); 
        }
    } 
    Box::new(border_points)
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

pub fn egui_colour(rgb: RGB) -> Color32 {
    Color32::from_rgb(rgb.red(), rgb.green(), rgb.blue())
}
   
 

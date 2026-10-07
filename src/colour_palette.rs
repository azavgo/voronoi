use std::fmt; 

pub struct ColourPalette {
    pub background: u32, 
    pub borders: u32, 
    pub points: u32,
}

impl ColourPalette {
    pub fn new(background: u32, borders: u32, points: u32) -> Self {
        Self{background, borders, points}
    }

    pub fn background(&self) -> u32 {
        self.background
    }

    pub fn borders(&self) -> u32 {
        self.borders
    }

    pub fn points(&self) -> u32 {
        self.points
    }
}

impl fmt::Display for ColourPalette {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write to the formatter using the uppercase hex format {:02X}
        // :02X means: 0-padded, 2 characters wide, Uppercase Hex
        write!(f, "background: #{:02X}, borders: #{:02X}, points: #{:02X}", self.background(), self.borders(), self.points())
    }
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

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "red: {}, green: {}, blue: {}", self.red(), self.green(), self.blue())
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
   
 

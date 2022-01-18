/// Struct to hold and work with the ascii image
pub struct AsciiImage {
    pub dimensions: (u32, u32),
    pub characters: Vec<char>
}

impl AsciiImage {
    
    /// Prints the ascii image to the console
    pub fn print(&self) {
        for y in 0..self.dimensions.1 {
            // Only drawing half of the lines to display undistorded img
            if y % 2 == 0 {
                for x in 0..self.dimensions.0 {
                    print!("{}", self.characters[(x * self.dimensions.1 + y) as usize]);
                }
                print!("\n");
            }
        }
    }

    /// Return a String of the ascii image
    pub fn as_string(&self) -> String {

        // Generate the string
        let mut string = String::new();
        for y in 0..self.dimensions.1 {
            // Only writing half of the lines to display undistorded img
            if y % 2 == 0 {
                for x in 0..self.dimensions.0 {
                    string.push(self.characters[(x * self.dimensions.1 + y) as usize]);
                }
                string.push('\n');
            }
        }
        
        string
    }
}
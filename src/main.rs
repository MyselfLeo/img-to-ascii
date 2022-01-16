use image::GenericImageView;
use std::env;


const SYMBOLS: [char; 5] = [' ', '░', '▒', '▓', '█'];
const MAX_WIDTH: u32 = 100;





struct AsciiImage {
    dimensions: (u32, u32),
    characters: Vec<char>
}

impl AsciiImage {
    fn print(&self) {
        for x in 0..self.dimensions.0 {
            for y in 0..self.dimensions.1 {
                print!("{}", self.characters[(y * self.dimensions.1 + x) as usize]);
            }
            print!("\n");
        }
    }
}




fn get_output_dimensions(img_dimensions: (u32, u32)) -> (u32, u32) {
    // Return the dimensions of the outputed ascii image, as its width cannot be greater than MAX_WIDTH.

    if img_dimensions.0 > MAX_WIDTH {
        let ratio = MAX_WIDTH as f64 / img_dimensions.0 as f64;
        return (MAX_WIDTH, (img_dimensions.1 as f64 * ratio) as u32);
    }
    else {
        return img_dimensions;
    }
}





fn convert_file(filepath: &String) -> AsciiImage {
    // Load image
    let image = image::open(filepath).expect("[ERROR] Unable to load the image.");
    let dimensions = image.dimensions();
    println!("[INFO] Image dimensions: {:?}", dimensions);

    // Compute output dimensions
    let output_dimensions = get_output_dimensions(dimensions);
    println!("[INFO] Output dimensions will be: {:?}", output_dimensions);

    // Create AsciiImage struct
    let mut ascii_image = AsciiImage{
        dimensions: output_dimensions,
        characters: Vec::new(),
    };

    // Read pixels from the image and convert them to ascii characters
    // TODO: Implement
}





fn main() {
    // Get file name from args
    let args: Vec<String> = env::args().collect();
    let mut filepath: &String = &String::new();
    match args.len() {
        1 => println!("[ERROR]] Please enter a valid path to the image."),
        _ => {filepath = &args[1];}
    }

    // Convert file
    convert_file(filepath);
}

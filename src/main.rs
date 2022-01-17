use image::GenericImageView;
use clap::Parser;
use std::fs::File;
use std::io::Write;

const SYMBOLS: [char; 10] = [' ', '.', ':', ';', '+', 'o', 'O', '&', '@', '█'];
const DEFAULT_MAX_WIDTH: u32 = 100;





struct AsciiImage {
    dimensions: (u32, u32),
    characters: Vec<char>
}

impl AsciiImage {
    /// Prints the ascii image to the console
    fn print(&self) {
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
    fn as_string(&self) -> String {

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




fn get_output_dimensions(img_dimensions: (u32, u32), width: u32) -> (u32, u32) {
    // Return the dimensions of the outputed ascii image
    let ratio = img_dimensions.0 as f32 / img_dimensions.1 as f32;
    (width, (width as f32 / ratio) as u32)
}





fn convert_file(filepath: &String, width: Option<u32>) -> AsciiImage {
    // Select the correct width for the outputed image
    let width = width.unwrap_or(DEFAULT_MAX_WIDTH);

    // Load image
    let mut image = image::open(filepath).expect("[ERROR] Unable to load the image.");
    let dimensions = image.dimensions();

    // Compute output dimensions
    let output_dimensions = get_output_dimensions(dimensions, width);

    // Create AsciiImage struct
    let mut ascii_image = AsciiImage{
        dimensions: output_dimensions,
        characters: Vec::new(),
    };

    // Convert the image to greyscale
    image = image.grayscale();
    let ratio = dimensions.0 / output_dimensions.0; // an image of 1000w converted to 100w => ratio = 10

    for x in 0..output_dimensions.0 {
        for y in 0..output_dimensions.1 {
            let pixel = image.get_pixel(x * ratio, y * ratio);
            let average = (pixel[0] as f64 + pixel[1] as f64 + pixel[2] as f64) / 3 as f64;

            // This code was generated by autopilot. Thank you autopilot, very cool.
            ascii_image.characters.push(SYMBOLS[(average / (255 as f64 / (SYMBOLS.len() - 1) as f64)) as usize]);
        }
    }

    ascii_image
}



// Args parsing using clap
#[derive(Parser)]
#[clap(author = "myselfleo", version = "0.1.0", about = "A simple image to ascii converter")]
struct Args {
    /// The path to the image to convert
    filepath: String,

    /// Specifies a file to write the output to, instead of printing it to the console
    #[clap(short, long)]
    output: Option<String>,

    /// Specifies the width of the outputted image
    #[clap(short, long)]
    width: Option<u32>,
}




fn main() {
    // Get arguments from command line
    let args = Args::parse();

    // Convert file
    let ascii_image = convert_file(&args.filepath, args.width);

    if args.output.is_some() {
        // Write to file
        let mut file = File::create(args.output.unwrap()).expect("[ERROR] Unable to create the output file.");
        file.write(ascii_image.as_string().as_bytes()).expect("[ERROR] Unable to write to the output file.");
    }
    else {
        // Print to console
        ascii_image.print();
    }
}

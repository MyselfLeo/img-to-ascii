use image::io::Reader as ImageReader;
use std::env;


const SYMBOLS: [char; 5] = [' ', '░', '▒', '▓', '█'];




fn convert_file(filepath: &String) -> &String {
    // Take in input an image file and return a str representing this img in ascii format
    return filepath;
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


    println!("{}", filepath);
}

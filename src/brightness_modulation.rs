// Brightness is an integer between 0 and 255.
// The brightness of a pixel in an image is considered its "raw brightness".
// The brightness_modulation functions will take the raw brightness and apply a brightness modulation.
// The outputed brightness will still be between 0 and 255.


fn linear_modulation(brightness: f64, mult: f64) -> f64 {
    brightness * mult
}
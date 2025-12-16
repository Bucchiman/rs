use std::error::Error;
use std::io::Cursor;
use image::ImageReader;

fn main() -> Result<(), Box<dyn Error>> {
    let img = ImageReader::open("rain.jpg")?.decode()?;
    let img2 = ImageReader::new(Cursor::new(bytes)).with_guessed_format()?.decode()?;
    Ok(())
}

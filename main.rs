use image::{DynamicImage, GenericImageView, Pixel};
use std::fs;

fn main() {
    // Usefull variables
    let image_path: &str = "resources/";
    let image_name: &str = "image.png";
    let output_file_name: &str = "./output.txt";

    let greyscale: [char; 69] = [
        ' ', '.', '\'', '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '>', '<', '~', '+', '_',
        '-', '?', ']', '[', '}', '{', '1', ')', '(', '|', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u',
        'v', 'c', 'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd',
        'b', 'k', 'h', 'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$',
    ];

    let mut img: DynamicImage = image::open(format!("{}{}", image_path, image_name)).unwrap();

    // If the image is bigger than 500x500 resize the image to the correct size
    if img.dimensions().0 > 500 && img.dimensions().1 > 500 {
        img.resize(500, 500, image::imageops::FilterType::Lanczos3);
    }
    img = img.grayscale();

    let mut final_string: String = "".to_owned();
    for y in 0..img.dimensions().1 {
        for x in 0..img.dimensions().0 {
            final_string.push(
                greyscale[((greyscale.len() - 1) as f32
                    * (img.get_pixel(x, y).to_rgb().channels()[0] as f32 / 255.0))
                    as usize],
            );
        }
        final_string.push('\n');
    }

    // Write file with the ASCII values into
    let res: Result<(), std::io::Error> = fs::write(output_file_name, final_string);
    if res.is_ok() {
        println!("El archivo se ha guardado correctamente");
    } else {
        panic!("Error al guardar los datos en el archivo")
    }
}

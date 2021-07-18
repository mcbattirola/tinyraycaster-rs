fn main() {
    const IMAGE_WIDTH: usize = 512;
    const IMAGE_HEIGHT: usize = 512;

    let mut frame_buffer = [255; IMAGE_WIDTH * IMAGE_HEIGHT];

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let c = ColorChannel {
                red: (255 * j / IMAGE_HEIGHT) as u8,
                green: (255 * i / IMAGE_HEIGHT) as u8,
                blue: 0,
                alpha: 0,
            };

            frame_buffer[i + j * IMAGE_WIDTH] = pack_color(c);
        }
    }

    drop_ppm_image(
        "./out.ppm",
        &frame_buffer,
        IMAGE_WIDTH as i32,
        IMAGE_HEIGHT as i32,
    ).unwrap();
}

struct ColorChannel {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

fn pack_color(c: ColorChannel) -> i32 {
    return ((c.alpha as i32) << 24)
        + ((c.blue as i32) << 16)
        + ((c.green as i32) << 8)
        + (c.red as i32);
}

fn unpack_color(color: i32) -> ColorChannel {
    return ColorChannel {
        red: (((color >> 0)) & 255) as u8,
        green: (((color >> 8)) & 255) as u8,
        blue: (((color >> 16)) & 255) as u8,
        alpha: (((color >> 24)) & 255) as u8,
    };
}

fn drop_ppm_image(filename: &str, image: &[i32], width: i32, height: i32) -> std::io::Result<()> {
    use std::io::prelude::*;

    let mut file = std::fs::File::create(filename)?;

    let file_header =
        String::from("P6\n") + &width.to_string() + " " + &height.to_string() + "\n255\n";
    file.write_all(file_header.as_bytes())?;

    for i in 0..(height * width) {
        let c = unpack_color(image[i as usize]);
        file.write_all(&[c.red, c.green, c.blue])?;
    }

    return Ok({});
}

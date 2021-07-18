fn main() {
    const IMAGE_WIDTH: usize = 512 * 2; // * 2 because we display two screens
    const IMAGE_HEIGHT: usize = 512;

    let mut frame_buffer = [pack_color(ColorChannel {
        red: 255,
        green: 255,
        blue: 255,
        alpha: 0,
    }); IMAGE_WIDTH * IMAGE_HEIGHT];

    let map_w = 16;
    let map_h = 16;
    let map = "0000222222220000\
1              0\
1      1111111 0\
1     0        0\
0     0  1110000\
0     3        0\
0   10000      0\
0   0   11100  0\
0   0   0      0\
0   0   1  00000\
0       1      0\
2       1      0\
0       0      0\
0 0000000      0\
0              0\
0002222222200000"
        .as_bytes();

    let mut player = Player {
        x: 3.456,
        y: 2.345,
        view_angle: 1.523,
        fov: std::f64::consts::PI / 3 as f64,
    };

    // colors
    let colors = [
        pack_color(ColorChannel {
            red: 0,
            green: 48,
            blue: 73,
            alpha: 0,
        }),
        pack_color(ColorChannel {
            red: 214,
            green: 40,
            blue: 40,
            alpha: 0,
        }),
        pack_color(ColorChannel {
            red: 247,
            green: 127,
            blue: 0,
            alpha: 0,
        }),
        pack_color(ColorChannel {
            red: 252,
            green: 191,
            blue: 73,
            alpha: 0,
        }),
    ];

    // generate map
    let rect_w = IMAGE_WIDTH / (map_w * 2);
    let rect_h = IMAGE_HEIGHT / map_h;

    for frame in 0..360 {
        player.view_angle += 2.0 * std::f64::consts::PI / 360.0;

        // clear screen
        frame_buffer = [pack_color(ColorChannel {
            red: 255,
            green: 255,
            blue: 255,
            alpha: 0,
        }); IMAGE_WIDTH * IMAGE_HEIGHT];

        for j in 0..map_h {
            for i in 0..map_w {
                // ignore whitespaces
                if map[i + j * map_w] == 32u8 {
                    continue;
                };

                let rect_x = i * rect_w;
                let rect_y = j * rect_h;
                let c = ColorChannel {
                    red: 0,
                    green: 255,
                    blue: 255,
                    alpha: 0,
                };

                let color_index = ((map[i + j * map_w] - 0u8) as char).to_digit(10).unwrap();

                // print!("color index: {}\n", color_index);

                draw_rectangle(
                    &mut frame_buffer,
                    IMAGE_WIDTH,
                    IMAGE_HEIGHT,
                    rect_x,
                    rect_y,
                    rect_w,
                    rect_h,
                    colors[color_index as usize],
                );
            }
        }

        // draw visibility cone and the 3D view
        for j in 0..IMAGE_WIDTH / 2 {
            let angle = player.view_angle as f64 - (player.fov / 2.0)
                + player.fov as f64 * j as f64 / (IMAGE_WIDTH / 2) as f64;
            for i in 0..2000 {
                let t = i as f64 * 0.01;
                let cx = player.x as f64 + t * angle.cos();
                let cy = player.y as f64 + t * angle.sin();

                let pix_x: usize = (cx * rect_w as f64) as usize;
                let pix_y: usize = (cy * rect_h as f64) as usize;

                // draws the visibility cone
                frame_buffer[pix_x + (pix_y * IMAGE_WIDTH)] = pack_color(ColorChannel {
                    red: 160,
                    green: 160,
                    blue: 160,
                    alpha: 0,
                });

                // if the ray touches a wall, draw a vertical column
                if map[cx as usize + cy as usize * map_w] != 32u8 {
                    let col_height = IMAGE_HEIGHT as f64 / t;
                    let color_index = ((map[cx as usize + cy as usize * map_w] - 0u8) as char)
                        .to_digit(10)
                        .unwrap();

                    draw_rectangle(
                        &mut frame_buffer,
                        IMAGE_WIDTH,
                        IMAGE_HEIGHT,
                        IMAGE_WIDTH / 2 + j,
                        (IMAGE_HEIGHT as f64 / 2.0 - col_height / 2.0) as usize,
                        1,
                        col_height as usize,
                        colors[color_index as usize],
                    );
                    break;
                }
            }
        }

        let filename = format!("{:0>3}.ppm", frame);

        drop_ppm_image(
            &filename,
            &frame_buffer,
            IMAGE_WIDTH as i32,
            IMAGE_HEIGHT as i32,
        )
        .unwrap();
    }
}

// image

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
        red: ((color >> 0) & 255) as u8,
        green: ((color >> 8) & 255) as u8,
        blue: ((color >> 16) & 255) as u8,
        alpha: ((color >> 24) & 255) as u8,
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

// map

fn draw_rectangle(
    image: &mut [i32],
    img_w: usize,
    img_h: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    color: i32,
) {
    for i in 0..w {
        for j in 0..h {
            let cx = x + i;
            let cy = y + j;

            if cx >= img_w || cy >= img_h {
                continue;
            }

            image[cx + cy * img_w] = color;
        }
    }
}

// player

struct Player {
    x: f32,
    y: f32,
    view_angle: f64,
    fov: f64,
}

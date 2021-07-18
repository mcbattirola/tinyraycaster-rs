fn main() {
    const IMAGE_WIDTH: usize = 512;
    const IMAGE_HEIGHT: usize = 512;

    let mut frame_buffer = [255; IMAGE_WIDTH * IMAGE_HEIGHT];

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

    let player = Player {
        x: 3.456,
        y: 2.345,
        view_angle: 1.523,
    };

    // generate image
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

    // generate map
    let rect_w = IMAGE_WIDTH / map_w;
    let rect_h = IMAGE_HEIGHT / map_h;
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

            draw_rectangle(
                &mut frame_buffer,
                IMAGE_WIDTH,
                IMAGE_HEIGHT,
                rect_x,
                rect_y,
                rect_w,
                rect_h,
                pack_color(c),
            );
        }
    }

    // draw player
    let player_color = ColorChannel {
        red: 255,
        green: 255,
        blue: 255,
        alpha: 0,
    };

    draw_rectangle(
        &mut frame_buffer,
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        (player.x * (rect_w as f32)) as usize,
        (player.y * (rect_h as f32)) as usize,
        5,
        5,
        pack_color(player_color),
    );

    // cast ray from player
    for i in 0..400 {
        let t = i as f32 * 0.05;
        let cx = player.x + t * player.view_angle.cos();
        let cy = player.y + t * player.view_angle.sin();

        if map[cx as usize + cy as usize * map_w] != 32u8 {
            break;
        }

        let pix_x: usize = (cx * rect_w as f32) as usize;
        let pix_y: usize = (cy * rect_h as f32) as usize;

        frame_buffer[pix_x + (pix_y * IMAGE_WIDTH)] = pack_color(ColorChannel {
            red: 255,
            green: 255,
            blue: 255,
            alpha: 0,
        })
    }

    drop_ppm_image(
        "./out.ppm",
        &frame_buffer,
        IMAGE_WIDTH as i32,
        IMAGE_HEIGHT as i32,
    )
    .unwrap();
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
            image[cx + cy * img_w] = color;
        }
    }
}

// player

struct Player {
    x: f32,
    y: f32,
    view_angle: f32,
}

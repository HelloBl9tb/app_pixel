use image::*;
use kurbo::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Square {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    x4: f32,
    y4: f32,
}

pub fn generate_squares(step: f32, img: DynamicImage) -> Vec<Square> {
    let mut squares = Vec::new();
    let (width, height) = img.dimensions();

    let mut y = 0.0;
    while y < height as f32 {
        let mut x = 0.0;
        while x < width as f32 {
            let s = Square {
                x1: x,
                y1: y,
                x2: x + step,
                y2: y,
                x3: x + step,
                y3: y + step,
                x4: x,
                y4: y + step,
            };

            squares.push(s);

            x += step;
        }

        y += step;
    }

    squares
}

pub fn dominant_colors(img: DynamicImage) -> Vec<Rgba<u8>> {
    let (width, height) = img.dimensions();
    let mut vec_colors_coordinats = Vec::new();
    let cell_size = 10;

    for y in (0..height).step_by(cell_size) {
        for x in (0..width).step_by(cell_size) {
            let mut color_counts = std::collections::HashMap::new();

            for y2 in y..y + cell_size as u32 {
                for x2 in x..x + cell_size as u32 {
                    let pixel = img.get_pixel(x2, y2);
                    *color_counts.entry(pixel).or_insert(0) += 1;
                }
            }

            if let Some((color, _)) = color_counts.iter().max_by_key(|(_, count)| *count) {
                vec_colors_coordinats.push(*color);
            }
        }
    }

    vec_colors_coordinats
}

pub fn line(img: DynamicImage, cell_size: usize) -> Vec<kurbo::Line> {
    let (width, height) = img.dimensions();
    let mut vec_line = Vec::new();

    //vertical
    for i in (0..width).step_by(cell_size) {
        let line_1 = Line::new(
            Point::new(i as f64, 0.0),
            Point::new(i as f64, width as f64),
        );
        vec_line.push(line_1);
    }
    //horizontal
    for i in (0..height).step_by(cell_size) {
        let line_1 = Line::new(
            Point::new(0.0, i as f64),
            Point::new(height as f64, i as f64),
        );
        vec_line.push(line_1);
    }
    //last vertical
    for _i in (0..width).step_by(cell_size) {
        let line_1 = Line::new(
            Point::new(width as f64, 0.0),
            Point::new(width as f64, width as f64),
        );
        vec_line.push(line_1);
    }
    //last horizontal
    for _i in (0..height).step_by(cell_size) {
        let line_1 = Line::new(
            Point::new(0.0, height as f64),
            Point::new(height as f64, height as f64),
        );
        vec_line.push(line_1);
    }
    vec_line
}

pub fn paint_coordinats(
    square: Vec<Square>,
    color: Vec<Rgba<u8>>,
    img: DynamicImage,
    line: Vec<Line>,
) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut img_out = RgbaImage::new(width + 1, height + 1);

    //paint_colors
    for c in 0..square.len() {
        // println!("{:?}", square[c].x1);
        for y in square[c].y1 as u32..square[c].y3 as u32 {
            for x in square[c].x1 as u32..square[c].x3 as u32 {
                img_out.put_pixel(x, y, color[c]);
            }
        }
    }

    // paint black_line
    for i in 0..line.len() {
        for y in (line[i].p0.y as u32)..=(line[i].p1.y as u32) {
            for x in (line[i].p0.x as u32)..=(line[i].p1.x as u32) {
                img_out.put_pixel(x, y, Rgba([0, 0, 0, 255]));
            }
        }
    }

    DynamicImage::ImageRgba8(img_out)
}

pub fn format_color(
    sqauare: Vec<Square>,
    color: Vec<Rgba<u8>>,
    coordinats: String,
    format: String,
) -> String {
    let mut string = "".to_string();
    // Format string in Square
    let v: Vec<i32> = coordinats.split(',').map(|s| s.parse().unwrap()).collect();
    let sqauare_2 = Square {
        x1: v[0] as f32,
        y1: v[1] as f32,
        x2: v[2] as f32,
        y2: v[3] as f32,
        x3: v[4] as f32,
        y3: v[5] as f32,
        x4: v[6] as f32,
        y4: v[7] as f32,
    };

    let mut count = 0;
    for a in 0..sqauare.len() {
        if sqauare_2 == sqauare[a] {
            count += a as usize;
            println!("Match")
        }
    }

    // Format rgb in hex the coordinats
    if format == "rgb".to_string() {
        let color_1 = color[count].clone().to_rgb();
        string.push('R');
        string.push('g');
        string.push('b');
        string.push('(');
        string.push('[');
        for a in 0..3 {
            for b in (color_1[a].to_string()).chars() {
                string.push(b);
            }

            if a != 2 {
                string.push(',');
            }
        }
        string.push(')');
        string.push(']');
    } else if format == "hex".to_string() {
        string.push('#');
        for a in 0..3 {
            let hex = format!("{:x}", color[count][a]);
            for b in hex.to_uppercase().chars() {
                string.push(b);
            }
        }
    }

    string
}

pub fn decode_image(encoded_image: &str) -> Result<Vec<u8>, base64::DecodeError> {
    base64::decode(encoded_image)
}

use base64::{engine::general_purpose, Engine};
use image::{ImageBuffer, Rgb, RgbImage};
use rusttype::{Font, Point, Scale};
use std::io::Cursor;

const FONT_BYTES: &[u8] = &[];

pub struct TextOverlay<'a> {
    pub text: &'a str,
    pub x: i32,
    pub y: i32,
    pub font_size: f32,
    pub color: [u8; 3],
}

pub fn gen_image(texts: Vec<TextOverlay>) -> String {
    let (width, height) = (400, 300);
    let background_color = [255, 200, 200];
    let mut img: RgbImage = ImageBuffer::from_fn(width, height, |_, _| Rgb(background_color));
    let font = Font::try_from_bytes(FONT_BYTES).expect("Failed to load font");

    for text in texts.iter() {
        let scale = Scale::uniform(text.font_size);
        let _v_metrics = font.v_metrics(scale);

        // Draw each character separately
        let mut x_offset = text.x as f32;
        for (i, ch) in text.text.chars().enumerate() {
            let y_offset = text.y as f32 + (i as f32 * 5.0).sin() * 10.0; // Wave effect

            let glyph = font.glyph(ch).scaled(scale).positioned(Point {
                x: x_offset,
                y: y_offset,
            });

            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let x = x + bounding_box.min.x as u32;
                    let y = y + bounding_box.min.y as u32;
                    if x < width && y < height {
                        let pixel = img.get_pixel_mut(x, y);
                        let color_factor = (v * 255.0) as u8;
                        *pixel = Rgb([
                            ((255 - color_factor) as u32 * background_color[0] as u32 / 255
                                + color_factor as u32 * text.color[0] as u32 / 255)
                                as u8,
                            ((255 - color_factor) as u32 * background_color[1] as u32 / 255
                                + color_factor as u32 * text.color[1] as u32 / 255)
                                as u8,
                            ((255 - color_factor) as u32 * background_color[2] as u32 / 255
                                + color_factor as u32 * text.color[2] as u32 / 255)
                                as u8,
                        ]);
                    }
                });
            }

            x_offset += glyph.unpositioned().h_metrics().advance_width;
        }
    }

    // Encode the image to PNG format
    let mut output = Vec::new();
    img.write_to(&mut Cursor::new(&mut output), image::ImageOutputFormat::Png)
        .expect("Failed to encode image");
    let base64_image = general_purpose::STANDARD.encode(&output);

    // image to data uri format
    format!("data:image/png;base64,{}", base64_image)
}

/*
pub fn gen_image(TextOverlay { text }: TextOverlay) -> String {
    let rgb = [255, 200, 200]; // light pink
    let (width, height) = (400, 300);
    let mut rgb_image: RgbImage = ImageBuffer::from_fn(width, height, |_, _| Rgb(rgb));
    let font = Font::try_from_bytes(FONT_BYTES).expect("Failed to load font");
    // Draw each text overlay
    for (index, text) in text.chars().enumerate() {
        let scale = Scale::uniform(50);

        // calculating text dimensions
        let text_width = font
            .layout(&text, scale, Point { x: 0.0, y: 0.0 })
            .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
            .last()
            .unwrap_or(0.0);
    }
    /*
    for (index, text) in text.iter().enumerate() {
        let scale = Scale::uniform(text.font_size);

        // Calculate text dimensions
        let text_width = font
            .layout(&text.text, scale, Point { x: 0.0, y: 0.0 })
            .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
            .last()
            .unwrap_or(0.0);

        let v_metrics = font.v_metrics(scale);
        let text_height = v_metrics.ascent - v_metrics.descent;

        // Calculate centered position
        let x = ((width as f32 - text_width) / 2.0).floor() as i32;
        let y = ((height as f32 - (texts.len() as f32 * text_height)) / 2.0
            + (index as f32 * text_height))
            .floor() as i32;

        draw_text_mut(&mut img, Rgb(text.color), x, y, scale, &font, &text.text);
    }

    // Encode the image to PNG format
    let mut output = Vec::new();
    img.write_to(&mut Cursor::new(&mut output), image::ImageOutputFormat::Png)
        .expect("Failed to encode image");

    let base64_image = general_purpose::STANDARD_NO_PAD.encode(&output);

    // image to data uri format
    format!("data:image/png;base64,{}", base64_image)
    */
    todo!()
}
*/

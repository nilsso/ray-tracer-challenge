use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::Color;

#[derive(Debug)]
pub enum CanvasError {
    InvalidIndex,
}

/// Rectangular grid of pixels
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self::with_color(width, height, Default::default())
    }

    pub fn with_color(width: usize, height: usize, color: Color) -> Self {
        Self {
            width,
            height,
            pixels: vec![color; width * height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn pixel_index(&self, x: usize, y: usize) -> Result<usize, CanvasError> {
        let i = x + y * self.width;

        if i < self.pixels.len() {
            Ok(i)
        } else {
            Err(CanvasError::InvalidIndex)
        }
    }

    pub fn pixels(&self) -> &Vec<Color> {
        &self.pixels
    }

    pub fn pixels_mut(&mut self) -> &mut Vec<Color> {
        &mut self.pixels
    }

    pub fn pixel(&self, x: usize, y: usize) -> Result<&Color, CanvasError> {
        let i = self.pixel_index(x, y)?;

        Ok(&self.pixels[i])
    }

    pub fn pixel_mut(&mut self, x: usize, y: usize) -> Result<&mut Color, CanvasError> {
        let i = self.pixel_index(x, y)?;

        Ok(&mut self.pixels[i])
    }

    pub fn write_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut f = File::create(path)?;

        writeln!(f, "P3\n{} {}\n255", self.width, self.height)?;
        for p in self.pixels.iter() {
            writeln!(f, "{}", p)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Canvas, BLACK, WHITE};

    #[test]
    fn new_canvas_is_all_black() {
        let canvas = Canvas::new(10, 20);

        assert_eq!(10, canvas.width());
        assert_eq!(20, canvas.height());
        assert!(canvas.pixels().iter().all(|&p| p == BLACK));
    }

    #[test]
    fn write_single_pixel() {
        let mut canvas = Canvas::new(10, 20);

        let pixel = canvas.pixel_mut(0, 0).unwrap();

        *pixel = WHITE;

        assert!(canvas.pixels()[1..].iter().all(|&p| p == BLACK));
        assert!(*canvas.pixel(0, 0).unwrap() == WHITE);
    }
}

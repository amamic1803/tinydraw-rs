//! Indexing functions for the [Image] struct.

use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use crate::colors::{Color, ColorType};
use crate::error::Error;
use crate::image::Image;

/// Trait for specifying the part of the image in set functions.
pub trait ImageSetIndex {
    /// Returns the start index of the range (inclusive).
    fn start(&self) -> usize;
    /// Returns the end index of the range (exclusive). If there is no end, returns [None].
    fn end(&self) -> Option<usize>;
}
impl ImageSetIndex for usize {
    fn start(&self) -> usize {
        *self
    }
    fn end(&self) -> Option<usize> {
        Some(*self + 1)
    }
}
impl ImageSetIndex for Range<usize> {
    fn start(&self) -> usize {
        self.start
    }
    fn end(&self) -> Option<usize> {
        Some(self.end)
    }
}
impl ImageSetIndex for RangeFrom<usize> {
    fn start(&self) -> usize {
        self.start
    }
    fn end(&self) -> Option<usize> {
        None
    }
}
impl ImageSetIndex for RangeFull {
    fn start(&self) -> usize {
        0
    }
    fn end(&self) -> Option<usize> {
        None
    }
}
impl ImageSetIndex for RangeInclusive<usize> {
    fn start(&self) -> usize {
        *self.start()
    }
    fn end(&self) -> Option<usize> {
        Some(*self.end() + 1)
    }
}
impl ImageSetIndex for RangeTo<usize> {
    fn start(&self) -> usize {
        0
    }
    fn end(&self) -> Option<usize> {
        Some(self.end)
    }
}
impl ImageSetIndex for RangeToInclusive<usize> {
    fn start(&self) -> usize {
        0
    }
    fn end(&self) -> Option<usize> {
        Some(self.end + 1)
    }
}

impl Image {
    /// Returns the index of the first byte of the pixel at the given coordinates.
    /// # Arguments
    /// * ```index``` - The tuple with the coordinates of the pixel (x, y).
    /// # Returns
    /// * [Result] which holds the index of the first byte of the pixel or [Err] with [Error].
    /// # Errors
    /// * [Error::IndexOutOfBounds] - If the index is out of bounds.
    #[inline]
    pub fn index(&self, index: (usize, usize)) -> Result<usize, Error> {
        if index.0 >= self.width || index.1 >= self.height {
            Err(Error::IndexOutOfBounds)
        } else {
            Ok(self.index_unchecked(index))
        }
    }

    /// Returns the index of the first byte of the pixel at the given coordinates without performing checks.
    /// # Arguments
    /// * ```index``` - The tuple with the coordinates of the pixel (x, y).
    /// # Returns
    /// * The index of the first byte of the pixel.
    #[inline]
    pub fn index_unchecked(&self, index: (usize, usize)) -> usize {
        ((self.height - index.1 - 1) * self.width + index.0) * self.color_type.bytes_per_pixel()
    }

    /// Returns the value of the pixel at the given coordinates.
    /// # Arguments
    /// * ```index``` - The tuple with the coordinates of the pixel (x, y).
    /// # Returns
    /// * [Result] which holds the value of the pixel or [Err] with [Error].
    /// # Errors
    /// * [Error::IndexOutOfBounds] - If the index is out of bounds.
    #[inline]
    pub fn get(&self, index: (usize, usize)) -> Result<Color, Error> {
        if index.0 >= self.width || index.1 >= self.height {
            Err(Error::IndexOutOfBounds)
        } else {
            Ok(self.get_unchecked(index))
        }
    }

    /// Returns the value of the pixel at the given coordinates without performing checks.
    /// # Arguments
    /// * ```index``` - The tuple with the coordinates of the pixel (x, y).
    /// # Returns
    /// * The value of the pixel.
    pub fn get_unchecked(&self, index: (usize, usize)) -> Color {
        let index_temp = self.index_unchecked(index);
        Color::from_bytes(self.color_type, &self.data[index_temp..index_temp + self.color_type.bytes_per_pixel()])
    }

    /// Sets the color of the pixels at the given locations.
    /// # Arguments
    /// * ```index``` - The tuple with the coordinates ```(x, y)```. The coordinates can be ```usize```, ```Range<usize>```, ```RangeFrom<usize>```, ```RangeFull```, ```RangeInclusive<usize>```, ```RangeTo<usize>```, ```RangeToInclusive<usize>```.
    /// * ```color``` - The color to set.
    /// # Returns
    /// * [Result] which holds [Ok] or [Err] with [Error].
    /// # Errors
    /// * [Error::IndexOutOfBounds] - If the index is out of bounds.
    /// * [Error::WrongColor] - If the color type of the image does not match the color type of the color.
    #[inline]
    pub fn set<RX: ImageSetIndex, RY: ImageSetIndex>(&mut self, index: (RX, RY), color: Color) -> Result<(), Error> {
        if index.0.start() >= self.width || index.1.start() >= self.height {
            return Err(Error::IndexOutOfBounds);
        } else if let Some(x) = index.0.end() {
            if x > self.width {
                return Err(Error::IndexOutOfBounds);
            }
        } else if let Some(y) = index.1.end() {
            if y > self.height {
                return Err(Error::IndexOutOfBounds);
            }
        } else if ColorType::from(color) != self.color_type {
            return Err(Error::WrongColor);
        }

        self.set_unchecked(index, color);

        Ok(())
    }

    /// Sets the color of the pixels at the given locations without performing checks.
    /// # Arguments
    /// * ```index``` - The tuple with the coordinates ```(x, y)```. The coordinates can be ```usize```, ```Range<usize>```, ```RangeFrom<usize>```, ```RangeFull```, ```RangeInclusive<usize>```, ```RangeTo<usize>```, ```RangeToInclusive<usize>```.
    /// * ```color``` - The color to set.
    pub fn set_unchecked<RX: ImageSetIndex, RY: ImageSetIndex>(&mut self, index: (RX, RY), color: Color) {
        let x_index_low = index.0.start();  // inclusive
        let x_index_high = match index.0.end() {  // exclusive
            Some(x) => x,
            None => self.width,
        };
        let y_index_low = index.1.start();  // inclusive
        let y_index_high = match index.1.end() {  // exclusive
            Some(y) => y,
            None => self.height,
        };
        
        let bytes = color.as_bytes();
        let x_offset = (x_index_high - x_index_low) * bytes.len();
        for y in y_index_low..y_index_high {
            let index_low = self.index_unchecked((x_index_low, y));
            let index_high = index_low + x_offset;
            for x in (index_low..index_high).step_by(bytes.len()) {
                self.data[x..x + bytes.len()].copy_from_slice(bytes);
            }
        }
    }

    /// Sets the value of the pixels at the given locations by blending the color with the current value at the given coordinates.
    /// # Arguments
    /// * ```index``` - The tuple with the coordinates ```(x, y)```. The coordinates can be ```usize```, ```Range<usize>```, ```RangeFrom<usize>```, ```RangeFull```, ```RangeInclusive<usize>```, ```RangeTo<usize>```, ```RangeToInclusive<usize>```.
    /// * ```color``` - The color to set.
    /// * ```opacity``` - The opacity for blending.
    /// # Returns
    /// * [Result] which holds [Ok] or [Err] with [Error].
    /// # Errors
    /// * [Error::IndexOutOfBounds] - If the index is out of bounds.
    /// * [Error::WrongColor] - If the color type of the image does not match the color type of the color.
    /// * [Error::InvalidOpacity] - If the opacity is NaN or not in the range [0.0, 1.0].
    #[inline]
    pub fn set_transparent<RX: ImageSetIndex, RY: ImageSetIndex>(&mut self, index: (RX, RY), color: Color, opacity: f64) -> Result<(), Error> {
        if index.0.start() >= self.width || index.1.start() >= self.height {
            return Err(Error::IndexOutOfBounds);
        }
        if let Some(x) = index.0.end() {
            if x > self.width {
                return Err(Error::IndexOutOfBounds);
            }
        }
        if let Some(y) = index.1.end() {
            if y > self.height {
                return Err(Error::IndexOutOfBounds);
            }
        }
        if ColorType::from(color) != self.color_type {
            return Err(Error::WrongColor);
        }
        if opacity.is_nan() || !(0.0..=1.0).contains(&opacity) {
            return Err(Error::InvalidOpacity);
        }

        self.set_transparent_unchecked(index, color, opacity);
        
        Ok(())
    }

    /// Sets the value of the pixels at the given locations by blending the color with the current value at the given coordinates without performing checks.
    /// # Arguments
    /// * ```index``` - The tuple with the coordinates ```(x, y)```. The coordinates can be ```usize```, ```Range<usize>```, ```RangeFrom<usize>```, ```RangeFull```, ```RangeInclusive<usize>```, ```RangeTo<usize>```, ```RangeToInclusive<usize>```.
    /// * ```color``` - The color to set.
    /// * ```opacity``` - The opacity for blending.
    pub fn set_transparent_unchecked<RX: ImageSetIndex, RY: ImageSetIndex>(&mut self, index: (RX, RY), color: Color, opacity: f64) {
        let x_index_low = index.0.start();  // inclusive
        let x_index_high = match index.0.end() {  // exclusive
            Some(x) => x,
            None => self.width,
        };
        let y_index_low = index.1.start();  // inclusive
        let y_index_high = match index.1.end() {  // exclusive
            Some(y) => y,
            None => self.height,
        };

        // background color aware
        // ===> color = color + (new_color - color) * color_percentage
        // ===> color = color * (1 - color_percentage) + new_color * color_percentage
        
        let bytes = color.as_bytes();
        let x_offset = (x_index_high - x_index_low) * bytes.len();
        for y in y_index_low..y_index_high {
            let index_low = self.index_unchecked((x_index_low, y));
            let index_high = index_low + x_offset;
            for x in (index_low..index_high).step_by(bytes.len()) {
                let current_color = Color::from_bytes(self.color_type, &self.data[x..x + bytes.len()]);
                let new_color = match (current_color, color) {
                    (Color::GRAY8(curr_val), Color::GRAY8(new_val)) => {
                        Color::GRAY8((curr_val as f64 * (1.0 - opacity) + new_val as f64 * opacity).round() as u8)
                    }
                    (Color::GRAYA8(curr_val), Color::GRAYA8(new_val)) => {
                        Color::GRAYA8([
                            (curr_val[0] as f64 * (1.0 - opacity) + new_val[0] as f64 * opacity).round() as u8,
                            (curr_val[1] as f64 * (1.0 - opacity) + new_val[1] as f64 * opacity).round() as u8,
                        ])
                    }
                    (Color::GRAY16(curr_val), Color::GRAY16(new_val)) => {
                        Color::GRAY16((curr_val as f64 * (1.0 - opacity) + new_val as f64 * opacity).round() as u16)
                    }
                    (Color::GRAYA16(curr_val), Color::GRAYA16(new_val)) => {
                        Color::GRAYA16([
                            (curr_val[0] as f64 * (1.0 - opacity) + new_val[0] as f64 * opacity).round() as u16,
                            (curr_val[1] as f64 * (1.0 - opacity) + new_val[1] as f64 * opacity).round() as u16,
                        ])
                    }
                    (Color::RGB8(curr_val), Color::RGB8(new_val)) => {
                        Color::RGB8([
                            (curr_val[0] as f64 * (1.0 - opacity) + new_val[0] as f64 * opacity).round() as u8,
                            (curr_val[1] as f64 * (1.0 - opacity) + new_val[1] as f64 * opacity).round() as u8,
                            (curr_val[2] as f64 * (1.0 - opacity) + new_val[2] as f64 * opacity).round() as u8,
                        ])
                    }
                    (Color::RGBA8(curr_val), Color::RGBA8(new_val)) => {
                        Color::RGBA8([
                            (curr_val[0] as f64 * (1.0 - opacity) + new_val[0] as f64 * opacity).round() as u8,
                            (curr_val[1] as f64 * (1.0 - opacity) + new_val[1] as f64 * opacity).round() as u8,
                            (curr_val[2] as f64 * (1.0 - opacity) + new_val[2] as f64 * opacity).round() as u8,
                            (curr_val[3] as f64 * (1.0 - opacity) + new_val[3] as f64 * opacity).round() as u8,
                        ])
                    }
                    (Color::RGB16(curr_val), Color::RGB16(new_val)) => {
                        Color::RGB16([
                            (curr_val[0] as f64 * (1.0 - opacity) + new_val[0] as f64 * opacity).round() as u16,
                            (curr_val[1] as f64 * (1.0 - opacity) + new_val[1] as f64 * opacity).round() as u16,
                            (curr_val[2] as f64 * (1.0 - opacity) + new_val[2] as f64 * opacity).round() as u16,
                        ])
                    }
                    (Color::RGBA16(curr_val), Color::RGBA16(new_val)) => {
                        Color::RGBA16([
                            (curr_val[0] as f64 * (1.0 - opacity) + new_val[0] as f64 * opacity).round() as u16,
                            (curr_val[1] as f64 * (1.0 - opacity) + new_val[1] as f64 * opacity).round() as u16,
                            (curr_val[2] as f64 * (1.0 - opacity) + new_val[2] as f64 * opacity).round() as u16,
                            (curr_val[3] as f64 * (1.0 - opacity) + new_val[3] as f64 * opacity).round() as u16,
                        ])
                    }
                    _ => panic!("color must match the color type of the image"),
                };
                self.data[x..x + bytes.len()].copy_from_slice(new_color.as_bytes());
            }
        }
    }
}

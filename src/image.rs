//! A module that contains the [Image] struct and related functions.

// standard library imports
use crate::colors::{Color, ColorType};
use crate::error::Error;
use std::{
    cmp::{max, min},
    f64::consts::FRAC_1_SQRT_2,
    fmt::Display,
};

/// A struct that holds an image
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Image {
    /// The image pixel data
    pub(crate) data: Vec<u8>,
    /// The width of the image
    pub(crate) width: usize,
    /// The height of the image
    pub(crate) height: usize,
    /// The color type of the image
    pub(crate) color_type: ColorType,
    /// The background color of the image, None if not set
    pub(crate) background_color: Option<Color>,
}
impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Image:\n   - dimensions: {}x{}\n   - color type: {}   - size: {} bytes",
            self.width,
            self.height,
            self.color_type,
            self.data.len()
        )
    }
}

/// A trait for drawing on images.
pub trait Drawing {
    /// Draws a circle on the image. If the circle is not fully contained in the image, it will be clipped.
    /// # Arguments
    /// * ```center``` - The coordinates of the center of the circle.
    /// * ```radius``` - The radius of the circle.
    /// * ```color``` - The color of the circle.
    /// * ```thickness``` - The thickness of the circle. If the thickness is 0, the circle will be filled.
    /// * ```opacity``` - The opacity of the circle.
    fn draw_circle(&mut self, center: (usize, usize), radius: usize, color: Color, thickness: usize, opacity: f64) -> Result<(), Error>;

    /// Draws an ellipse on the image. If the ellipse is not fully contained in the image, it will be clipped.
    /// # Arguments
    /// * ```center``` - The coordinates of the center of the ellipse.
    /// * ```axes``` - The lengths of the axes of the ellipse.
    /// * ```color``` - The color of the ellipse.
    /// * ```thickness``` - The thickness of the ellipse. If the thickness is 0, the ellipse will be filled.
    /// * ```opacity``` - The opacity of the ellipse.
    fn draw_ellipse(&mut self, center: (usize, usize), axes: (usize, usize), color: Color, thickness: usize, opacity: f64) -> Result<(), Error>;

    /// Draws a line on the image. If the line is not fully contained in the image, it will be clipped.
    /// # Arguments
    /// * ```point1``` - The coordinates of the first point of the line.
    /// * ```point2``` - The coordinates of the second point of the line.
    /// * ```color``` - The color of the line.
    /// * ```thickness``` - The thickness of the line.
    /// * ```opacity``` - The opacity of the line.
    fn draw_line(&mut self, point1: (usize, usize), point2: (usize, usize), color: Color, thickness: usize, opacity: f64) -> Result<(), Error>;

    /// Draws a rectangle on the image. If the rectangle is not fully contained in the image, it will be clipped.
    /// # Arguments
    /// * ```point1``` - The coordinates of the first point of the rectangle.
    /// * ```point2``` - The coordinates of the second point of the rectangle.
    /// * ```color``` - The color of the rectangle.
    /// * ```thickness``` - The thickness of the rectangle. If the thickness is 0, the rectangle will be filled.
    /// * ```opacity``` - The opacity of the rectangle.
    fn draw_rectangle(&mut self, point1: (usize, usize), point2: (usize, usize), color: Color, thickness: usize, opacity: f64) -> Result<(), Error>;
}

/*
impl Image<[u8; 3]> {
    #[allow(clippy::too_many_arguments)]
    pub fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: [u8; 3], thickness: usize, opacity: f64) {
        //! Draws a new line. `x1`, `y1` are coordinates of the starting point. `x2`, `y2` are coordinates of the ending point.
        //! `color` defines the color of the line.
        //! `thickness` defines how thick the line will be. (currently doesn't do anything). If set to 0, nothing will be drawn.
        //! `opacity` sets the transparency of the line. `<= 0.0` means the line will be completely transparent, while `>= 1.0` means the line won't be transparent.

        if (thickness != 0) && (opacity >= 0.0) {
            if (x1 == x2) && (x1 < self.width) {
                // if line is vertical just draw it
                let lower_y: usize = if min(y1, y2) >= self.height {
                    self.height - 1
                } else {
                    min(y1, y2)
                };
                let upper_y: usize = if max(y1, y2) >= self.height {
                    self.height - 1
                } else {
                    max(y1, y2)
                };
                if opacity >= 1.0 {
                    for y in lower_y..(upper_y + 1) {
                        self.data[self.width * (self.height - 1 - y) + x1] = color;
                    }
                } else {
                    for y in lower_y..(upper_y + 1) {
                        for channel in 0..color.len() {
                            self.data[self.width * (self.height - 1 - y) + x1][channel] = ((self.data[self.width * (self.height - 1 - y) + x1][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                        }
                    }
                }
            } else if (y1 == y2) && (y1 < self.height) {
                // if line is horizontal, just draw it
                let lower_x: usize = if min(x1, x2) >= self.height {
                    self.height - 1
                } else {
                    min(x1, x2)
                };
                let upper_x: usize = if max(x1, x2) >= self.height {
                    self.height - 1
                } else {
                    max(x1, x2)
                };
                let row: usize = self.width * (self.height - 1 - y1);
                if opacity >= 1.0 {
                    self.data[(row + lower_x)..(row + upper_x + 1)].fill(color);
                } else {
                    for ind in (row + lower_x)..(row + upper_x + 1) {
                        for channel in 0..color.len() {
                            self.data[ind][channel] = ((self.data[ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                        }
                    }
                }
            } else {
                // line is diagonal here
                let slope: f64 = ((y1 as f64) - (y2 as f64)) / ((x1 as f64) - (x2 as f64));
                let mut x1_calc: usize = x1;
                let mut x2_calc: usize = x2;
                let mut y1_calc: usize = y1;
                let mut y2_calc: usize = y2;
                if (x1 >= self.width) && (y1 >= self.height) {
                    let y_temp: usize = self.height - 1;
                    let x_temp: usize = ((((y_temp as f64) - (y1 as f64)) / slope) + (x1 as f64)).floor() as usize;
                    if x_temp >= self.width {
                        let x_temp: usize = self.width - 1;
                        let y_temp: usize = (slope * ((x_temp as f64) - (x1 as f64)) + (y1 as f64)).floor() as usize;
                        if y_temp >= self.height {
                            return
                        } else {
                            x1_calc = x_temp;
                            y1_calc = y_temp;
                        }
                    } else {
                        x1_calc = x_temp;
                        y1_calc = y_temp;
                    }
                } else if x1 >= self.width {
                    let x_temp: usize = self.width - 1;
                    let y_temp: usize = (slope * ((x_temp as f64) - (x1 as f64)) + (y1 as f64)).floor() as usize;
                    if y_temp >= self.height {
                        return
                    } else {
                        x1_calc = x_temp;
                        y1_calc = y_temp;
                    }
                } else if y1 >= self.height {
                    let y_temp: usize = self.height - 1;
                    let x_temp: usize = ((((y_temp as f64) - (y1 as f64)) / slope) + (x1 as f64)).floor() as usize;
                    if x_temp >= self.width {
                        return
                    } else {
                        x1_calc = x_temp;
                        y1_calc = y_temp;
                    }
                }
                if (x2 >= self.width) && (y2 >= self.height) {
                    let y_temp: usize = self.height - 1;
                    let x_temp: usize = ((((y_temp as f64) - (y1 as f64)) / slope) + (x1 as f64)).floor() as usize;
                    if x_temp >= self.width {
                        let x_temp: usize = self.width - 1;
                        let y_temp: usize = (slope * ((x_temp as f64) - (x1 as f64)) + (y1 as f64)).floor() as usize;
                        if y_temp >= self.height {
                            return
                        } else {
                            x2_calc = x_temp;
                            y2_calc = y_temp;
                        }
                    } else {
                        x2_calc = x_temp;
                        y2_calc = y_temp;
                    }
                } else if x2 >= self.width {
                    let x_temp: usize = self.width - 1;
                    let y_temp: usize = (slope * ((x_temp as f64) - (x1 as f64)) + (y1 as f64)).floor() as usize;
                    if y_temp >= self.height {
                        return
                    } else {
                        x2_calc = x_temp;
                        y2_calc = y_temp;
                    }
                } else if y2 >= self.height {
                    let y_temp: usize = self.height - 1;
                    let x_temp: usize = ((((y_temp as f64) - (y1 as f64)) / slope) + (x1 as f64)).floor() as usize;
                    if x_temp >= self.width {
                        return
                    } else {
                        x2_calc = x_temp;
                        y2_calc = y_temp;
                    }
                }
                if (x1_calc == x2_calc) && (y1_calc == y2_calc) {
                    return
                }
                // if line has slope use Xiaolin Wu's algorithm to draw it anti aliased
                // if slope is more horizontal (<= 1), antialiasing with pixels above and below
                // if slope is more vertical (> 1), antialiasing with pixels left and right
                if slope.abs() <= 1.0 {
                    for x in min(x1_calc, x2_calc)..(max(x1_calc, x2_calc) + 1) {
                        let y: f64 = slope * ((x - x1) as f64) + (y1 as f64);
                        if (y - y.round()).abs() < 0.00001 {
                            // if point is very close to integer, just draw it on that pixel
                            if opacity >= 1.0 {
                                self.data[self.width * (self.height - 1 - (y.round() as usize)) + x] = color;
                            } else {
                                let ind: usize = self.width * (self.height - 1 - (y.round() as usize)) + x;
                                for channel in 0..color.len() {
                                    self.data[ind][channel] = ((self.data[ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                }
                            }
                        } else {
                            // split point between two pixels
                            let mut pix1_percentage: f64 = y - y.floor();
                            let mut pix2_percentage: f64 = 1.0 - pix1_percentage;
                            if opacity < 1.0 {
                                pix1_percentage *= opacity;
                                pix2_percentage *= opacity
                            }
                            let pix1_ind: usize = self.width * (self.height - 1 - (y.ceil() as usize)) + x;
                            let pix2_ind: usize = pix1_ind + self.width;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.data[pix1_ind][channel] = ((self.data[pix1_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[pix2_ind][channel] = ((self.data[pix2_ind][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;
                            }
                        }
                    }
                } else {
                    for y in min(y1_calc, y2_calc)..(max(y1_calc, y2_calc) + 1) {
                        let x: f64 = (((y - y1) as f64) / slope) + (x1 as f64);
                        if (x - x.round()).abs() < 0.00001 {
                            // if point is very close to integer, just draw it on that pixel
                            if opacity >= 1.0 {
                                self.data[self.width * (self.height - 1 - y) + (x.round() as usize)] = color;
                            } else {
                                let ind: usize = self.width * (self.height - 1 - y) + (x.round() as usize);
                                for channel in 0..color.len() {
                                    self.data[ind][channel] = ((self.data[ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                }
                            }
                        } else {
                            // split point between two pixels
                            let mut pix1_percentage: f64 = x.ceil() - x;
                            let mut pix2_percentage: f64 = 1.0 - pix1_percentage;
                            if opacity < 1.0 {
                                pix1_percentage *= opacity;
                                pix2_percentage *= opacity
                            }
                            let pix1_ind: usize = self.width * (self.height - 1 - y) + (x.floor() as usize);
                            let pix2_ind: usize = pix1_ind + 1;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.data[pix1_ind][channel] = ((self.data[pix1_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[pix2_ind][channel] = ((self.data[pix2_ind][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;
                            }
                        }
                    }
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::collapsible_else_if)]
    pub fn draw_rectangle(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, color: [u8; 3], thickness: usize, opacity: f64) {
        //! Draws a new rectangle. `x1`, `y1` are the coordinates of the first corner, and `x2`, `y2` are the coordinates of the opposite corner.
        //! `color` defines the color of the rectangle.
        //! `thickness` defines how thick the rectangle will be. (thickness is added to the inside of the rectangle). If set to 0, the rectangle will be filled.
        //! `opacity` sets the transparency of the rectangle. `<= 0.0` means the rectangle will be completely transparent, while `>= 1.0` means the rectangle won't be transparent.


    }

    #[allow(clippy::collapsible_else_if)]
    pub fn draw_circle(&mut self, x: usize, y: usize, radius: usize, color: [u8; 3], thickness: usize, opacity: f64) {
        //! Draws a new circle. `x`, `y` are the coordinates of the center of the circle.
        //! `radius` defines the radius of the circle.
        //! `color` defines the color of the circle.
        //! `thickness` defines how thick the circle will be. (currently doesn't do anything). If set to 0, the circle will be filled.
        //! `opacity` sets the transparency of the circle.
        //! `<= 0.0` means the circle will be completely transparent, while `>= 1.0` means the circle won't be transparent.

        if (radius > 0) && (opacity >= 0.0) && (x < self.width) && (y < self.height) && (x >= radius) && (y >= radius) && (x + radius < self.width) && (y + radius < self.height) {
            let x0: f64 = x as f64;
            let y0: f64 = y as f64;
            let radius_sqrd: f64 = radius.pow(2) as f64;

            let x_upperlimit: usize = x + ((((radius as f64) * FRAC_1_SQRT_2).round()) as usize);  // x up to which draw adjacent pixels vertically
            let mut y_upperlimit: usize = 0;  // y up to which draw adjacent pixels horizontally (is initialized to 0, but is changed later)

            if thickness == 0 {
                if opacity >= 1.0 {
                    // DRAW FILLED SOLID CIRCLE

                    // for every x, calculate y and draw outer pixel, connecting inner pixels with solid line (for filling circle)
                    let mut previous_y: usize = y + radius;
                    for x_coord in x..(x_upperlimit + 1) {
                        let y_coord: f64 = y0 + (radius_sqrd - (x_coord as f64 - x0).powi(2)).sqrt();

                        if x_coord == x_upperlimit {
                            y_upperlimit = y_coord.floor() as usize;
                        }

                        if (y_coord - y_coord.round()).abs() < 0.00001 {
                            // if point is very close to integer, just draw it on that pixel, and mirror to other 3 symmetric pixels
                            if x_coord == x {
                                self.data[self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord] = color;
                                self.data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord] = color;
                            } else if y_coord.round() as usize > y_upperlimit {
                                self.data[(self.width * (self.height - 1 - (y_coord.round() as usize)) + (2 * x - x_coord))..(self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord + 1)].fill(color);
                                self.data[(self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + (2 * x - x_coord))..(self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord + 1)].fill(color);
                                previous_y = y_coord.round() as usize;
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = y_coord - y_coord.floor();
                            let pix2_percentage: f64 = 1.0 - pix1_percentage;
                            let pix1_ind: usize = self.width * (self.height - 1 - (y_coord.ceil() as usize)) + x_coord;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.data[pix1_ind][channel] = ((self.data[pix1_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.data[pix1_ind - 2 * (x_coord - x)][channel] = ((self.data[pix1_ind - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] = ((self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                            }
                            if (y_coord.ceil() as usize) < previous_y {
                                self.data[(pix1_ind - 2 * (x_coord - x) + 1)..pix1_ind].fill(color);
                                self.data[(pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x) + 1)..(pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width)].fill(color);
                                previous_y = y_coord.ceil() as usize
                            }
                        }
                    }

                    // for every y, calculate x and draw outer pixel, connecting inner pixels with solid line (for filling circle)
                    for y_coord in y..(y_upperlimit + 1) {
                        let x_coord: f64 = x0 + (radius_sqrd - (y_coord as f64 - y0).powi(2)).sqrt();

                        if (x_coord - x_coord.round()).abs() < 0.00001 {
                            // if point is very close to integer, just draw it on that pixel, and mirror to other 3 symmetric pixels
                            if y_coord == y {
                                self.data[(self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize))..(self.width * (self.height - 1 - y_coord) + x_coord.round() as usize + 1)].fill(color);
                            } else {
                                self.data[(self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize))..(self.width * (self.height - 1 - y_coord) + (x_coord.round() as usize) + 1)].fill(color);
                                self.data[(self.width * (self.height - 1 - (2 * y - y_coord)) + (2 * x - x_coord.round() as usize))..(self.width * (self.height - 1 - (2 * y - y_coord)) + (x_coord.round() as usize) + 1)].fill(color);
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = x_coord - x_coord.floor();
                            let pix2_percentage: f64 = 1.0 - pix1_percentage;
                            let pix_ind: usize = self.width * (self.height - 1 - y_coord) + x_coord.ceil() as usize;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.data[pix_ind][channel] = ((self.data[pix_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                temp_ind = pix_ind + 2 * (y_coord - y) * self.width;
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                temp_ind -= 2 * (x_coord.ceil() as usize - x);
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                            }
                            self.data[(pix_ind - 2 * (x_coord.ceil() as usize - x) + 1)..pix_ind].fill(color);
                            self.data[(pix_ind + 2 * (y_coord - y) * self.width - 2 * (x_coord.ceil() as usize - x) + 1)..(pix_ind + 2 * (y_coord - y) * self.width)].fill(color);
                        }
                    }

                } else {
                    // DRAW FILLED TRANSPARENT CIRCLE

                    // for every x, calculate y and draw outer pixel, connecting inner pixels with solid line (for filling circle)
                    let mut previous_y: usize = y + radius;
                    for x_coord in x..(x_upperlimit + 1) {
                        let y_coord: f64 = y0 + (radius_sqrd - (x_coord as f64 - x0).powi(2)).sqrt();

                        if x_coord == x_upperlimit {
                            y_upperlimit = y_coord.floor() as usize;
                        }

                        if (y_coord - y_coord.round()).abs() < 0.00001 {
                            // if point is very close to integer, just draw it on that pixel, and mirror to other 3 symmetric pixels
                            if x_coord == x {
                                for channel in 0..color.len() {
                                    self.data[self.width * (self.height - 1 - y_coord.round() as usize) + x_coord][channel] = ((self.data[self.width * (self.height - 1 - y_coord.round() as usize) + x_coord][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    self.data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord][channel] = ((self.data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                }
                            } else if y_coord.round() as usize > y_upperlimit {
                                for pixel_ind in (self.width * (self.height - 1 - (y_coord.round() as usize)) + (2 * x - x_coord))..(self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord + 1) {
                                    for channel in 0..color.len() {
                                        self.data[pixel_ind][channel] = ((self.data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    }
                                }
                                for pixel_ind in (self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + (2 * x - x_coord))..(self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord + 1) {
                                    for channel in 0..color.len() {
                                        self.data[pixel_ind][channel] = ((self.data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    }
                                }
                                previous_y = y_coord.round() as usize;
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = (y_coord - y_coord.floor()) * opacity;
                            let pix2_percentage: f64 = 1.0 - pix1_percentage;
                            let pix1_ind: usize = self.width * (self.height - 1 - (y_coord.ceil() as usize)) + x_coord;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.data[pix1_ind][channel] = ((self.data[pix1_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.data[pix1_ind - 2 * (x_coord - x)][channel] = ((self.data[pix1_ind - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] = ((self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                            }
                            if (y_coord.ceil() as usize) < previous_y {
                                for pixel_ind in (pix1_ind - 2 * (x_coord - x) + 1)..pix1_ind {
                                    for channel in 0..color.len() {
                                        self.data[pixel_ind][channel] = ((self.data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    }
                                }
                                for pixel_ind in (pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x) + 1)..(pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width) {
                                    for channel in 0..color.len() {
                                        self.data[pixel_ind][channel] = ((self.data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    }
                                }
                                previous_y = y_coord.ceil() as usize
                            }
                        }
                    }

                    // for every y, calculate x and draw outer pixel, connecting inner pixels with solid line (for filling circle)
                    for y_coord in y..(y_upperlimit + 1) {
                        let x_coord: f64 = x0 + (radius_sqrd - (y_coord as f64 - y0).powi(2)).sqrt();

                        if (x_coord - x_coord.round()).abs() < 0.00001 {
                            // if point is very close to integer, just draw it on that pixel, and mirror to other 3 symmetric pixels
                            if y_coord == y {
                                for pixel_ind in (self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize))..(self.width * (self.height - 1 - y_coord) + x_coord.round() as usize + 1) {
                                    for channel in 0..color.len() {
                                        self.data[pixel_ind][channel] = ((self.data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    }
                                }
                            } else {
                                for pixel_ind in (self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize))..(self.width * (self.height - 1 - y_coord) + x_coord.round() as usize + 1) {
                                    for channel in 0..color.len() {
                                        self.data[pixel_ind][channel] = ((self.data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    }
                                }
                                for pixel_ind in (self.width * (self.height - 1 - (2 * y - y_coord)) + (2 * x - x_coord.round() as usize))..(self.width * (self.height - 1 - (2 * y - y_coord)) + x_coord.round() as usize + 1) {
                                    for channel in 0..color.len() {
                                        self.data[pixel_ind][channel] = ((self.data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    }
                                }
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = (x_coord - x_coord.floor()) * opacity;
                            let pix2_percentage: f64 = 1.0 - pix1_percentage;
                            let pix_ind: usize = self.width * (self.height - 1 - y_coord) + x_coord.ceil() as usize;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.data[pix_ind][channel] = ((self.data[pix_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                temp_ind = pix_ind + 2 * (y_coord - y) * self.width;
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                temp_ind -= 2 * (x_coord.ceil() as usize - x);
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                            }
                            for pixel_ind in (pix_ind - 2 * (x_coord.ceil() as usize - x) + 1)..pix_ind {
                                for channel in 0..color.len() {
                                    self.data[pixel_ind][channel] = ((self.data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                }
                            }
                            for pixel_ind in (pix_ind + 2 * (y_coord - y) * self.width - 2 * (x_coord.ceil() as usize - x) + 1)..(pix_ind + 2 * (y_coord - y) * self.width) {
                                for channel in 0..color.len() {
                                    self.data[pixel_ind][channel] = ((self.data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                }
                            }
                        }
                    }
                }
            } else {
                if opacity >= 1.0 {
                    // DRAW SOLID CIRCLE

                    // for every x, calculate y and split between top and bottom pixel
                    for x_coord in x..(x_upperlimit + 1) {
                        let y_coord: f64 = y0 + (radius_sqrd - (x_coord as f64 - x0).powi(2)).sqrt();

                        if x_coord == x_upperlimit {
                            y_upperlimit = y_coord.floor() as usize;
                        }

                        if (y_coord - y_coord.round()).abs() < 0.00001 {
                            // if point is very close to integer, just draw it on that pixel, and mirror to other 3 symmetric pixels
                            self.data[self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord] = color;
                            self.data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord] = color;
                            if x_coord != x {
                                // new x coord (mirrored to left) ===> x_coord = x - (x_coord - x)
                                self.data[self.width * (self.height - 1 - (y_coord.round() as usize)) + (2 * x - x_coord)] = color;
                                self.data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + (2 * x - x_coord)] = color;
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = y_coord - y_coord.floor();
                            let pix2_percentage: f64 = 1.0 - pix1_percentage;
                            let pix1_ind: usize = self.width * (self.height - 1 - (y_coord.ceil() as usize)) + x_coord;
                            let pix2_ind: usize = pix1_ind + self.width;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.data[pix1_ind][channel] = ((self.data[pix1_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[pix2_ind][channel] = ((self.data[pix2_ind][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.data[pix1_ind - 2 * (x_coord - x)][channel] = ((self.data[pix1_ind - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[pix2_ind - 2 * (x_coord - x)][channel] = ((self.data[pix2_ind - 2 * (x_coord - x)][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] = ((self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width][channel] = ((self.data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                            }
                        }
                    }

                    // for every y, calculate x and split between left and right pixel (up to y_upperlimit - 1 as it is a special case handled separately)
                    for y_coord in y..y_upperlimit {
                        let x_coord: f64 = x0 + (radius_sqrd - (y_coord as f64 - y0).powi(2)).sqrt();

                        if (x_coord - x_coord.round()).abs() < 0.00001 {
                            // if point is very close to integer, just draw it on that pixel, and mirror to other 3 symmetric pixels
                            self.data[self.width * (self.height - 1 - y_coord) + x_coord.round() as usize] = color;
                            self.data[self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize)] = color;
                            if y_coord != y {
                                // new x coord (mirrored to left) ===> x_coord = x - (x_coord - x)
                                self.data[self.width * (self.height - 1 - (2 * y - y_coord)) + x_coord.round() as usize] = color;
                                self.data[self.width * (self.height - 1 - (2 * y - y_coord)) + (2 * x - x_coord.round() as usize)] = color;
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = x_coord - x_coord.floor();
                            let pix2_percentage: f64 = 1.0 - pix1_percentage;
                            let pix_ind: usize = self.width * (self.height - 1 - y_coord) + x_coord.ceil() as usize;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.data[pix_ind][channel] = ((self.data[pix_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[pix_ind - 1][channel] = ((self.data[pix_ind - 1][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[temp_ind + 1][channel] = ((self.data[temp_ind + 1][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                temp_ind = pix_ind + 2 * (y_coord - y) * self.width;
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[temp_ind - 1][channel] = ((self.data[temp_ind - 1][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                temp_ind -= 2 * (x_coord.ceil() as usize - x);
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[temp_ind + 1][channel] = ((self.data[temp_ind + 1][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                            }
                        }
                    }

                    // special case when y = y_upperlimit, draw only outer pixel (as inner was already filled with something in x loop)
                    let x_coord: f64 = x0 + (radius_sqrd - (y_upperlimit as f64 - y0).powi(2)).sqrt();
                    let pix1_percentage: f64 = x_coord - x_coord.floor();
                    let pix2_percentage: f64 = 1.0 - pix1_percentage;
                    let pix_ind: usize = self.width * (self.height - 1 - y_upperlimit) + x_coord.ceil() as usize;
                    for channel in 0..color.len() {
                        // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                        self.data[pix_ind][channel] = ((self.data[pix_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                        self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        temp_ind = pix_ind + 2 * (y_upperlimit - y) * self.width;
                        self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        temp_ind -= 2 * (x_coord.ceil() as usize - x);
                        self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                    }

                } else {
                    // DRAW TRANSPARENT CIRCLE

                    // for every x, calculate y and split between top and bottom pixel
                    for x_coord in x..(x_upperlimit + 1) {
                        let y_coord: f64 = y0 + (radius_sqrd - (x_coord as f64 - x0).powi(2)).sqrt();

                        if x_coord == x_upperlimit {
                            y_upperlimit = y_coord.floor() as usize;
                        }

                        if (y_coord - y_coord.round()).abs() < 0.00001 {
                            // if point is very close to integer, just draw it on that pixel, and mirror to other 3 symmetric pixels
                            for channel in 0..color.len() {
                                self.data[self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord][channel] = ((self.data[self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                self.data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord][channel] = ((self.data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                            }
                            if x_coord != x {
                                // new x coord (mirrored to left) ===> x_coord = x - (x_coord - x)
                                for channel in 0..color.len() {
                                    self.data[self.width * (self.height - 1 - (y_coord.round() as usize)) + (2 * x - x_coord)][channel] = ((self.data[self.width * (self.height - 1 - (y_coord.round() as usize)) + (2 * x - x_coord)][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    self.data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + (2 * x - x_coord)][channel] = ((self.data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + (2 * x - x_coord)][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                }
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = (y_coord - y_coord.floor()) * opacity;
                            let pix2_percentage: f64 = (y_coord.ceil() - y_coord) * opacity;
                            let pix1_ind: usize = self.width * (self.height - 1 - (y_coord.ceil() as usize)) + x_coord;
                            let pix2_ind: usize = pix1_ind + self.width;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.data[pix1_ind][channel] = ((self.data[pix1_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[pix2_ind][channel] = ((self.data[pix2_ind][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.data[pix1_ind - 2 * (x_coord - x)][channel] = ((self.data[pix1_ind - 2 * (x_coord - x)][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[pix2_ind - 2 * (x_coord - x)][channel] = ((self.data[pix2_ind - 2 * (x_coord - x)][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] = ((self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width][channel] = ((self.data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                            }
                        }
                    }

                    // for every y, calculate x and split between left and right pixel (up to y_upperlimit - 1 as it is a special case handled separately)
                    for y_coord in y..y_upperlimit {
                        let x_coord: f64 = x0 + (radius_sqrd - (y_coord as f64 - y0).powi(2)).sqrt();

                        if (x_coord - x_coord.round()).abs() < 0.00001 {
                            // if point is very close to integer, just draw it on that pixel, and mirror to other 3 symmetric pixels
                            for channel in 0..color.len() {
                                self.data[self.width * (self.height - 1 - y_coord) + x_coord.round() as usize][channel] = ((self.data[self.width * (self.height - 1 - y_coord) + x_coord.round() as usize][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                self.data[self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize)][channel] = ((self.data[self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize)][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                            }
                            if y_coord != y {
                                // new x coord (mirrored to left) ===> x_coord = x - (x_coord - x)
                                for channel in 0..color.len() {
                                    self.data[self.width * (self.height - 1 - (2 * y - y_coord)) + x_coord.round() as usize][channel] = ((self.data[self.width * (self.height - 1 - (2 * y - y_coord)) + x_coord.round() as usize][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    self.data[self.width * (self.height - 1 - (2 * y - y_coord)) + (2 * x - x_coord.round() as usize)][channel] = ((self.data[self.width * (self.height - 1 - (2 * y - y_coord)) + (2 * x - x_coord.round() as usize)][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                }
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = (x_coord - x_coord.floor()) * opacity;
                            let pix2_percentage: f64 = (x_coord.ceil() - x_coord) * opacity;
                            let pix_ind: usize = self.width * (self.height - 1 - y_coord) + x_coord.ceil() as usize;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.data[pix_ind][channel] = ((self.data[pix_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[pix_ind - 1][channel] = ((self.data[pix_ind - 1][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[temp_ind + 1][channel] = ((self.data[temp_ind + 1][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                temp_ind = pix_ind + 2 * (y_coord - y) * self.width;
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[temp_ind - 1][channel] = ((self.data[temp_ind - 1][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                temp_ind -= 2 * (x_coord.ceil() as usize - x);
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[temp_ind + 1][channel] = ((self.data[temp_ind + 1][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                            }
                        }
                    }

                    // special case when y = y_upperlimit, draw only outer pixel (as inner was already filled with something in x loop)
                    let x_coord: f64 = x0 + (radius_sqrd - (y_upperlimit as f64 - y0).powi(2)).sqrt();
                    let pix1_percentage: f64 = (x_coord - x_coord.floor()) * opacity;
                    let pix_ind: usize = self.width * (self.height - 1 - y_upperlimit) + x_coord.ceil() as usize;
                    for channel in 0..color.len() {
                        // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                        self.data[pix_ind][channel] = ((self.data[pix_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                        self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        temp_ind = pix_ind + 2 * (y_upperlimit - y) * self.width;
                        self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        temp_ind -= 2 * (x_coord.ceil() as usize - x);
                        self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                    }
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::collapsible_else_if)]
    pub fn draw_ellipse(&mut self, x: usize, y: usize, horizontal_axis: usize, vertical_axis: usize, color: [u8; 3], thickness: usize, opacity: f64) {
        //! Draws a new ellipse. `x`, `y` are the coordinates of the center of the ellipse.
        //! `horizontal_axis` defines the half length of the horizontal axis.
        //! `vertical_axis` defines the half length of the vertical axis.
        //! `color` defines the color of the ellipse.
        //! `thickness` defines how thick the ellipse will be. (currently doesn't do anything). If set to 0, the ellipse will be filled.
        //! `opacity` sets the transparency of the ellipse.
        //! `<= 0.0` means the ellipse will be completely transparent, while `>= 1.0` means the ellipse won't be transparent.

        if (horizontal_axis > 0) && (vertical_axis > 0) && (opacity >= 0.0) && (x < self.width) && (y < self.height) && (x >= horizontal_axis) && (y >= vertical_axis) && (x + horizontal_axis < self.width) && (y + vertical_axis < self.height) {
            let x0: f64 = x as f64;
            let y0: f64 = y as f64;
            let x_upperlimit: usize = x + ((horizontal_axis.pow(2) as f64) / ((horizontal_axis.pow(2) + vertical_axis.pow(2)) as f64).sqrt()).round() as usize;  // x up to which draw adjacent pixels vertically
            let mut y_upperlimit: usize = 0;  // y up to which draw adjacent pixels horizontally (is initialized to 0, but is changed later)

            if thickness == 0 {
                if opacity >= 1.0 {
                    // DRAW FILLED SOLID ELLIPSE

                    // for every x, calculate y and draw outer pixel, connecting inner pixels with solid line (for filling circle)
                    let mut previous_y: usize = y + vertical_axis;
                    for x_coord in x..(x_upperlimit + 1) {
                        let y_coord: f64 = y0 + (vertical_axis as f64) * (1.0 - ((x_coord as f64 - x0) / horizontal_axis as f64).powi(2)).sqrt();

                        if x_coord == x_upperlimit {
                            y_upperlimit = y_coord.floor() as usize;
                        }

                        if (y_coord - y_coord.round()).abs() < 0.00001 {
                            // if point is very close to integer, just draw it on that pixel, and mirror to other 3 symmetric pixels
                            if x_coord == x {
                                self.data[self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord] = color;
                                self.data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord] = color;
                            } else if y_coord.round() as usize > y_upperlimit {
                                self.data[(self.width * (self.height - 1 - (y_coord.round() as usize)) + (2 * x - x_coord))..(self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord + 1)].fill(color);
                                self.data[(self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + (2 * x - x_coord))..(self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord + 1)].fill(color);
                                previous_y = y_coord.round() as usize;
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = y_coord - y_coord.floor();
                            let pix2_percentage: f64 = 1.0 - pix1_percentage;
                            let pix1_ind: usize = self.width * (self.height - 1 - (y_coord.ceil() as usize)) + x_coord;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.data[pix1_ind][channel] = ((self.data[pix1_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.data[pix1_ind - 2 * (x_coord - x)][channel] = ((self.data[pix1_ind - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] = ((self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                            }
                            if (y_coord.ceil() as usize) < previous_y {
                                self.data[(pix1_ind - 2 * (x_coord - x) + 1)..pix1_ind].fill(color);
                                self.data[(pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x) + 1)..(pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width)].fill(color);
                                previous_y = y_coord.ceil() as usize
                            }
                        }
                    }

                    // for every y, calculate x and draw outer pixel, connecting inner pixels with solid line (for filling circle)
                    for y_coord in y..(y_upperlimit + 1) {
                        let x_coord: f64 = x0 + (horizontal_axis as f64) * (1.0 - ((y_coord as f64 - y0) / vertical_axis as f64).powi(2)).sqrt();

                        if (x_coord - x_coord.round()).abs() < 0.00001 {
                            // if point is very close to integer, just draw it on that pixel, and mirror to other 3 symmetric pixels
                            if y_coord == y {
                                self.data[(self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize))..(self.width * (self.height - 1 - y_coord) + x_coord.round() as usize + 1)].fill(color);
                            } else {
                                self.data[(self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize))..(self.width * (self.height - 1 - y_coord) + (x_coord.round() as usize) + 1)].fill(color);
                                self.data[(self.width * (self.height - 1 - (2 * y - y_coord)) + (2 * x - x_coord.round() as usize))..(self.width * (self.height - 1 - (2 * y - y_coord)) + (x_coord.round() as usize) + 1)].fill(color);
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = x_coord - x_coord.floor();
                            let pix2_percentage: f64 = 1.0 - pix1_percentage;
                            let pix_ind: usize = self.width * (self.height - 1 - y_coord) + x_coord.ceil() as usize;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.data[pix_ind][channel] = ((self.data[pix_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                temp_ind = pix_ind + 2 * (y_coord - y) * self.width;
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                temp_ind -= 2 * (x_coord.ceil() as usize - x);
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                            }
                            self.data[(pix_ind - 2 * (x_coord.ceil() as usize - x) + 1)..pix_ind].fill(color);
                            self.data[(pix_ind + 2 * (y_coord - y) * self.width - 2 * (x_coord.ceil() as usize - x) + 1)..(pix_ind + 2 * (y_coord - y) * self.width)].fill(color);
                        }
                    }

                } else {
                    // DRAW FILLED TRANSPARENT ELLIPSE

                    // for every x, calculate y and draw outer pixel, connecting inner pixels with solid line (for filling circle)
                    let mut previous_y: usize = y + vertical_axis;
                    for x_coord in x..(x_upperlimit + 1) {
                        let y_coord: f64 = y0 + (vertical_axis as f64) * (1.0 - ((x_coord as f64 - x0) / horizontal_axis as f64).powi(2)).sqrt();

                        if x_coord == x_upperlimit {
                            y_upperlimit = y_coord.floor() as usize;
                        }

                        if (y_coord - y_coord.round()).abs() < 0.00001 {
                            // if point is very close to integer, just draw it on that pixel, and mirror to other 3 symmetric pixels
                            if x_coord == x {
                                for channel in 0..color.len() {
                                    self.data[self.width * (self.height - 1 - y_coord.round() as usize) + x_coord][channel] = ((self.data[self.width * (self.height - 1 - y_coord.round() as usize) + x_coord][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    self.data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord][channel] = ((self.data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                }
                            } else if y_coord.round() as usize > y_upperlimit {
                                for pixel_ind in (self.width * (self.height - 1 - (y_coord.round() as usize)) + (2 * x - x_coord))..(self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord + 1) {
                                    for channel in 0..color.len() {
                                        self.data[pixel_ind][channel] = ((self.data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    }
                                }
                                for pixel_ind in (self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + (2 * x - x_coord))..(self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord + 1) {
                                    for channel in 0..color.len() {
                                        self.data[pixel_ind][channel] = ((self.data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    }
                                }
                                previous_y = y_coord.round() as usize;
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = (y_coord - y_coord.floor()) * opacity;
                            let pix2_percentage: f64 = 1.0 - pix1_percentage;
                            let pix1_ind: usize = self.width * (self.height - 1 - (y_coord.ceil() as usize)) + x_coord;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.data[pix1_ind][channel] = ((self.data[pix1_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.data[pix1_ind - 2 * (x_coord - x)][channel] = ((self.data[pix1_ind - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] = ((self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                            }
                            if (y_coord.ceil() as usize) < previous_y {
                                for pixel_ind in (pix1_ind - 2 * (x_coord - x) + 1)..pix1_ind {
                                    for channel in 0..color.len() {
                                        self.data[pixel_ind][channel] = ((self.data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    }
                                }
                                for pixel_ind in (pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x) + 1)..(pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width) {
                                    for channel in 0..color.len() {
                                        self.data[pixel_ind][channel] = ((self.data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    }
                                }
                                previous_y = y_coord.ceil() as usize
                            }
                        }
                    }

                    // for every y, calculate x and draw outer pixel, connecting inner pixels with solid line (for filling circle)
                    for y_coord in y..(y_upperlimit + 1) {
                        let x_coord: f64 = x0 + (horizontal_axis as f64) * (1.0 - ((y_coord as f64 - y0) / vertical_axis as f64).powi(2)).sqrt();

                        if (x_coord - x_coord.round()).abs() < 0.00001 {
                            // if point is very close to integer, just draw it on that pixel, and mirror to other 3 symmetric pixels
                            if x_coord.round() as usize == x {
                                continue
                            } else if y_coord == y {
                                for pixel_ind in (self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize))..(self.width * (self.height - 1 - y_coord) + x_coord.round() as usize + 1) {
                                    for channel in 0..color.len() {
                                        self.data[pixel_ind][channel] = ((self.data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    }
                                }
                            } else {
                                for pixel_ind in (self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize))..(self.width * (self.height - 1 - y_coord) + x_coord.round() as usize + 1) {
                                    for channel in 0..color.len() {
                                        self.data[pixel_ind][channel] = ((self.data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    }
                                }
                                for pixel_ind in (self.width * (self.height - 1 - (2 * y - y_coord)) + (2 * x - x_coord.round() as usize))..(self.width * (self.height - 1 - (2 * y - y_coord)) + x_coord.round() as usize + 1) {
                                    for channel in 0..color.len() {
                                        self.data[pixel_ind][channel] = ((self.data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    }
                                }
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = (x_coord - x_coord.floor()) * opacity;
                            let pix2_percentage: f64 = 1.0 - pix1_percentage;
                            let pix_ind: usize = self.width * (self.height - 1 - y_coord) + x_coord.ceil() as usize;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.data[pix_ind][channel] = ((self.data[pix_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                temp_ind = pix_ind + 2 * (y_coord - y) * self.width;
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                temp_ind -= 2 * (x_coord.ceil() as usize - x);
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                            }
                            for pixel_ind in (pix_ind - 2 * (x_coord.ceil() as usize - x) + 1)..pix_ind {
                                for channel in 0..color.len() {
                                    self.data[pixel_ind][channel] = ((self.data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                }
                            }
                            for pixel_ind in (pix_ind + 2 * (y_coord - y) * self.width - 2 * (x_coord.ceil() as usize - x) + 1)..(pix_ind + 2 * (y_coord - y) * self.width) {
                                for channel in 0..color.len() {
                                    self.data[pixel_ind][channel] = ((self.data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                }
                            }
                        }
                    }
                }
            } else {
                if opacity >= 1.0 {
                    // DRAW SOLID ELLIPSE

                    // for every x, calculate y and split between top and bottom pixel
                    for x_coord in x..(x_upperlimit + 1) {
                        let y_coord: f64 = y0 + (vertical_axis as f64) * (1.0 - ((x_coord as f64 - x0) / horizontal_axis as f64).powi(2)).sqrt();

                        if x_coord == x_upperlimit {
                            y_upperlimit = y_coord.floor() as usize;
                        }

                        if (y_coord - y_coord.round()).abs() < 0.00001 {
                            // if point is very close to integer, just draw it on that pixel, and mirror to other 3 symmetric pixels
                            self.data[self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord] = color;
                            self.data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord] = color;
                            if x_coord != x {
                                // new x coord (mirrored to left) ===> x_coord = x - (x_coord - x)
                                self.data[self.width * (self.height - 1 - (y_coord.round() as usize)) + (2 * x - x_coord)] = color;
                                self.data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + (2 * x - x_coord)] = color;
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = y_coord - y_coord.floor();
                            let pix2_percentage: f64 = 1.0 - pix1_percentage;
                            let pix1_ind: usize = self.width * (self.height - 1 - (y_coord.ceil() as usize)) + x_coord;
                            let pix2_ind: usize = pix1_ind + self.width;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.data[pix1_ind][channel] = ((self.data[pix1_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[pix2_ind][channel] = ((self.data[pix2_ind][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.data[pix1_ind - 2 * (x_coord - x)][channel] = ((self.data[pix1_ind - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[pix2_ind - 2 * (x_coord - x)][channel] = ((self.data[pix2_ind - 2 * (x_coord - x)][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] = ((self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width][channel] = ((self.data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                            }
                        }
                    }

                    // for every y, calculate x and split between left and right pixel (up to y_upperlimit - 1 as it is a special case handled separately)
                    for y_coord in y..y_upperlimit {
                        let x_coord: f64 = x0 + (horizontal_axis as f64) * (1.0 - ((y_coord as f64 - y0) / vertical_axis as f64).powi(2)).sqrt();

                        if (x_coord - x_coord.round()).abs() < 0.00001 {
                            // if point is very close to integer, just draw it on that pixel, and mirror to other 3 symmetric pixels
                            self.data[self.width * (self.height - 1 - y_coord) + x_coord.round() as usize] = color;
                            self.data[self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize)] = color;
                            if y_coord != y {
                                // new x coord (mirrored to left) ===> x_coord = x - (x_coord - x)
                                self.data[self.width * (self.height - 1 - (2 * y - y_coord)) + x_coord.round() as usize] = color;
                                self.data[self.width * (self.height - 1 - (2 * y - y_coord)) + (2 * x - x_coord.round() as usize)] = color;
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = x_coord - x_coord.floor();
                            let pix2_percentage: f64 = 1.0 - pix1_percentage;
                            let pix_ind: usize = self.width * (self.height - 1 - y_coord) + x_coord.ceil() as usize;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.data[pix_ind][channel] = ((self.data[pix_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[pix_ind - 1][channel] = ((self.data[pix_ind - 1][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[temp_ind + 1][channel] = ((self.data[temp_ind + 1][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                temp_ind = pix_ind + 2 * (y_coord - y) * self.width;
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[temp_ind - 1][channel] = ((self.data[temp_ind - 1][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                temp_ind -= 2 * (x_coord.ceil() as usize - x);
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[temp_ind + 1][channel] = ((self.data[temp_ind + 1][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                            }
                        }
                    }

                    // special case when y = y_upperlimit, draw only outer pixel (as inner was already filled with something in x loop)
                    let x_coord: f64 = x0 + (horizontal_axis as f64) * (1.0 - ((y_upperlimit as f64 - y0) / vertical_axis as f64).powi(2)).sqrt();
                    let pix1_percentage: f64 = x_coord - x_coord.floor();
                    let pix2_percentage: f64 = 1.0 - pix1_percentage;
                    let pix_ind: usize = self.width * (self.height - 1 - y_upperlimit) + x_coord.ceil() as usize;
                    for channel in 0..color.len() {
                        // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                        self.data[pix_ind][channel] = ((self.data[pix_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                        self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        temp_ind = pix_ind + 2 * (y_upperlimit - y) * self.width;
                        self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        temp_ind -= 2 * (x_coord.ceil() as usize - x);
                        self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                    }

                } else {
                    // DRAW TRANSPARENT ELLIPSE

                    // for every x, calculate y and split between top and bottom pixel
                    for x_coord in x..(x_upperlimit + 1) {
                        let y_coord: f64 = y0 + (vertical_axis as f64) * (1.0 - ((x_coord as f64 - x0) / horizontal_axis as f64).powi(2)).sqrt();

                        if x_coord == x_upperlimit {
                            y_upperlimit = y_coord.floor() as usize;
                        }

                        if (y_coord - y_coord.round()).abs() < 0.00001 {
                            // if point is very close to integer, just draw it on that pixel, and mirror to other 3 symmetric pixels
                            for channel in 0..color.len() {
                                self.data[self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord][channel] = ((self.data[self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                            }
                            if y_coord.round() as usize != y {
                                for channel in 0..color.len() {
                                    self.data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord][channel] = ((self.data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                }
                            }
                            if x_coord != x {
                                // new x coord (mirrored to left) ===> x_coord = x - (x_coord - x)
                                for channel in 0..color.len() {
                                    self.data[self.width * (self.height - 1 - (y_coord.round() as usize)) + (2 * x - x_coord)][channel] = ((self.data[self.width * (self.height - 1 - (y_coord.round() as usize)) + (2 * x - x_coord)][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                }
                                if y_coord.round() as usize != y {
                                    for channel in 0..color.len() {
                                        self.data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + (2 * x - x_coord)][channel] = ((self.data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + (2 * x - x_coord)][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    }
                                }
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = (y_coord - y_coord.floor()) * opacity;
                            let pix2_percentage: f64 = (y_coord.ceil() - y_coord) * opacity;
                            let pix1_ind: usize = self.width * (self.height - 1 - (y_coord.ceil() as usize)) + x_coord;
                            let pix2_ind: usize = pix1_ind + self.width;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.data[pix1_ind][channel] = ((self.data[pix1_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[pix2_ind][channel] = ((self.data[pix2_ind][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.data[pix1_ind - 2 * (x_coord - x)][channel] = ((self.data[pix1_ind - 2 * (x_coord - x)][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[pix2_ind - 2 * (x_coord - x)][channel] = ((self.data[pix2_ind - 2 * (x_coord - x)][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] = ((self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width][channel] = ((self.data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                            }
                        }
                    }

                    // for every y, calculate x and split between left and right pixel (up to y_upperlimit - 1 as it is a special case handled separately)
                    for y_coord in y..y_upperlimit {
                        let x_coord: f64 = x0 + (horizontal_axis as f64) * (1.0 - ((y_coord as f64 - y0) / vertical_axis as f64).powi(2)).sqrt();

                        if (x_coord - x_coord.round()).abs() < 0.00001 {
                            // if point is very close to integer, just draw it on that pixel, and mirror to other 3 symmetric pixels
                            for channel in 0..color.len() {
                                self.data[self.width * (self.height - 1 - y_coord) + x_coord.round() as usize][channel] = ((self.data[self.width * (self.height - 1 - y_coord) + x_coord.round() as usize][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                self.data[self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize)][channel] = ((self.data[self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize)][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                            }
                            if y_coord != y {
                                // new x coord (mirrored to left) ===> x_coord = x - (x_coord - x)
                                for channel in 0..color.len() {
                                    self.data[self.width * (self.height - 1 - (2 * y - y_coord)) + x_coord.round() as usize][channel] = ((self.data[self.width * (self.height - 1 - (2 * y - y_coord)) + x_coord.round() as usize][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    self.data[self.width * (self.height - 1 - (2 * y - y_coord)) + (2 * x - x_coord.round() as usize)][channel] = ((self.data[self.width * (self.height - 1 - (2 * y - y_coord)) + (2 * x - x_coord.round() as usize)][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                }
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = (x_coord - x_coord.floor()) * opacity;
                            let pix2_percentage: f64 = (x_coord.ceil() - x_coord) * opacity;
                            let pix_ind: usize = self.width * (self.height - 1 - y_coord) + x_coord.ceil() as usize;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.data[pix_ind][channel] = ((self.data[pix_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[pix_ind - 1][channel] = ((self.data[pix_ind - 1][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[temp_ind + 1][channel] = ((self.data[temp_ind + 1][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                temp_ind = pix_ind + 2 * (y_coord - y) * self.width;
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[temp_ind - 1][channel] = ((self.data[temp_ind - 1][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                temp_ind -= 2 * (x_coord.ceil() as usize - x);
                                self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.data[temp_ind + 1][channel] = ((self.data[temp_ind + 1][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                            }
                        }
                    }

                    // special case when y = y_upperlimit, draw only outer pixel (as inner was already filled with something in x loop)
                    let x_coord: f64 = x0 + (horizontal_axis as f64) * (1.0 - ((y_upperlimit as f64 - y0) / vertical_axis as f64).powi(2)).sqrt();
                    let pix1_percentage: f64 = (x_coord - x_coord.floor()) * opacity;
                    let pix_ind: usize = self.width * (self.height - 1 - y_upperlimit) + x_coord.ceil() as usize;
                    for channel in 0..color.len() {
                        // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                        self.data[pix_ind][channel] = ((self.data[pix_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                        self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        temp_ind = pix_ind + 2 * (y_upperlimit - y) * self.width;
                        self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        temp_ind -= 2 * (x_coord.ceil() as usize - x);
                        self.data[temp_ind][channel] = ((self.data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                    }
                }
            }
        }
    }
}
*/

impl Image {
    /// Creates a new image with the given width, height, and background color.
    /// # Arguments
    /// * `width` - The width of the image.
    /// * `height` - The height of the image.
    /// * `background` - The background color of the image.
    /// # Returns
    /// * The new image.
    #[allow(clippy::uninit_vec)]
    pub fn new(width: usize, height: usize, background_color: Color) -> Self {
        // create uninitialized data vector
        let len = width * height * background_color.as_bytes().len();
        let mut data = Vec::with_capacity(len);
        unsafe { data.set_len(len); }
        
        // create image
        let mut image = Self {
            data,
            width,
            height,
            color_type: ColorType::from(background_color),
            background_color: Some(background_color),
        };
        
        // call clear to fill the image with the background color (initialize data)
        image.clear();
        
        image
    }

    /// Returns the width of the image.
    /// # Returns
    /// * The width of the image.
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the image.
    /// # Returns
    /// * The height of the image.
    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the type of the image.
    /// # Returns
    /// * The type of the image.
    #[inline]
    pub fn color_type(&self) -> ColorType {
        self.color_type
    }

    /// Returns the bytes of the image as a native endian slice.
    /// # Returns
    /// * The bytes of the image.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    /// Returns the bytes of the image as a mutable native endian slice.
    /// # Returns
    /// * The bytes of the image.
    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Returns the background color of the image.
    /// # Returns
    /// * The background color of the image.
    #[inline]
    pub fn background_color(&self) -> Option<Color> {
        self.background_color
    }

    /// Sets the background color of the image.
    /// # Arguments
    /// * `color` - The new background color.
    /// # Returns
    /// * Ok(()) if the background color was set successfully.
    /// * Err(Error::WrongColor) if the color is not compatible with the image type.
    pub fn set_background_color(&mut self, color: Color) -> Result<(), Error> {
        if ColorType::from(color) == self.color_type {
            self.background_color = Some(color);
            Ok(())
        } else {
            Err(Error::WrongColor)
        }
    }

    /// Reset the image to the background color.
    /// If the background color is not set, this is no-op.
    pub fn clear(&mut self) {
        if let Some(color) = self.background_color {
            let color_slice = color.as_bytes();
            for i in (0..self.data.len()).step_by(color_slice.len()) {
                self.data[i..(color_slice.len() + i)].copy_from_slice(color_slice);
            }
        }
    }
}

impl Drawing for Image {
    fn draw_circle(&mut self, center: (usize, usize), radius: usize, color: Color, thickness: usize, opacity: f64) -> Result<(), Error> {
        todo!()
    }

    fn draw_ellipse(&mut self, center: (usize, usize), axes: (usize, usize), color: Color, thickness: usize, opacity: f64) -> Result<(), Error> {
        todo!()
    }

    fn draw_line(&mut self, point1: (usize, usize), point2: (usize, usize), color: Color, thickness: usize, opacity: f64) -> Result<(), Error> {
        todo!()
    }

    fn draw_rectangle(&mut self, point1: (usize, usize), point2: (usize, usize), color: Color, thickness: usize, opacity: f64) -> Result<(), Error> {
        // check if color is valid for this image type
        if ColorType::from(color) != self.color_type {
            return Err(Error::WrongColor);
        }

        // if opacity is less than 0.0, bigger than 1.0, or NaN, return error
        if opacity.is_nan() || !(0.0..=1.0).contains(&opacity) {
            return Err(Error::InvalidOpacity);
        }

        // if opacity is 0.0, nothing is to be drawn
        if opacity == 0.0 {
            return Ok(());
        }

        // find corners
        let mut smaller_x = min(point1.0, point2.0);
        let mut bigger_x = max(point1.0, point2.0);
        let mut smaller_y = min(point1.1, point2.1);
        let mut bigger_y = max(point1.1, point2.1);
        if smaller_x >= self.width || smaller_y >= self.height {
            return Ok(()); // rectangle is out of image, nothing is to be drawn.
        }

        if thickness == 0 {
            if bigger_x >= self.width {
                bigger_x = self.width - 1;
            }
            if bigger_y >= self.height {
                bigger_y = self.height - 1;
            }

            if opacity >= 1.0 {
                // Draw filled, solid rectangle.
                self.set_unchecked((smaller_x..(bigger_x + 1), smaller_y..(bigger_y + 1)), color);
            } else {
                // Draw filled, transparent rectangle.
                self.set_transparent_unchecked((smaller_x..(bigger_x + 1), smaller_y..(bigger_y + 1)), color, opacity);
            }
        } else {
            // new thickness variable, as it will be modified
            let mut used_thickness = thickness;
            // find maximum possible thickness
            let limit_x = (bigger_x - smaller_x) / 2 + 1;
            let limit_y = (bigger_y - smaller_y) / 2 + 1;
            if (thickness > limit_x) || (thickness > limit_y) {
                used_thickness = min(limit_x, limit_y);
            }
            used_thickness = min(used_thickness, self.width - smaller_x);
            used_thickness = min(used_thickness, self.height - smaller_y);

            // draw smaller and smaller rectangles until given thickness is achieved
            if opacity >= 1.0 {
                // Draw rectangle, solid

                while used_thickness > 0 {
                    if bigger_x == smaller_x {
                        if bigger_y == smaller_y {
                            self.set_unchecked((smaller_x, smaller_y), color);
                        } else {
                            self.set_unchecked((smaller_x..(smaller_x + 1), smaller_y..min(self.height, bigger_y + 1)), color);
                        }
                    } else if bigger_y == smaller_y {
                        self.set_unchecked((smaller_x..min(self.width, bigger_x + 1), smaller_y..(smaller_y + 1)), color);
                    } else {
                        self.set_unchecked((smaller_x..min(self.width, bigger_x + 1), smaller_y..(smaller_y + 1)), color); // bottom
                        if bigger_y < self.height {
                            self.set_unchecked((smaller_x..min(self.width, bigger_x + 1), bigger_y..(bigger_y + 1)), color);
                            // top
                        }
                        self.set_unchecked((smaller_x..(smaller_x + 1), (smaller_y + 1)..min(bigger_y, self.height)), color); // left
                        if bigger_x < self.width {
                            self.set_unchecked((bigger_x..(bigger_x + 1), (smaller_y + 1)..min(bigger_y, self.height)), color);
                            // right
                        }
                    }

                    smaller_x += 1;
                    smaller_y += 1;
                    bigger_x -= 1;
                    bigger_y -= 1;

                    used_thickness -= 1;
                }
            } else {
                // Draw rectangle, transparent

                while used_thickness > 0 {
                    if bigger_x == smaller_x {
                        if bigger_y == smaller_y {
                            self.set_transparent_unchecked((smaller_x, smaller_y), color, opacity);
                        } else {
                            self.set_transparent_unchecked((smaller_x..(smaller_x + 1), smaller_y..min(self.height, bigger_y + 1)), color, opacity);
                        }
                    } else if bigger_y == smaller_y {
                        self.set_transparent_unchecked((smaller_x..min(self.width, bigger_x + 1), smaller_y..(smaller_y + 1)), color, opacity);
                    } else {
                        self.set_transparent_unchecked((smaller_x..min(self.width, bigger_x + 1), smaller_y..(smaller_y + 1)), color, opacity); // bottom
                        if bigger_y < self.height {
                            self.set_transparent_unchecked((smaller_x..min(self.width, bigger_x + 1), bigger_y..(bigger_y + 1)), color, opacity);
                            // top
                        }
                        self.set_transparent_unchecked((smaller_x..(smaller_x + 1), (smaller_y + 1)..min(bigger_y, self.height)), color, opacity); // left
                        if bigger_x < self.width {
                            self.set_transparent_unchecked((bigger_x..(bigger_x + 1), (smaller_y + 1)..min(bigger_y, self.height)), color, opacity);
                            // right
                        }
                    }

                    smaller_x += 1;
                    smaller_y += 1;
                    bigger_x -= 1;
                    bigger_y -= 1;

                    used_thickness -= 1;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drawing_rectangle() {
        let mut image = Image::new(100, 100, Color::RGB8([255, 255, 255]));

        // test errors
        if image.draw_rectangle((0, 0), (10, 10), Color::RGBA8([0, 0, 0, 0]), 1, 1.0).is_ok() {
            panic!("Should fail!")
        }
        if image.draw_rectangle((0, 0), (10, 10), Color::RGB8([0, 0, 0]), 0, 1.1).is_ok() {
            panic!("Should fail!")
        }

        // test drawing
        image.draw_rectangle((0, 0), (10, 10), Color::RGB8([0, 0, 0]), 1, 1.0).unwrap();
        image.draw_rectangle((20, 20), (31, 31), Color::RGB8([0, 0, 0]), 1, 0.5).unwrap();
        image.draw_rectangle((40, 40), (50, 50), Color::RGB8([0, 0, 0]), 3, 1.0).unwrap();
        image.draw_rectangle((60, 60), (70, 70), Color::RGB8([0, 0, 0]), 3, 0.5).unwrap();
        image.draw_rectangle((80, 80), (90, 90), Color::RGB8([0, 0, 0]), 0, 1.0).unwrap();
        image.draw_rectangle((10, 90), (20, 80), Color::RGB8([0, 0, 0]), 0, 0.5).unwrap();
        image.draw_rectangle((30, 70), (40, 60), Color::RGB8([0, 0, 0]), 1000000, 1.0).unwrap();
        image.draw_rectangle((80, 10), (90, 30), Color::RGB8([0, 0, 0]), 1000000, 0.5).unwrap();

        // image.to_file("test_drawing_rectangle.png", true).unwrap();
    }

    #[test]
    fn utilities_fields() {
        let image = Image::new(100, 100, Color::GRAY8(255));

        assert_eq!(image.as_bytes(), &vec![255; 100 * 100]);
        assert_eq!(image.as_bytes(), &image.data);

        assert_eq!(image.width(), 100);
        assert_eq!(image.width(), image.width);

        assert_eq!(image.height(), 100);
        assert_eq!(image.height(), image.height);

        assert_eq!(image.color_type(), ColorType::GRAY8);

        // TODO: test set background, write docs for set background
    }
}

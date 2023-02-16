//! A module that contains the [ImageRGB8] struct and related functions.

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use bytemuck::try_cast_slice;
use std::cmp::{min, max};
use std::f64::consts::FRAC_1_SQRT_2;


fn bytes_to_rgb8(bytes: &[u8]) -> Vec<[u8; 3]> {
    // converts a slice of bytes to a vector of pixels
    try_cast_slice::<u8, [u8; 3]>(bytes).expect("This shouldn't fail!").to_vec()
}

fn rgb8_to_bytes(rgb8: &[[u8; 3]]) -> &[u8] {
    // returns a slice of bytes of a vector (slice) of pixels
    try_cast_slice(rgb8).expect("This shouldn't fail!")
}

enum BackgroundRGB8 {
    // enum that holds image background information for ImageRGB8
    Color([u8; 3]),
    Image(Vec<[u8; 3]>)
}

/// A struct that holds an RGB image with bit depth of 8
pub struct ImageRGB8 {
    /// The width of the image
    pub width: usize,
    /// The height of the image
    pub height: usize,
    /// The image pixel data
    pub image_data: Vec<[u8; 3]>,
    background_data: BackgroundRGB8
}

impl ImageRGB8 {
    pub fn new(width: usize, height: usize, background: [u8; 3]) -> Self {
        //! Returns a new [ImageRGB8].
        //! ```width```, ```height``` are image dimensions.
        //! ```background``` is image's color.

        Self { width, height, image_data: vec![background; width * height], background_data: BackgroundRGB8::Color(background) }
    }

    pub fn from_png(path: &str) -> Result<Self, &'static str> {
        //! Reads the image from a PNG file.
        //! Returns [Result] which holds new [ImageRGB8] or [Err] with informative message.
        //! ```path``` is the path to the PNG file.
        //! The PNG file should be RGB or RGBA with bit depth of 8.

        match File::open(path) {
            Ok(file) =>
                {
                    let decoder = png::Decoder::new(file);
                    match decoder.read_info() {
                        Ok(information) =>
                            {
                                let mut reader = information;
                                // Allocate the output buffer.
                                let mut buf = vec![0; reader.output_buffer_size()];
                                // Read the next frame. An APNG might contain multiple frames.
                                match reader.next_frame(&mut buf) {
                                    Ok(new_information) =>
                                        {
                                            let info = new_information;
                                            // Grab the bytes of the image.
                                            let bytes: &[u8];
                                            if info.bit_depth == png::BitDepth::Eight {
                                                // if image is not RGB panic, if it is RGBA convert to RGB
                                                match info.color_type {
                                                    png::ColorType::Rgb => {
                                                        bytes = &buf[..info.buffer_size()];
                                                        // return ImageRGB8 struct
                                                        Ok(Self::from_bytes(info.width as usize, info.height as usize, bytes).expect("This shouldn't fail!"))
                                                    },
                                                    png::ColorType::Rgba => {
                                                        buf.truncate(info.buffer_size());
                                                        let mut iterator = 1..(buf.len() + 1);
                                                        buf.retain(|_| iterator.next().expect("This shouldn't fail!") % 4 != 0);
                                                        bytes = &buf;
                                                        // return ImageRGB8 struct
                                                        Ok(Self::from_bytes(info.width as usize, info.height as usize, bytes).expect("This shouldn't fail!"))
                                                    },
                                                    _ => Err("Image color not RGB or RGBA!")
                                                }
                                            } else {
                                                Err("Image bit depth is not 8!")
                                            }
                                        },
                                    Err(_) => Err("Can't read file!")
                                }
                            },
                        Err(_) => Err("Can't read file!"),
                    }
                },
            Err(_) => Err("Can't open file!"),
        }
    }

    pub fn from_bytes(width: usize, height: usize, bytes: &[u8]) -> Result<Self, &'static str> {
        //! Returns [Result] with new [ImageRGB8] or [Err] with informative message.
        //! It is constructed from ```width```, ```height``` and ```bytes```

        if width * height * 3 != bytes.len() {
            // if number of bytes doesn't match expected number of bytes, panic
            Err("Number of bytes does not match an RGB image with given dimensions!")
        } else {
            // generate RGB image from bytes separately as it needs to be cloned as two separate instances are needed
            let img = bytes_to_rgb8(bytes);
            Ok(Self { width, height, image_data: img.clone(), background_data: BackgroundRGB8::Image(img) })
        }
    }

    pub fn to_png(&self, path: &str) -> Result<(), &'static str> {
        //! Saves the image as PNG.
        //! ```path``` is a path + filename where it will be saved.
        //! Returns [Ok] if everything goes well, or [Err] with description of the error.
        let path = Path::new(path);

        match File::create(path) {
            Ok(new_file) =>
                {
                    let file = new_file;
                    let w = BufWriter::new(file);

                    let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
                    encoder.set_color(png::ColorType::Rgb);
                    encoder.set_depth(png::BitDepth::Eight);

                    match encoder.write_header() {
                        Ok(mut writer) =>
                            {
                                match writer.write_image_data(self.to_bytes()) {
                                    Ok(_) => Ok(()),
                                    Err(_) => Err("Can't write image to file!")
                                }
                            },
                        Err(_) => Err("Can't write image to file!")
                    }
                },
            Err(_) => Err("Can't create file!")
        }
    }

    pub fn to_bytes(&self) -> &[u8] {
        //! Returns a slice of bytes of the ```image_data```
        rgb8_to_bytes(&self.image_data)
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Result<[u8; 3], &'static str> {
        //! Returns an RGB value of the specified pixel if that pixel exists.

        if x >= self.width || y >= self.height {
            Err("Given coordinates exceed image limits!")
        } else {
            Ok(self.image_data[self.width * (self.height - 1 - y) + x])
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: [u8; 3]) {
        //! Changes the specified pixel to the given ```color```.
        //! If the pixel doesn't exist, does nothing.

        if x < self.width || y < self.width {
            self.image_data[self.width * (self.height - 1 - y) + x] = color;
        }
    }

    pub fn clear(&mut self) {
        //! Clears ```image_data``` of any drawings (resets it to the state it was in when [ImageRGB8] was created, unless [ImageRGB8::set_background_color()] was used).

        match &self.background_data {
            BackgroundRGB8::Color(color) => self.image_data.fill(*color),
            BackgroundRGB8::Image(img) => self.image_data = img.clone(),
        }
    }

    pub fn set_background_color(&mut self, color: [u8; 3]) {
        //! Sets a new color that will be used as background.
        //! This only changes internal background data, if you want to apply this to image, call [ImageRGB8::clear()] after this.

        self.background_data = BackgroundRGB8::Color(color);
    }

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
                        self.image_data[self.width * (self.height - 1 - y) + x1] = color;
                    }
                } else {
                    for y in lower_y..(upper_y + 1) {
                        for channel in 0..color.len() {
                            self.image_data[self.width * (self.height - 1 - y) + x1][channel] = ((self.image_data[self.width * (self.height - 1 - y) + x1][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
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
                    self.image_data[(row + lower_x)..(row + upper_x + 1)].fill(color);
                } else {
                    for ind in (row + lower_x)..(row + upper_x + 1) {
                        for channel in 0..color.len() {
                            self.image_data[ind][channel] = ((self.image_data[ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
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
                                self.image_data[self.width * (self.height - 1 - (y.round() as usize)) + x] = color;
                            } else {
                                let ind: usize = self.width * (self.height - 1 - (y.round() as usize)) + x;
                                for channel in 0..color.len() {
                                    self.image_data[ind][channel] = ((self.image_data[ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
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
                                self.image_data[pix1_ind][channel] = ((self.image_data[pix1_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[pix2_ind][channel] = ((self.image_data[pix2_ind][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;
                            }
                        }
                    }
                } else {
                    for y in min(y1_calc, y2_calc)..(max(y1_calc, y2_calc) + 1) {
                        let x: f64 = (((y - y1) as f64) / slope) + (x1 as f64);
                        if (x - x.round()).abs() < 0.00001 {
                            // if point is very close to integer, just draw it on that pixel
                            if opacity >= 1.0 {
                                self.image_data[self.width * (self.height - 1 - y) + (x.round() as usize)] = color;
                            } else {
                                let ind: usize = self.width * (self.height - 1 - y) + (x.round() as usize);
                                for channel in 0..color.len() {
                                    self.image_data[ind][channel] = ((self.image_data[ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
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
                                self.image_data[pix1_ind][channel] = ((self.image_data[pix1_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[pix2_ind][channel] = ((self.image_data[pix2_ind][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;
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

        // if opacity is 0.0, or less, then rectangle is transparent, nothing is to be drawn.
        if opacity >= 0.0 {

            // find corners
            let mut smaller_x = min(x1, x2);
            let mut bigger_x = max(x1, x2);
            let mut smaller_y = min(y1, y2);
            let mut bigger_y = max(y1, y2);
            if bigger_x >= self.width {
                bigger_x = self.width - 1;
            }
            if bigger_y >= self.height {
                bigger_y = self.height - 1;
            }

            if thickness == 0 {
                if opacity >= 1.0 {
                    // Draw filled, solid rectangle.
                    // draws line by line
                    for y in smaller_y..(bigger_y + 1) {
                        let base_location = self.width * (self.height - 1 - y);
                        self.image_data[(base_location + smaller_x)..(base_location + bigger_x + 1)].fill(color);
                    }
                } else {
                    // Draw filled, transparent rectangle.
                    // draws each pixel by blending it to the background (because of transparency)
                    let reverse_opacity = 1.0 - opacity;
                    for y in smaller_y..(bigger_y + 1) {
                        let base_location = self.width * (self.height - 1 - y);  // base index of line
                        for x in (base_location + smaller_x)..(base_location + bigger_x + 1) { // range of indexes on that line (horizontal line)
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.image_data[x][channel] = ((self.image_data[x][channel] as f64) * reverse_opacity + (color[channel] as f64) * opacity).round() as u8;
                            }
                        }
                    }
                }
            } else {
                if opacity >= 1.0 {
                    // Draw rectangle, solid

                    let mut used_thickness = thickness;  // new thickness variable
                    // limits maximum thickness
                    let limit_x = ((bigger_x - smaller_x) / 2) + 1;
                    let limit_y = ((bigger_y - smaller_y) / 2) + 1;
                    if (thickness > limit_x) || (thickness > limit_y) {
                        used_thickness = min(limit_x, limit_y);
                    }

                    // draw smaller and smaller rectangles until given thickness is achieved
                    while used_thickness > 0 {
                        used_thickness -= 1;

                        // draw horizontal sides
                        self.image_data[(self.width * (self.height - 1 - smaller_y) + smaller_x)..(self.width * (self.height - 1 - smaller_y) + bigger_x + 1)].fill(color);
                        self.image_data[(self.width * (self.height - 1 - bigger_y) + smaller_x)..(self.width * (self.height - 1 - bigger_y) + bigger_x + 1)].fill(color);
                        // draw vertical sides
                        for y in (smaller_y + 1)..bigger_y {
                            let base_location = self.width * (self.height - 1 - y);
                            self.image_data[base_location + smaller_x] = color;
                            self.image_data[base_location + bigger_x] = color;
                        }

                        smaller_x += 1;
                        smaller_y += 1;
                        bigger_x -= 1;
                        bigger_y -= 1;
                    }
                } else {
                    // Draw rectangle, transparent

                    let mut used_thickness = thickness; // new variable used for thickness
                    // limits maximum thickness
                    let limit_x = ((bigger_x - smaller_x) / 2) + 1;
                    let limit_y = ((bigger_y - smaller_y) / 2) + 1;
                    if (thickness > limit_x) || (thickness > limit_y) {
                        used_thickness = min(limit_x, limit_y);
                    }

                    let reverse_opacity = 1.0 - opacity;  // no explicit meaning, just used as a value when blending (to reduce unnecessary calculations)

                    while used_thickness > 0 {
                        used_thickness -= 1;

                        // draw horizontal sides
                        for y in [smaller_y, bigger_y] {  // bottom and top side
                            let base_location = self.width * (self.height - 1 - y);  // starting index of line where sides are
                            for x in (base_location + smaller_x)..(base_location + bigger_x + 1) {  // pixels in those sides
                                for channel in 0..color.len() {
                                    // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                    self.image_data[x][channel] = ((self.image_data[x][channel] as f64) * reverse_opacity + (color[channel] as f64) * opacity).round() as u8;
                                }
                            }
                        }

                        // draw vertical sides
                        for y in (smaller_y + 1)..bigger_y {
                            let base_location = self.width * (self.height - 1 - y);
                            for x in [smaller_x, bigger_x] {
                                let ind_location = base_location + x;
                                for channel in 0..color.len() {
                                    // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                    self.image_data[ind_location][channel] = ((self.image_data[ind_location][channel] as f64) * reverse_opacity + (color[channel] as f64) * opacity).round() as u8;
                                }
                            }
                        }

                        smaller_x += 1;
                        smaller_y += 1;
                        bigger_x -= 1;
                        bigger_y -= 1;
                    }
                }
            }
        }
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
                                self.image_data[self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord] = color;
                                self.image_data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord] = color;
                            } else if y_coord.round() as usize > y_upperlimit {
                                self.image_data[(self.width * (self.height - 1 - (y_coord.round() as usize)) + (2 * x - x_coord))..(self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord + 1)].fill(color);
                                self.image_data[(self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + (2 * x - x_coord))..(self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord + 1)].fill(color);
                                previous_y = y_coord.round() as usize;
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = y_coord - y_coord.floor();
                            let pix2_percentage: f64 = 1.0 - pix1_percentage;
                            let pix1_ind: usize = self.width * (self.height - 1 - (y_coord.ceil() as usize)) + x_coord;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.image_data[pix1_ind][channel] = ((self.image_data[pix1_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.image_data[pix1_ind - 2 * (x_coord - x)][channel] = ((self.image_data[pix1_ind - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] = ((self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                            }
                            if (y_coord.ceil() as usize) < previous_y {
                                self.image_data[(pix1_ind - 2 * (x_coord - x) + 1)..pix1_ind].fill(color);
                                self.image_data[(pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x) + 1)..(pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width)].fill(color);
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
                                self.image_data[(self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize))..(self.width * (self.height - 1 - y_coord) + x_coord.round() as usize + 1)].fill(color);
                            } else {
                                self.image_data[(self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize))..(self.width * (self.height - 1 - y_coord) + (x_coord.round() as usize) + 1)].fill(color);
                                self.image_data[(self.width * (self.height - 1 - (2 * y - y_coord)) + (2 * x - x_coord.round() as usize))..(self.width * (self.height - 1 - (2 * y - y_coord)) + (x_coord.round() as usize) + 1)].fill(color);
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = x_coord - x_coord.floor();
                            let pix2_percentage: f64 = 1.0 - pix1_percentage;
                            let pix_ind: usize = self.width * (self.height - 1 - y_coord) + x_coord.ceil() as usize;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.image_data[pix_ind][channel] = ((self.image_data[pix_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                temp_ind = pix_ind + 2 * (y_coord - y) * self.width;
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                temp_ind -= 2 * (x_coord.ceil() as usize - x);
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                            }
                            self.image_data[(pix_ind - 2 * (x_coord.ceil() as usize - x) + 1)..pix_ind].fill(color);
                            self.image_data[(pix_ind + 2 * (y_coord - y) * self.width - 2 * (x_coord.ceil() as usize - x) + 1)..(pix_ind + 2 * (y_coord - y) * self.width)].fill(color);
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
                                    self.image_data[self.width * (self.height - 1 - y_coord.round() as usize) + x_coord][channel] = ((self.image_data[self.width * (self.height - 1 - y_coord.round() as usize) + x_coord][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    self.image_data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord][channel] = ((self.image_data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                }
                            } else if y_coord.round() as usize > y_upperlimit {
                                for pixel_ind in (self.width * (self.height - 1 - (y_coord.round() as usize)) + (2 * x - x_coord))..(self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord + 1) {
                                    for channel in 0..color.len() {
                                        self.image_data[pixel_ind][channel] = ((self.image_data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    }
                                }
                                for pixel_ind in (self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + (2 * x - x_coord))..(self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord + 1) {
                                    for channel in 0..color.len() {
                                        self.image_data[pixel_ind][channel] = ((self.image_data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
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
                                self.image_data[pix1_ind][channel] = ((self.image_data[pix1_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.image_data[pix1_ind - 2 * (x_coord - x)][channel] = ((self.image_data[pix1_ind - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] = ((self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                            }
                            if (y_coord.ceil() as usize) < previous_y {
                                for pixel_ind in (pix1_ind - 2 * (x_coord - x) + 1)..pix1_ind {
                                    for channel in 0..color.len() {
                                        self.image_data[pixel_ind][channel] = ((self.image_data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    }
                                }
                                for pixel_ind in (pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x) + 1)..(pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width) {
                                    for channel in 0..color.len() {
                                        self.image_data[pixel_ind][channel] = ((self.image_data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
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
                                        self.image_data[pixel_ind][channel] = ((self.image_data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    }
                                }
                            } else {
                                for pixel_ind in (self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize))..(self.width * (self.height - 1 - y_coord) + x_coord.round() as usize + 1) {
                                    for channel in 0..color.len() {
                                        self.image_data[pixel_ind][channel] = ((self.image_data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    }
                                }
                                for pixel_ind in (self.width * (self.height - 1 - (2 * y - y_coord)) + (2 * x - x_coord.round() as usize))..(self.width * (self.height - 1 - (2 * y - y_coord)) + x_coord.round() as usize + 1) {
                                    for channel in 0..color.len() {
                                        self.image_data[pixel_ind][channel] = ((self.image_data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
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
                                self.image_data[pix_ind][channel] = ((self.image_data[pix_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                temp_ind = pix_ind + 2 * (y_coord - y) * self.width;
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                temp_ind -= 2 * (x_coord.ceil() as usize - x);
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                            }
                            for pixel_ind in (pix_ind - 2 * (x_coord.ceil() as usize - x) + 1)..pix_ind {
                                for channel in 0..color.len() {
                                    self.image_data[pixel_ind][channel] = ((self.image_data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                }
                            }
                            for pixel_ind in (pix_ind + 2 * (y_coord - y) * self.width - 2 * (x_coord.ceil() as usize - x) + 1)..(pix_ind + 2 * (y_coord - y) * self.width) {
                                for channel in 0..color.len() {
                                    self.image_data[pixel_ind][channel] = ((self.image_data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
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
                            self.image_data[self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord] = color;
                            self.image_data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord] = color;
                            if x_coord != x {
                                // new x coord (mirrored to left) ===> x_coord = x - (x_coord - x)
                                self.image_data[self.width * (self.height - 1 - (y_coord.round() as usize)) + (2 * x - x_coord)] = color;
                                self.image_data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + (2 * x - x_coord)] = color;
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = y_coord - y_coord.floor();
                            let pix2_percentage: f64 = 1.0 - pix1_percentage;
                            let pix1_ind: usize = self.width * (self.height - 1 - (y_coord.ceil() as usize)) + x_coord;
                            let pix2_ind: usize = pix1_ind + self.width;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.image_data[pix1_ind][channel] = ((self.image_data[pix1_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[pix2_ind][channel] = ((self.image_data[pix2_ind][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.image_data[pix1_ind - 2 * (x_coord - x)][channel] = ((self.image_data[pix1_ind - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[pix2_ind - 2 * (x_coord - x)][channel] = ((self.image_data[pix2_ind - 2 * (x_coord - x)][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] = ((self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width][channel] = ((self.image_data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.image_data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                            }
                        }
                    }

                    // for every y, calculate x and split between left and right pixel (up to y_upperlimit - 1 as it is a special case handled separately)
                    for y_coord in y..y_upperlimit {
                        let x_coord: f64 = x0 + (radius_sqrd - (y_coord as f64 - y0).powi(2)).sqrt();

                        if (x_coord - x_coord.round()).abs() < 0.00001 {
                            // if point is very close to integer, just draw it on that pixel, and mirror to other 3 symmetric pixels
                            self.image_data[self.width * (self.height - 1 - y_coord) + x_coord.round() as usize] = color;
                            self.image_data[self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize)] = color;
                            if y_coord != y {
                                // new x coord (mirrored to left) ===> x_coord = x - (x_coord - x)
                                self.image_data[self.width * (self.height - 1 - (2 * y - y_coord)) + x_coord.round() as usize] = color;
                                self.image_data[self.width * (self.height - 1 - (2 * y - y_coord)) + (2 * x - x_coord.round() as usize)] = color;
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = x_coord - x_coord.floor();
                            let pix2_percentage: f64 = 1.0 - pix1_percentage;
                            let pix_ind: usize = self.width * (self.height - 1 - y_coord) + x_coord.ceil() as usize;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.image_data[pix_ind][channel] = ((self.image_data[pix_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[pix_ind - 1][channel] = ((self.image_data[pix_ind - 1][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[temp_ind + 1][channel] = ((self.image_data[temp_ind + 1][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                temp_ind = pix_ind + 2 * (y_coord - y) * self.width;
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[temp_ind - 1][channel] = ((self.image_data[temp_ind - 1][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                temp_ind -= 2 * (x_coord.ceil() as usize - x);
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[temp_ind + 1][channel] = ((self.image_data[temp_ind + 1][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

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
                        self.image_data[pix_ind][channel] = ((self.image_data[pix_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                        self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        temp_ind = pix_ind + 2 * (y_upperlimit - y) * self.width;
                        self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        temp_ind -= 2 * (x_coord.ceil() as usize - x);
                        self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
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
                                self.image_data[self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord][channel] = ((self.image_data[self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                self.image_data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord][channel] = ((self.image_data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                            }
                            if x_coord != x {
                                // new x coord (mirrored to left) ===> x_coord = x - (x_coord - x)
                                for channel in 0..color.len() {
                                    self.image_data[self.width * (self.height - 1 - (y_coord.round() as usize)) + (2 * x - x_coord)][channel] = ((self.image_data[self.width * (self.height - 1 - (y_coord.round() as usize)) + (2 * x - x_coord)][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    self.image_data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + (2 * x - x_coord)][channel] = ((self.image_data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + (2 * x - x_coord)][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
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
                                self.image_data[pix1_ind][channel] = ((self.image_data[pix1_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[pix2_ind][channel] = ((self.image_data[pix2_ind][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.image_data[pix1_ind - 2 * (x_coord - x)][channel] = ((self.image_data[pix1_ind - 2 * (x_coord - x)][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[pix2_ind - 2 * (x_coord - x)][channel] = ((self.image_data[pix2_ind - 2 * (x_coord - x)][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] = ((self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width][channel] = ((self.image_data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.image_data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                            }
                        }
                    }

                    // for every y, calculate x and split between left and right pixel (up to y_upperlimit - 1 as it is a special case handled separately)
                    for y_coord in y..y_upperlimit {
                        let x_coord: f64 = x0 + (radius_sqrd - (y_coord as f64 - y0).powi(2)).sqrt();

                        if (x_coord - x_coord.round()).abs() < 0.00001 {
                            // if point is very close to integer, just draw it on that pixel, and mirror to other 3 symmetric pixels
                            for channel in 0..color.len() {
                                self.image_data[self.width * (self.height - 1 - y_coord) + x_coord.round() as usize][channel] = ((self.image_data[self.width * (self.height - 1 - y_coord) + x_coord.round() as usize][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                self.image_data[self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize)][channel] = ((self.image_data[self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize)][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                            }
                            if y_coord != y {
                                // new x coord (mirrored to left) ===> x_coord = x - (x_coord - x)
                                for channel in 0..color.len() {
                                    self.image_data[self.width * (self.height - 1 - (2 * y - y_coord)) + x_coord.round() as usize][channel] = ((self.image_data[self.width * (self.height - 1 - (2 * y - y_coord)) + x_coord.round() as usize][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    self.image_data[self.width * (self.height - 1 - (2 * y - y_coord)) + (2 * x - x_coord.round() as usize)][channel] = ((self.image_data[self.width * (self.height - 1 - (2 * y - y_coord)) + (2 * x - x_coord.round() as usize)][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                }
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = (x_coord - x_coord.floor()) * opacity;
                            let pix2_percentage: f64 = (x_coord.ceil() - x_coord) * opacity;
                            let pix_ind: usize = self.width * (self.height - 1 - y_coord) + x_coord.ceil() as usize;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.image_data[pix_ind][channel] = ((self.image_data[pix_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[pix_ind - 1][channel] = ((self.image_data[pix_ind - 1][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[temp_ind + 1][channel] = ((self.image_data[temp_ind + 1][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                temp_ind = pix_ind + 2 * (y_coord - y) * self.width;
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[temp_ind - 1][channel] = ((self.image_data[temp_ind - 1][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                temp_ind -= 2 * (x_coord.ceil() as usize - x);
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[temp_ind + 1][channel] = ((self.image_data[temp_ind + 1][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                            }
                        }
                    }

                    // special case when y = y_upperlimit, draw only outer pixel (as inner was already filled with something in x loop)
                    let x_coord: f64 = x0 + (radius_sqrd - (y_upperlimit as f64 - y0).powi(2)).sqrt();
                    let pix1_percentage: f64 = (x_coord - x_coord.floor()) * opacity;
                    let pix_ind: usize = self.width * (self.height - 1 - y_upperlimit) + x_coord.ceil() as usize;
                    for channel in 0..color.len() {
                        // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                        self.image_data[pix_ind][channel] = ((self.image_data[pix_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                        self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        temp_ind = pix_ind + 2 * (y_upperlimit - y) * self.width;
                        self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        temp_ind -= 2 * (x_coord.ceil() as usize - x);
                        self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
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
                                self.image_data[self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord] = color;
                                self.image_data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord] = color;
                            } else if y_coord.round() as usize > y_upperlimit {
                                self.image_data[(self.width * (self.height - 1 - (y_coord.round() as usize)) + (2 * x - x_coord))..(self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord + 1)].fill(color);
                                self.image_data[(self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + (2 * x - x_coord))..(self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord + 1)].fill(color);
                                previous_y = y_coord.round() as usize;
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = y_coord - y_coord.floor();
                            let pix2_percentage: f64 = 1.0 - pix1_percentage;
                            let pix1_ind: usize = self.width * (self.height - 1 - (y_coord.ceil() as usize)) + x_coord;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.image_data[pix1_ind][channel] = ((self.image_data[pix1_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.image_data[pix1_ind - 2 * (x_coord - x)][channel] = ((self.image_data[pix1_ind - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] = ((self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                            }
                            if (y_coord.ceil() as usize) < previous_y {
                                self.image_data[(pix1_ind - 2 * (x_coord - x) + 1)..pix1_ind].fill(color);
                                self.image_data[(pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x) + 1)..(pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width)].fill(color);
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
                                self.image_data[(self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize))..(self.width * (self.height - 1 - y_coord) + x_coord.round() as usize + 1)].fill(color);
                            } else {
                                self.image_data[(self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize))..(self.width * (self.height - 1 - y_coord) + (x_coord.round() as usize) + 1)].fill(color);
                                self.image_data[(self.width * (self.height - 1 - (2 * y - y_coord)) + (2 * x - x_coord.round() as usize))..(self.width * (self.height - 1 - (2 * y - y_coord)) + (x_coord.round() as usize) + 1)].fill(color);
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = x_coord - x_coord.floor();
                            let pix2_percentage: f64 = 1.0 - pix1_percentage;
                            let pix_ind: usize = self.width * (self.height - 1 - y_coord) + x_coord.ceil() as usize;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.image_data[pix_ind][channel] = ((self.image_data[pix_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                temp_ind = pix_ind + 2 * (y_coord - y) * self.width;
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                temp_ind -= 2 * (x_coord.ceil() as usize - x);
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                            }
                            self.image_data[(pix_ind - 2 * (x_coord.ceil() as usize - x) + 1)..pix_ind].fill(color);
                            self.image_data[(pix_ind + 2 * (y_coord - y) * self.width - 2 * (x_coord.ceil() as usize - x) + 1)..(pix_ind + 2 * (y_coord - y) * self.width)].fill(color);
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
                                    self.image_data[self.width * (self.height - 1 - y_coord.round() as usize) + x_coord][channel] = ((self.image_data[self.width * (self.height - 1 - y_coord.round() as usize) + x_coord][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    self.image_data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord][channel] = ((self.image_data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                }
                            } else if y_coord.round() as usize > y_upperlimit {
                                for pixel_ind in (self.width * (self.height - 1 - (y_coord.round() as usize)) + (2 * x - x_coord))..(self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord + 1) {
                                    for channel in 0..color.len() {
                                        self.image_data[pixel_ind][channel] = ((self.image_data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    }
                                }
                                for pixel_ind in (self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + (2 * x - x_coord))..(self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord + 1) {
                                    for channel in 0..color.len() {
                                        self.image_data[pixel_ind][channel] = ((self.image_data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
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
                                self.image_data[pix1_ind][channel] = ((self.image_data[pix1_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.image_data[pix1_ind - 2 * (x_coord - x)][channel] = ((self.image_data[pix1_ind - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] = ((self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                            }
                            if (y_coord.ceil() as usize) < previous_y {
                                for pixel_ind in (pix1_ind - 2 * (x_coord - x) + 1)..pix1_ind {
                                    for channel in 0..color.len() {
                                        self.image_data[pixel_ind][channel] = ((self.image_data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    }
                                }
                                for pixel_ind in (pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x) + 1)..(pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width) {
                                    for channel in 0..color.len() {
                                        self.image_data[pixel_ind][channel] = ((self.image_data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
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
                                        self.image_data[pixel_ind][channel] = ((self.image_data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    }
                                }
                            } else {
                                for pixel_ind in (self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize))..(self.width * (self.height - 1 - y_coord) + x_coord.round() as usize + 1) {
                                    for channel in 0..color.len() {
                                        self.image_data[pixel_ind][channel] = ((self.image_data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    }
                                }
                                for pixel_ind in (self.width * (self.height - 1 - (2 * y - y_coord)) + (2 * x - x_coord.round() as usize))..(self.width * (self.height - 1 - (2 * y - y_coord)) + x_coord.round() as usize + 1) {
                                    for channel in 0..color.len() {
                                        self.image_data[pixel_ind][channel] = ((self.image_data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
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
                                self.image_data[pix_ind][channel] = ((self.image_data[pix_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                temp_ind = pix_ind + 2 * (y_coord - y) * self.width;
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                                temp_ind -= 2 * (x_coord.ceil() as usize - x);
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                            }
                            for pixel_ind in (pix_ind - 2 * (x_coord.ceil() as usize - x) + 1)..pix_ind {
                                for channel in 0..color.len() {
                                    self.image_data[pixel_ind][channel] = ((self.image_data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                }
                            }
                            for pixel_ind in (pix_ind + 2 * (y_coord - y) * self.width - 2 * (x_coord.ceil() as usize - x) + 1)..(pix_ind + 2 * (y_coord - y) * self.width) {
                                for channel in 0..color.len() {
                                    self.image_data[pixel_ind][channel] = ((self.image_data[pixel_ind][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
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
                            self.image_data[self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord] = color;
                            self.image_data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord] = color;
                            if x_coord != x {
                                // new x coord (mirrored to left) ===> x_coord = x - (x_coord - x)
                                self.image_data[self.width * (self.height - 1 - (y_coord.round() as usize)) + (2 * x - x_coord)] = color;
                                self.image_data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + (2 * x - x_coord)] = color;
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = y_coord - y_coord.floor();
                            let pix2_percentage: f64 = 1.0 - pix1_percentage;
                            let pix1_ind: usize = self.width * (self.height - 1 - (y_coord.ceil() as usize)) + x_coord;
                            let pix2_ind: usize = pix1_ind + self.width;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.image_data[pix1_ind][channel] = ((self.image_data[pix1_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[pix2_ind][channel] = ((self.image_data[pix2_ind][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.image_data[pix1_ind - 2 * (x_coord - x)][channel] = ((self.image_data[pix1_ind - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[pix2_ind - 2 * (x_coord - x)][channel] = ((self.image_data[pix2_ind - 2 * (x_coord - x)][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] = ((self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width][channel] = ((self.image_data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.image_data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                            }
                        }
                    }

                    // for every y, calculate x and split between left and right pixel (up to y_upperlimit - 1 as it is a special case handled separately)
                    for y_coord in y..y_upperlimit {
                        let x_coord: f64 = x0 + (horizontal_axis as f64) * (1.0 - ((y_coord as f64 - y0) / vertical_axis as f64).powi(2)).sqrt();

                        if (x_coord - x_coord.round()).abs() < 0.00001 {
                            // if point is very close to integer, just draw it on that pixel, and mirror to other 3 symmetric pixels
                            self.image_data[self.width * (self.height - 1 - y_coord) + x_coord.round() as usize] = color;
                            self.image_data[self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize)] = color;
                            if y_coord != y {
                                // new x coord (mirrored to left) ===> x_coord = x - (x_coord - x)
                                self.image_data[self.width * (self.height - 1 - (2 * y - y_coord)) + x_coord.round() as usize] = color;
                                self.image_data[self.width * (self.height - 1 - (2 * y - y_coord)) + (2 * x - x_coord.round() as usize)] = color;
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = x_coord - x_coord.floor();
                            let pix2_percentage: f64 = 1.0 - pix1_percentage;
                            let pix_ind: usize = self.width * (self.height - 1 - y_coord) + x_coord.ceil() as usize;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.image_data[pix_ind][channel] = ((self.image_data[pix_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[pix_ind - 1][channel] = ((self.image_data[pix_ind - 1][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[temp_ind + 1][channel] = ((self.image_data[temp_ind + 1][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                temp_ind = pix_ind + 2 * (y_coord - y) * self.width;
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[temp_ind - 1][channel] = ((self.image_data[temp_ind - 1][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                temp_ind -= 2 * (x_coord.ceil() as usize - x);
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[temp_ind + 1][channel] = ((self.image_data[temp_ind + 1][channel] as f64) * (pix1_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

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
                        self.image_data[pix_ind][channel] = ((self.image_data[pix_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                        self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        temp_ind = pix_ind + 2 * (y_upperlimit - y) * self.width;
                        self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        temp_ind -= 2 * (x_coord.ceil() as usize - x);
                        self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (pix2_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
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
                                self.image_data[self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord][channel] = ((self.image_data[self.width * (self.height - 1 - (y_coord.round() as usize)) + x_coord][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                            }
                            if y_coord.round() as usize != y {
                                for channel in 0..color.len() {
                                    self.image_data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord][channel] = ((self.image_data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + x_coord][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                }
                            }
                            if x_coord != x {
                                // new x coord (mirrored to left) ===> x_coord = x - (x_coord - x)
                                for channel in 0..color.len() {
                                    self.image_data[self.width * (self.height - 1 - (y_coord.round() as usize)) + (2 * x - x_coord)][channel] = ((self.image_data[self.width * (self.height - 1 - (y_coord.round() as usize)) + (2 * x - x_coord)][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                }
                                if y_coord.round() as usize != y {
                                    for channel in 0..color.len() {
                                        self.image_data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + (2 * x - x_coord)][channel] = ((self.image_data[self.width * (self.height - 1 - (2 * y - (y_coord.round() as usize))) + (2 * x - x_coord)][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
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
                                self.image_data[pix1_ind][channel] = ((self.image_data[pix1_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[pix2_ind][channel] = ((self.image_data[pix2_ind][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.image_data[pix1_ind - 2 * (x_coord - x)][channel] = ((self.image_data[pix1_ind - 2 * (x_coord - x)][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[pix2_ind - 2 * (x_coord - x)][channel] = ((self.image_data[pix2_ind - 2 * (x_coord - x)][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] = ((self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width][channel] = ((self.image_data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.image_data[pix1_ind + 2 * (y_coord.ceil() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width - 2 * (x_coord - x)][channel] = ((self.image_data[pix2_ind + 2 * (y_coord.floor() as usize - y) * self.width - 2 * (x_coord - x)][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                            }
                        }
                    }

                    // for every y, calculate x and split between left and right pixel (up to y_upperlimit - 1 as it is a special case handled separately)
                    for y_coord in y..y_upperlimit {
                        let x_coord: f64 = x0 + (horizontal_axis as f64) * (1.0 - ((y_coord as f64 - y0) / vertical_axis as f64).powi(2)).sqrt();

                        if (x_coord - x_coord.round()).abs() < 0.00001 {
                            // if point is very close to integer, just draw it on that pixel, and mirror to other 3 symmetric pixels
                            for channel in 0..color.len() {
                                self.image_data[self.width * (self.height - 1 - y_coord) + x_coord.round() as usize][channel] = ((self.image_data[self.width * (self.height - 1 - y_coord) + x_coord.round() as usize][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                self.image_data[self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize)][channel] = ((self.image_data[self.width * (self.height - 1 - y_coord) + (2 * x - x_coord.round() as usize)][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                            }
                            if y_coord != y {
                                // new x coord (mirrored to left) ===> x_coord = x - (x_coord - x)
                                for channel in 0..color.len() {
                                    self.image_data[self.width * (self.height - 1 - (2 * y - y_coord)) + x_coord.round() as usize][channel] = ((self.image_data[self.width * (self.height - 1 - (2 * y - y_coord)) + x_coord.round() as usize][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                    self.image_data[self.width * (self.height - 1 - (2 * y - y_coord)) + (2 * x - x_coord.round() as usize)][channel] = ((self.image_data[self.width * (self.height - 1 - (2 * y - y_coord)) + (2 * x - x_coord.round() as usize)][channel] as f64) * (1.0 - opacity) + (color[channel] as f64) * opacity).round() as u8;
                                }
                            }
                        } else {
                            // split point between two pixels, and mirror to other 3 symmetric pixels
                            let pix1_percentage: f64 = (x_coord - x_coord.floor()) * opacity;
                            let pix2_percentage: f64 = (x_coord.ceil() - x_coord) * opacity;
                            let pix_ind: usize = self.width * (self.height - 1 - y_coord) + x_coord.ceil() as usize;
                            for channel in 0..color.len() {
                                // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                                self.image_data[pix_ind][channel] = ((self.image_data[pix_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[pix_ind - 1][channel] = ((self.image_data[pix_ind - 1][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[temp_ind + 1][channel] = ((self.image_data[temp_ind + 1][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                temp_ind = pix_ind + 2 * (y_coord - y) * self.width;
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[temp_ind - 1][channel] = ((self.image_data[temp_ind - 1][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                                temp_ind -= 2 * (x_coord.ceil() as usize - x);
                                self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                                self.image_data[temp_ind + 1][channel] = ((self.image_data[temp_ind + 1][channel] as f64) * (1.0 - pix2_percentage) + (color[channel] as f64) * pix2_percentage).round() as u8;

                            }
                        }
                    }

                    // special case when y = y_upperlimit, draw only outer pixel (as inner was already filled with something in x loop)
                    let x_coord: f64 = x0 + (horizontal_axis as f64) * (1.0 - ((y_upperlimit as f64 - y0) / vertical_axis as f64).powi(2)).sqrt();
                    let pix1_percentage: f64 = (x_coord - x_coord.floor()) * opacity;
                    let pix_ind: usize = self.width * (self.height - 1 - y_upperlimit) + x_coord.ceil() as usize;
                    for channel in 0..color.len() {
                        // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage
                        self.image_data[pix_ind][channel] = ((self.image_data[pix_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        let mut temp_ind = pix_ind - 2 * (x_coord.ceil() as usize - x);
                        self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        temp_ind = pix_ind + 2 * (y_upperlimit - y) * self.width;
                        self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;

                        temp_ind -= 2 * (x_coord.ceil() as usize - x);
                        self.image_data[temp_ind][channel] = ((self.image_data[temp_ind][channel] as f64) * (1.0 - pix1_percentage) + (color[channel] as f64) * pix1_percentage).round() as u8;
                    }
                }
            }
        }
    }
}

//! A module that contains the [Image] struct and related functions.


// standard library imports
use std::{
    cmp::{min, max},
    error::Error,
    f64::consts::FRAC_1_SQRT_2,
    fmt::Display,
    ops::{Bound, RangeBounds},
    ptr::slice_from_raw_parts,
};

// standard library imports when file_io feature is enabled
#[cfg(feature = "file_io")]
use std::{
    fs::remove_file,
    io::ErrorKind,
    path::Path,
};

// external library imports when file_io feature is enabled
#[cfg(feature = "file_io")]
use image_io::{
    ColorType,
    DynamicImage,
    save_buffer,
    error::ImageError,
    io::Reader as ImageReader,
};





/// A struct that holds an image
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Image {
    /// The image pixel data
    data: Vec<u8>,
    /// The width of the image
    width: usize,
    /// The height of the image
    height: usize,
    /// The type of the image
    image_type: ImageType,
    /// The background of the image
    background_data: BackgroundData
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let img_size = match self.image_type {
            ImageType::GRAY8 => self.data.len(),
            ImageType::GRAYA8 => self.data.len(),
            ImageType::GRAY16 => self.data.len() * 2,
            ImageType::GRAYA16 => self.data.len() * 2,
            ImageType::RGB8 => self.data.len(),
            ImageType::RGBA8 => self.data.len(),
            ImageType::RGB16 => self.data.len() * 2,
            ImageType::RGBA16 => self.data.len() * 2,
        };
        write!(f, "Image:\n   - dimensions: {}x{}\n   - type: {}   - size: {} bytes", self.width, self.height, self.image_type, img_size)
    }
}





/// An enum that holds the background information for [Image]
#[derive(Debug, Clone, Eq, PartialEq)]
enum BackgroundData {
    /// The background is a color
    Color(Colors),
    /// The background is an image
    Image(Vec<u8>)
}

/// An enum that holds the
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Colors {
    /// The 8-bit grayscale color
    GRAY8(u8),
    /// The 8-bit grayscale color + 8-bit alpha channel
    GRAYA8([u8; 2]),
    /// The 16-bit grayscale color
    GRAY16(u16),
    /// The 16-bit grayscale color + 16-bit alpha channel
    GRAYA16([u16; 2]),
    /// The 8-bit RGB color
    RGB8([u8; 3]),
    /// The 8-bit RGB color + 8-bit alpha channel
    RGBA8([u8; 4]),
    /// The 16-bit RGB color
    RGB16([u16; 3]),
    /// The 16-bit RGB color + 16-bit alpha channel
    RGBA16([u16; 4]),
}

/// An enum that holds the image type information
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ImageType {
    /// An image with 8-bit grayscale pixels
    GRAY8,
    /// An image with 8-bit grayscale pixels + 8-bit alpha channel
    GRAYA8,
    /// An image with 16-bit grayscale pixels
    GRAY16,
    /// An image with 16-bit grayscale pixels + 16-bit alpha channel
    GRAYA16,
    /// An image with 8-bit RGB pixels
    RGB8,
    /// An image with 8-bit RGB pixels + 8-bit alpha channel
    RGBA8,
    /// An image with 16-bit RGB pixels
    RGB16,
    /// An image with 16-bit RGB pixels + 16-bit alpha channel
    RGBA16,
}

/// An enum that holds the error information for [Drawing] trait
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DrawingError {
    /// The given color is wrong
    WrongColor,
    /// The invalid opacity value
    InvalidOpacity,
}

/// An enum that holds the error information for [Indexing] trait
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IndexingError {
    /// The invalid opacity value
    InvalidOpacity,
    /// The index is out of bounds
    OutOfBounds,
    /// The given color is wrong
    WrongColor,
}

/// An enum that holds the error information for [IO] trait
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IOError {
    /// Error while decoding the image
    Decoding,
    /// Error while encoding the image
    Encoding,
    /// The file already exists
    FileExists,
    /// The file was not found
    FileNotFound,
    /// The file is not an image
    InvalidData,
    /// The size of the image is invalid
    InvalidSize,
    /// The unsupported type
    InvalidType,
    /// The file could not be opened because of permissions
    NoPermission,
    /// The unknown error
    Unknown,
    /// The file can't be written
    WriteError,
}


impl Colors {

    #[inline]
    fn bytes_per_pixel(&self) -> usize {
        //! Returns the number of bytes per pixel

        match self {
            Colors::GRAY8(_) => 1,
            Colors::GRAYA8(_) => 2,
            Colors::GRAY16(_) => 2,
            Colors::GRAYA16(_) => 4,
            Colors::RGB8(_) => 3,
            Colors::RGBA8(_) => 4,
            Colors::RGB16(_) => 6,
            Colors::RGBA16(_) => 8,
        }
    }
}

impl ImageType {

    #[inline]
    fn bytes_per_pixel(&self) -> usize {
        //! Returns the number of bytes per pixel

        match self {
            ImageType::GRAY8 => 1,
            ImageType::GRAYA8 => 2,
            ImageType::GRAY16 => 2,
            ImageType::GRAYA16 => 4,
            ImageType::RGB8 => 3,
            ImageType::RGBA8 => 4,
            ImageType::RGB16 => 6,
            ImageType::RGBA16 => 8,
        }
    }
}

impl Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Colors::GRAY8(value) => write!(f, "GRAY8({:?})", value),
            Colors::GRAYA8(value) => write!(f, "GRAYA8({:?})", value),
            Colors::GRAY16(value) => write!(f, "GRAY16({:?})", value),
            Colors::GRAYA16(value) => write!(f, "GRAYA16({:?})", value),
            Colors::RGB8(value) => write!(f, "RGB8({:?})", value),
            Colors::RGBA8(value) => write!(f, "RGBA8({:?})", value),
            Colors::RGB16(value) => write!(f, "RGB16({:?})", value),
            Colors::RGBA16(value) => write!(f, "RGBA16({:?})", value),
        }
    }
}

impl Display for ImageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageType::GRAY8 => write!(f, "GRAY8"),
            ImageType::GRAYA8 => write!(f, "GRAYA8"),
            ImageType::GRAY16 => write!(f, "GRAY16"),
            ImageType::GRAYA16 => write!(f, "GRAYA16"),
            ImageType::RGB8 => write!(f, "RGB8"),
            ImageType::RGBA8 => write!(f, "RGBA8"),
            ImageType::RGB16 => write!(f, "RGB16"),
            ImageType::RGBA16 => write!(f, "RGBA16"),
        }
    }
}

impl Display for DrawingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DrawingError::WrongColor => write!(f, "DrawingError: Wrong color!"),
            DrawingError::InvalidOpacity => write!(f, "DrawingError: Invalid opacity value!"),
        }
    }
}

impl Display for IndexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexingError::InvalidOpacity => write!(f, "IndexingError: Invalid opacity value!"),
            IndexingError::OutOfBounds => write!(f, "IndexingError: Index out of bounds!"),
            IndexingError::WrongColor => write!(f, "IndexingError: Wrong color!"),
        }
    }
}

impl Display for IOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IOError::Decoding => write!(f, "IOError: Error while decoding the image!"),
            IOError::Encoding => write!(f, "IOError: Error while encoding the image!"),
            IOError::FileExists => write!(f, "IOError: File already exists!"),
            IOError::FileNotFound => write!(f, "IOError: File not found!"),
            IOError::InvalidData => write!(f, "IOError: Invalid data!"),
            IOError::InvalidSize => write!(f, "IOError: The size of the image is invalid!"),
            IOError::InvalidType => write!(f, "IOError: The unsupported type!"),
            IOError::NoPermission => write!(f, "IOError: Can't be completed because of denied permission."),
            IOError::Unknown => write!(f, "IOError: Unknown error!"),
            IOError::WriteError => write!(f, "IOError: Can't write to file!"),
        }
    }
}

impl Error for DrawingError {}

impl Error for IndexingError {}

impl Error for IOError {}

impl From<Colors> for ImageType {
    fn from(color: Colors) -> Self {
        match color {
            Colors::GRAY8(_) => ImageType::GRAY8,
            Colors::GRAYA8(_) => ImageType::GRAYA8,
            Colors::GRAY16(_) => ImageType::GRAY16,
            Colors::GRAYA16(_) => ImageType::GRAYA16,
            Colors::RGB8(_) => ImageType::RGB8,
            Colors::RGBA8(_) => ImageType::RGBA8,
            Colors::RGB16(_) => ImageType::RGB16,
            Colors::RGBA16(_) => ImageType::RGBA16,
        }
    }
}

#[cfg(feature = "file_io")]
impl From<ErrorKind> for IOError {
    fn from(error: ErrorKind) -> Self {
        match error {
            ErrorKind::NotFound => IOError::FileNotFound,
            ErrorKind::PermissionDenied => IOError::NoPermission,
            ErrorKind::ConnectionRefused => IOError::Unknown,
            ErrorKind::ConnectionReset => IOError::Unknown,
            ErrorKind::ConnectionAborted => IOError::Unknown,
            ErrorKind::NotConnected => IOError::Unknown,
            ErrorKind::AddrInUse => IOError::Unknown,
            ErrorKind::AddrNotAvailable => IOError::Unknown,
            ErrorKind::BrokenPipe => IOError::Unknown,
            ErrorKind::AlreadyExists => IOError::FileExists,
            ErrorKind::WouldBlock => IOError::Unknown,
            ErrorKind::InvalidInput => IOError::Unknown,
            ErrorKind::InvalidData => IOError::InvalidData,
            ErrorKind::TimedOut => IOError::Unknown,
            ErrorKind::WriteZero => IOError::WriteError,
            ErrorKind::Interrupted => IOError::Unknown,
            ErrorKind::Unsupported => IOError::Unknown,
            ErrorKind::UnexpectedEof => IOError::Unknown,
            ErrorKind::OutOfMemory => IOError::Unknown,
            ErrorKind::Other => IOError::Unknown,
            _ => IOError::Unknown,
        }
    }
}

#[cfg(feature = "file_io")]
impl From<ImageError> for IOError {
    fn from(error: ImageError) -> Self {
        match error {
            ImageError::Decoding(_) => IOError::Decoding,
            ImageError::Encoding(_) => IOError::Encoding,
            ImageError::Parameter(_) => IOError::Unknown,
            ImageError::Limits(_) => IOError::Unknown,
            ImageError::Unsupported(_) => IOError::Unknown,
            ImageError::IoError(err) => IOError::from(err.kind()),
        }
    }
}

#[cfg(feature = "file_io")]
impl From<ImageType> for ColorType {
    fn from(color: ImageType) -> Self {
        match color {
            ImageType::GRAY8 => ColorType::L8,
            ImageType::GRAYA8 => ColorType::La8,
            ImageType::GRAY16 => ColorType::L16,
            ImageType::GRAYA16 => ColorType::La16,
            ImageType::RGB8 => ColorType::Rgb8,
            ImageType::RGBA8 => ColorType::Rgba8,
            ImageType::RGB16 => ColorType::Rgb16,
            ImageType::RGBA16 => ColorType::Rgba16,
        }
    }
}





/// A trait for converting between different color/image types.
pub trait Conversions {

    /// Converts the image to the specified color type.
    fn convert(&mut self, image_type: ImageType);

    /// Checks if conversion to the specified color type is lossless.
    /// # Arguments
    /// * ```image_type``` - The color type to which the image will be converted.
    /// # Returns
    /// * ```true``` - If the conversion is lossless.
    fn lossless_cvt(&self, image_type: ImageType) -> bool;

    /// Checks if conversion to the specified color type is lossy.
    /// # Arguments
    /// * ```image_type``` - The color type to which the image will be converted.
    /// # Returns
    /// * ```true``` - If the conversion is lossy.
    fn lossy_cvt(&self, image_type: ImageType) -> bool;
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
    fn draw_circle(&mut self, center: (usize, usize), radius: usize, color: Colors, thickness: usize, opacity: f64) -> Result<(), DrawingError>;

    /// Draws an ellipse on the image. If the ellipse is not fully contained in the image, it will be clipped.
    /// # Arguments
    /// * ```center``` - The coordinates of the center of the ellipse.
    /// * ```axes``` - The lengths of the axes of the ellipse.
    /// * ```color``` - The color of the ellipse.
    /// * ```thickness``` - The thickness of the ellipse. If the thickness is 0, the ellipse will be filled.
    /// * ```opacity``` - The opacity of the ellipse.
    fn draw_ellipse(&mut self, center: (usize, usize), axes: (usize, usize), color: Colors, thickness: usize, opacity: f64) -> Result<(), DrawingError>;

    /// Draws a line on the image. If the line is not fully contained in the image, it will be clipped.
    /// # Arguments
    /// * ```point1``` - The coordinates of the first point of the line.
    /// * ```point2``` - The coordinates of the second point of the line.
    /// * ```color``` - The color of the line.
    /// * ```thickness``` - The thickness of the line.
    /// * ```opacity``` - The opacity of the line.
    fn draw_line(&mut self, point1: (usize, usize), point2: (usize, usize), color: Colors, thickness: usize, opacity: f64) -> Result<(), DrawingError>;

    /// Draws a rectangle on the image. If the rectangle is not fully contained in the image, it will be clipped.
    /// # Arguments
    /// * ```point1``` - The coordinates of the first point of the rectangle.
    /// * ```point2``` - The coordinates of the second point of the rectangle.
    /// * ```color``` - The color of the rectangle.
    /// * ```thickness``` - The thickness of the rectangle. If the thickness is 0, the rectangle will be filled.
    /// * ```opacity``` - The opacity of the rectangle.
    fn draw_rectangle(&mut self, point1: (usize, usize), point2: (usize, usize), color: Colors, thickness: usize, opacity: f64) -> Result<(), DrawingError>;
}

/// A trait for indexing into image data
pub trait Indexing {

    /// Returns the index of the first byte of the pixel at the given coordinates.
    /// # Arguments
    /// * ```index``` - The tuple with the coordinates of the pixel (x, y).
    /// # Returns
    /// * [Result] which holds the index of the first byte of the pixel or [Err] with [IndexingError].
    fn index(&self, index: (usize, usize)) -> Result<usize, IndexingError>;

    /// Returns the index of the first byte of the pixel at the given coordinates without performing checks.
    /// # Arguments
    /// * ```index``` - The tuple with the coordinates of the pixel (x, y).
    /// # Returns
    /// * The index of the first byte of the pixel.
    fn index_unchecked(&self, index: (usize, usize)) -> usize;

    /// Returns the value of the pixel at the given coordinates.
    /// # Arguments
    /// * ```index``` - The tuple with the coordinates of the pixel (x, y).
    /// # Returns
    /// * [Result] which holds the value of the pixel or [Err] with [IndexingError].
    fn get(&self, index: (usize, usize)) -> Result<Colors, IndexingError>;

    /// Returns the value of the pixel at the given coordinates without performing checks.
    /// # Arguments
    /// * ```index``` - The tuple with the coordinates of the pixel (x, y).
    /// # Returns
    /// * The value of the pixel.
    fn get_unchecked(&self, index: (usize, usize)) -> Colors;

    /// Sets the value of the pixel at the given coordinates.
    /// # Arguments
    /// * ```index``` - The tuple with the coordinates of the pixel (x, y).
    /// * ```value``` - The value to set.
    /// # Returns
    /// * [Result] which holds [Ok] or [Err] with [IndexingError].
    fn set(&mut self, index: (usize, usize), color: Colors) -> Result<(), IndexingError>;

    /// Sets the value of the pixel at the given coordinates without performing checks.
    /// # Arguments
    /// * ```index``` - The tuple with the coordinates of the pixel (x, y).
    /// * ```value``` - The value to set.
    fn set_unchecked(&mut self, index: (usize, usize), color: Colors);

    /// Sets the value of the pixel by blending it with the current value at the given coordinates.
    /// # Arguments
    /// * ```index``` - The tuple with the coordinates of the pixel (x, y).
    /// * ```value``` - The value to set.
    /// * ```opacity``` - The opacity of the new value.
    /// # Returns
    /// * [Result] which holds [Ok] or [Err] with [IndexingError].
    fn set_transparent(&mut self, index: (usize, usize), color: Colors, opacity: f64) -> Result<(), IndexingError>;

    /// Sets the value of the pixel by blending it with the current value at the given coordinates without performing checks.
    /// # Arguments
    /// * ```index``` - The tuple with the coordinates of the pixel (x, y).
    /// * ```value``` - The value to set.
    /// * ```opacity``` - The opacity of the new value.
    fn set_transparent_unchecked(&mut self, index: (usize, usize), color: Colors, opacity: f64);

    /// Fills the given range in image with the given value.
    /// # Arguments
    /// * ```index``` - The tuple with the ranges of the image to fill.
    /// * ```value``` - The value to fill with.
    /// # Returns
    /// * [Result] which holds [Ok] or [Err] with [IndexingError].
    fn fill<RX: RangeBounds<usize>, RY: RangeBounds<usize>>(&mut self, index: (RX, RY), color: Colors) -> Result<(), IndexingError>;

    /// Fills the given range in image with the given value without performing checks.
    /// # Arguments
    /// * ```index``` - The tuple with the ranges of the image to fill.
    /// * ```value``` - The value to fill with.
    fn fill_unchecked<RX: RangeBounds<usize>, RY: RangeBounds<usize>>(&mut self, index: (RX, RY), color: Colors);

    /// Fills the given range in image with the given value by blending it with the current value.
    /// # Arguments
    /// * ```index``` - The tuple with the ranges of the image to fill.
    /// * ```value``` - The value to fill with.
    /// * ```opacity``` - The opacity of the new value.
    /// # Returns
    /// * [Result] which holds [Ok] or [Err] with [IndexingError].
    fn fill_transparent<RX: RangeBounds<usize>, RY: RangeBounds<usize>>(&mut self, index: (RX, RY), color: Colors, opacity: f64) -> Result<(), IndexingError>;

    /// Fills the given range in image with the given value by blending it with the current value without performing checks.
    /// # Arguments
    /// * ```index``` - The tuple with the ranges of the image to fill.
    /// * ```value``` - The value to fill with.
    /// * ```opacity``` - The opacity of the new value.
    fn fill_transparent_unchecked<RX: RangeBounds<usize>, RY: RangeBounds<usize>>(&mut self, index: (RX, RY), color: Colors, opacity: f64);
}

/// A trait for image input/output
pub trait IO {

    /// Creates a new image from the given bytes.
    /// # Arguments
    /// * ```width``` - The width of the image.
    /// * ```height``` - The height of the image.
    /// * ```image_type``` - The type of the image.
    /// * ```bytes``` - The bytes of the image.
    /// # Returns
    /// * [Result] which holds new [Image] or [Err] with [IOError].
    fn from_bytes(width: usize, height: usize, image_type: ImageType, bytes: &[u8]) -> Result<Image, IOError>;

    /// Returns the bytes of the image as a vector.
    /// # Returns
    /// * The bytes of the image.
    fn to_bytes(&self) -> Vec<u8>;

    /// Returns the bytes of the image as a slice.
    /// # Returns
    /// * The bytes of the image.
    fn to_bytes_ref(&self) -> &[u8];

    /// Returns the bytes of the image as a mutable slice.
    /// # Returns
    /// * The bytes of the image.
    fn to_bytes_ref_mut(&mut self) -> &mut [u8];

    /// Creates a new image from the given file. Needs the ```file_io``` feature.
    /// # Arguments
    /// * ```path``` - The path to the file.
    /// # Returns
    /// * [Result] which holds new [Image] or [Err] with [IOError].
    #[cfg(feature = "file_io")]
    fn from_file(path: &str) -> Result<Image, IOError>;

    /// Writes the image to the given file. Needs the ```file_io``` feature.
    /// # Arguments
    /// * ```path``` - The path to the file.
    /// * ```overwrite``` - Whether to overwrite the file if it already exists.
    /// # Returns
    /// * [Result] which holds [Ok] or [Err] with [IOError].
    #[cfg(feature = "file_io")]
    fn to_file(&self, path: &str, overwrite: bool) -> Result<(), IOError>;
}

/// A trait with useful functions for images.
pub trait Utilities {

    /// Returns the bytes of the image as a slice.
    /// Equivalent to [IO::to_bytes_ref].
    /// If you want to edit the data, use [IO::to_bytes_ref_mut] or [Indexing] trait.
    /// # Returns
    /// * The bytes of the image.
    fn data(&self) -> &[u8];

    /// Returns the width of the image.
    /// # Returns
    /// * The width of the image.
    fn width(&self) -> usize;

    /// Returns the height of the image.
    /// # Returns
    /// * The height of the image.
    fn height(&self) -> usize;

    /// Returns the type of the image.
    /// # Returns
    /// * The type of the image.
    fn image_type(&self) -> ImageType;

    /// Deletes all drawings, restores image to match the saved background.
    fn clear(&mut self);

    /// Fills the whole image with the given value.
    fn fill_image(&mut self, color: Colors) -> Result<(), DrawingError>;

    /// Saves the current state of the image as background.
    fn save_background(&mut self);
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

impl Conversions for Image {

    fn convert(&mut self, image_type: ImageType) {
        match self.image_type {
            ImageType::GRAY8 => {
                match image_type {
                    ImageType::GRAY8 => {}, // do nothing (same type)
                    ImageType::GRAYA8 => {
                        let original_len = self.data.len();

                        self.data.reserve_exact(original_len);
                        for _ in 0..original_len {
                            self.data.push(255);
                        }

                        for i in (0..original_len).rev() {
                            self.data[i * 2] = self.data[i];
                        }
                        for i in (1..original_len).step_by(2) {
                            self.data[i] = 255;
                        }

                        self.image_type = ImageType::GRAYA8;
                    },
                    ImageType::GRAY16 => {
                        let original_len = self.data.len();

                        self.data.reserve_exact(original_len);
                        for _ in 0..original_len {
                            self.data.push(0);
                        }

                        let mul_const: f64 = u16::MAX as f64 / u8::MAX as f64;
                        for i in (0..original_len).rev() {
                            let new_val: u16 = (self.data[i] as f64 * mul_const).round() as u16;
                            let new_loc: usize = i * 2;
                            self.data[new_loc] = (new_val >> 8) as u8;
                            self.data[new_loc + 1] = new_val as u8;
                        }

                        self.image_type = ImageType::GRAY16;
                    },
                    ImageType::GRAYA16 => {
                        let original_len = self.data.len();

                        self.data.reserve_exact(original_len * 3);
                        for _ in 0..(original_len * 3) {
                            self.data.push(255);
                        }

                        let mul_const: f64 = u16::MAX as f64 / u8::MAX as f64;
                        for i in (0..original_len).rev() {
                            let new_val: u16 = (self.data[i] as f64 * mul_const).round() as u16;
                            let new_loc: usize = i * 4;
                            self.data[new_loc] = (new_val >> 8) as u8;
                            self.data[new_loc + 1] = new_val as u8;
                        }

                        for i in (2..original_len).step_by(4) {
                            self.data[i] = 255;
                            self.data[i + 1] = 255;
                        }

                        self.image_type = ImageType::GRAYA16;
                    },
                    ImageType::RGB8 => {
                        let original_len = self.data.len();

                        self.data.reserve_exact(original_len * 2);
                        for _ in 0..(original_len * 2) {
                            self.data.push(0);
                        }

                        for i in (0..original_len).rev() {
                            let new_loc: usize = i * 3;
                            self.data[new_loc] = self.data[i];
                            self.data[new_loc + 1] = self.data[i];
                            self.data[new_loc + 2] = self.data[i];
                        }

                        self.image_type = ImageType::RGB8;
                    },
                    ImageType::RGBA8 => {
                        let original_len = self.data.len();

                        self.data.reserve_exact(original_len * 3);
                        for _ in 0..(original_len * 3) {
                            self.data.push(255);
                        }

                        for i in (0..original_len).rev() {
                            let new_loc: usize = i * 4;
                            self.data[new_loc] = self.data[i];
                            self.data[new_loc + 1] = self.data[i];
                            self.data[new_loc + 2] = self.data[i];
                        }

                        for i in (3..original_len).step_by(4) {
                            self.data[i] = 255;
                        }

                        self.image_type = ImageType::RGBA8;
                    },
                    ImageType::RGB16 => {
                        let original_len = self.data.len();

                        self.data.reserve_exact(original_len * 5);
                        for _ in 0..(original_len * 5) {
                            self.data.push(0);
                        }

                        let mul_const: f64 = u16::MAX as f64 / u8::MAX as f64;
                        for i in (0..original_len).rev() {
                            let new_val: u16 = (self.data[i] as f64 * mul_const).round() as u16;
                            let new_loc: usize = i * 6;
                            self.data[new_loc] = (new_val >> 8) as u8;
                            self.data[new_loc + 1] = new_val as u8;
                            self.data[new_loc + 2] = (new_val >> 8) as u8;
                            self.data[new_loc + 3] = new_val as u8;
                            self.data[new_loc + 4] = (new_val >> 8) as u8;
                            self.data[new_loc + 5] = new_val as u8;
                        }

                        self.image_type = ImageType::RGB16;
                    },
                    ImageType::RGBA16 => {
                        let original_len = self.data.len();

                        self.data.reserve_exact(original_len * 7);
                        for _ in 0..(original_len * 7) {
                            self.data.push(255);
                        }

                        let mul_const: f64 = u16::MAX as f64 / u8::MAX as f64;
                        for i in (0..original_len).rev() {
                            let new_val: u16 = (self.data[i] as f64 * mul_const).round() as u16;
                            let new_loc: usize = i * 8;
                            self.data[new_loc] = (new_val >> 8) as u8;
                            self.data[new_loc + 1] = new_val as u8;
                            self.data[new_loc + 2] = (new_val >> 8) as u8;
                            self.data[new_loc + 3] = new_val as u8;
                            self.data[new_loc + 4] = (new_val >> 8) as u8;
                            self.data[new_loc + 5] = new_val as u8;
                        }

                        for i in (6..original_len).step_by(8) {
                            self.data[i] = 255;
                            self.data[i + 1] = 255;
                        }

                        self.image_type = ImageType::RGBA16;
                    },
                }
            },
            ImageType::GRAYA8 => {
                match image_type {
                    ImageType::GRAY8 => {},
                    ImageType::GRAYA8 => {},
                    ImageType::GRAY16 => {},
                    ImageType::GRAYA16 => {},
                    ImageType::RGB8 => {},
                    ImageType::RGBA8 => {},
                    ImageType::RGB16 => {},
                    ImageType::RGBA16 => {},
                }
            },
            ImageType::GRAY16 => {
                match image_type {
                    ImageType::GRAY8 => {},
                    ImageType::GRAYA8 => {},
                    ImageType::GRAY16 => {},
                    ImageType::GRAYA16 => {},
                    ImageType::RGB8 => {},
                    ImageType::RGBA8 => {},
                    ImageType::RGB16 => {},
                    ImageType::RGBA16 => {},
                }
            },
            ImageType::GRAYA16 => {
                match image_type {
                    ImageType::GRAY8 => {},
                    ImageType::GRAYA8 => {},
                    ImageType::GRAY16 => {},
                    ImageType::GRAYA16 => {},
                    ImageType::RGB8 => {},
                    ImageType::RGBA8 => {},
                    ImageType::RGB16 => {},
                    ImageType::RGBA16 => {},
                }
            },
            ImageType::RGB8 => {
                match image_type {
                    ImageType::GRAY8 => {},
                    ImageType::GRAYA8 => {},
                    ImageType::GRAY16 => {},
                    ImageType::GRAYA16 => {},
                    ImageType::RGB8 => {},
                    ImageType::RGBA8 => {},
                    ImageType::RGB16 => {},
                    ImageType::RGBA16 => {},
                }
            },
            ImageType::RGBA8 => {
                match image_type {
                    ImageType::GRAY8 => {},
                    ImageType::GRAYA8 => {},
                    ImageType::GRAY16 => {},
                    ImageType::GRAYA16 => {},
                    ImageType::RGB8 => {},
                    ImageType::RGBA8 => {},
                    ImageType::RGB16 => {},
                    ImageType::RGBA16 => {},
                }
            },
            ImageType::RGB16 => {
                match image_type {
                    ImageType::GRAY8 => {},
                    ImageType::GRAYA8 => {},
                    ImageType::GRAY16 => {},
                    ImageType::GRAYA16 => {},
                    ImageType::RGB8 => {},
                    ImageType::RGBA8 => {},
                    ImageType::RGB16 => {},
                    ImageType::RGBA16 => {},
                }
            },
            ImageType::RGBA16 => {
                match image_type {
                    ImageType::GRAY8 => {},
                    ImageType::GRAYA8 => {},
                    ImageType::GRAY16 => {},
                    ImageType::GRAYA16 => {},
                    ImageType::RGB8 => {},
                    ImageType::RGBA8 => {},
                    ImageType::RGB16 => {},
                    ImageType::RGBA16 => {},
                }
            },
        }
    }

    fn lossless_cvt(&self, image_type: ImageType) -> bool {
        match self.image_type {
            ImageType::GRAY8 => true,
            ImageType::GRAYA8 => {
                match image_type {
                    ImageType::GRAY8 => false,
                    ImageType::GRAYA8 => true,
                    ImageType::GRAY16 => false,
                    ImageType::GRAYA16 => true,
                    ImageType::RGB8 => false,
                    ImageType::RGBA8 => true,
                    ImageType::RGB16 => false,
                    ImageType::RGBA16 => true,
                }
            },
            ImageType::GRAY16 => {
                match image_type {
                    ImageType::GRAY8 => false,
                    ImageType::GRAYA8 => false,
                    ImageType::GRAY16 => true,
                    ImageType::GRAYA16 => true,
                    ImageType::RGB8 => false,
                    ImageType::RGBA8 => false,
                    ImageType::RGB16 => true,
                    ImageType::RGBA16 => true,
                }
            },
            ImageType::GRAYA16 => {
                match image_type {
                    ImageType::GRAY8 => false,
                    ImageType::GRAYA8 => false,
                    ImageType::GRAY16 => false,
                    ImageType::GRAYA16 => true,
                    ImageType::RGB8 => false,
                    ImageType::RGBA8 => false,
                    ImageType::RGB16 => false,
                    ImageType::RGBA16 => true,
                }
            },
            ImageType::RGB8 => {
                match image_type {
                    ImageType::GRAY8 => false,
                    ImageType::GRAYA8 => false,
                    ImageType::GRAY16 => false,
                    ImageType::GRAYA16 => false,
                    ImageType::RGB8 => true,
                    ImageType::RGBA8 => true,
                    ImageType::RGB16 => true,
                    ImageType::RGBA16 => true,
                }
            },
            ImageType::RGBA8 => {
                match image_type {
                    ImageType::GRAY8 => false,
                    ImageType::GRAYA8 => false,
                    ImageType::GRAY16 => false,
                    ImageType::GRAYA16 => false,
                    ImageType::RGB8 => false,
                    ImageType::RGBA8 => true,
                    ImageType::RGB16 => false,
                    ImageType::RGBA16 => true,
                }
            },
            ImageType::RGB16 => {
                match image_type {
                    ImageType::GRAY8 => false,
                    ImageType::GRAYA8 => false,
                    ImageType::GRAY16 => false,
                    ImageType::GRAYA16 => false,
                    ImageType::RGB8 => false,
                    ImageType::RGBA8 => false,
                    ImageType::RGB16 => true,
                    ImageType::RGBA16 => true,
                }
            },
            ImageType::RGBA16 => {
                matches!(image_type, ImageType::RGBA16)
            },
        }
    }

    fn lossy_cvt(&self, image_type: ImageType) -> bool {
        !self.lossless_cvt(image_type)
    }
}

impl Drawing for Image {

    fn draw_circle(&mut self, center: (usize, usize), radius: usize, color: Colors, thickness: usize, opacity: f64) -> Result<(), DrawingError> {
        todo!()
    }

    fn draw_ellipse(&mut self, center: (usize, usize), axes: (usize, usize), color: Colors, thickness: usize, opacity: f64) -> Result<(), DrawingError> {
        todo!()
    }

    fn draw_line(&mut self, point1: (usize, usize), point2: (usize, usize), color: Colors, thickness: usize, opacity: f64) -> Result<(), DrawingError> {
        todo!()
    }

    fn draw_rectangle(&mut self, point1: (usize, usize), point2: (usize, usize), color: Colors, thickness: usize, opacity: f64) -> Result<(), DrawingError> {

        // check if color is valid for this image type
        if ImageType::from(color) != self.image_type {
            return Err(DrawingError::WrongColor);
        }

        // if opacity is less than 0.0, bigger than 1.0, or NaN, return error
        if opacity.is_nan() || !(0.0..=1.0).contains(&opacity) {
            return Err(DrawingError::InvalidOpacity);
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
            return Ok(());  // rectangle is out of image, nothing is to be drawn.
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
                self.fill_unchecked((smaller_x..(bigger_x + 1), smaller_y..(bigger_y + 1)), color);
            } else {
                // Draw filled, transparent rectangle.
                self.fill_transparent_unchecked((smaller_x..(bigger_x + 1), smaller_y..(bigger_y + 1)), color, opacity);
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
                            self.fill_unchecked((smaller_x..(smaller_x + 1), smaller_y..min(self.height, bigger_y + 1)), color);
                        }
                    } else if bigger_y == smaller_y {
                        self.fill_unchecked((smaller_x..min(self.width, bigger_x + 1), smaller_y..(smaller_y + 1)), color);
                    } else {
                        self.fill_unchecked((smaller_x..min(self.width, bigger_x + 1), smaller_y..(smaller_y + 1)), color);  // bottom
                        if bigger_y < self.height {
                            self.fill_unchecked((smaller_x..min(self.width, bigger_x + 1), bigger_y..(bigger_y + 1)), color);  // top
                        }
                        self.fill_unchecked((smaller_x..(smaller_x + 1), (smaller_y + 1)..min(bigger_y, self.height)), color);  // left
                        if bigger_x < self.width {
                            self.fill_unchecked((bigger_x..(bigger_x + 1), (smaller_y + 1)..min(bigger_y, self.height)), color);  // right
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
                            self.fill_transparent_unchecked((smaller_x..(smaller_x + 1), smaller_y..min(self.height, bigger_y + 1)), color, opacity);
                        }
                    } else if bigger_y == smaller_y {
                        self.fill_transparent_unchecked((smaller_x..min(self.width, bigger_x + 1), smaller_y..(smaller_y + 1)), color, opacity);
                    } else {
                        self.fill_transparent_unchecked((smaller_x..min(self.width, bigger_x + 1), smaller_y..(smaller_y + 1)), color, opacity);  // bottom
                        if bigger_y < self.height {
                            self.fill_transparent_unchecked((smaller_x..min(self.width, bigger_x + 1), bigger_y..(bigger_y + 1)), color, opacity);  // top
                        }
                        self.fill_transparent_unchecked((smaller_x..(smaller_x + 1), (smaller_y + 1)..min(bigger_y, self.height)), color, opacity);  // left
                        if bigger_x < self.width {
                            self.fill_transparent_unchecked((bigger_x..(bigger_x + 1), (smaller_y + 1)..min(bigger_y, self.height)), color, opacity);  // right
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

impl Indexing for Image {

    #[inline]
    fn index(&self, index: (usize, usize)) -> Result<usize, IndexingError> {
        if index.0 >= self.width || index.1 >= self.height {
            Err(IndexingError::OutOfBounds)
        } else {
            Ok(self.index_unchecked(index))
        }
    }

    #[inline]
    fn index_unchecked(&self, index: (usize, usize)) -> usize {
        let pix_ind: usize = (self.height - index.1 - 1) * self.width + index.0;
        match self.image_type {
            ImageType::GRAY8 => pix_ind,
            ImageType::GRAYA8 => pix_ind * 2,
            ImageType::GRAY16 => pix_ind * 2,
            ImageType::GRAYA16 => pix_ind * 4,
            ImageType::RGB8 => pix_ind * 3,
            ImageType::RGBA8 => pix_ind * 4,
            ImageType::RGB16 => pix_ind * 6,
            ImageType::RGBA16 => pix_ind * 8,
        }
    }

    #[inline]
    fn get(&self, index: (usize, usize)) -> Result<Colors, IndexingError> {
        if index.0 >= self.width || index.1 >= self.height {
            Err(IndexingError::OutOfBounds)
        } else {
            Ok(self.get_unchecked(index))
        }
    }

    #[inline]
    fn get_unchecked(&self, index: (usize, usize)) -> Colors {
        let index_temp: usize = self.index_unchecked(index);
        match self.image_type {
            ImageType::GRAY8 => Colors::GRAY8(
                self.data[index_temp]
            ),
            ImageType::GRAYA8 => Colors::GRAYA8([
                self.data[index_temp],
                self.data[index_temp + 1]
            ]),
            ImageType::GRAY16 => Colors::GRAY16(
                ((self.data[index_temp] as u16) << 8) | (self.data[index_temp + 1] as u16)
            ),
            ImageType::GRAYA16 => Colors::GRAYA16([
                ((self.data[index_temp] as u16) << 8) | (self.data[index_temp + 1] as u16),
                ((self.data[index_temp + 2] as u16) << 8) | (self.data[index_temp + 3] as u16)
            ]),
            ImageType::RGB8 => Colors::RGB8([
                self.data[index_temp],
                self.data[index_temp + 1],
                self.data[index_temp + 2]
            ]),
            ImageType::RGBA8 => Colors::RGBA8([
                self.data[index_temp],
                self.data[index_temp + 1],
                self.data[index_temp + 2],
                self.data[index_temp + 3]
            ]),
            ImageType::RGB16 => Colors::RGB16([
                ((self.data[index_temp] as u16) << 8) | (self.data[index_temp + 1] as u16),
                ((self.data[index_temp + 2] as u16) << 8) | (self.data[index_temp + 3] as u16),
                ((self.data[index_temp + 4] as u16) << 8) | (self.data[index_temp + 5] as u16)
            ]),
            ImageType::RGBA16 => Colors::RGBA16([
                ((self.data[index_temp] as u16) << 8) | (self.data[index_temp + 1] as u16),
                ((self.data[index_temp + 2] as u16) << 8) | (self.data[index_temp + 3] as u16),
                ((self.data[index_temp + 4] as u16) << 8) | (self.data[index_temp + 5] as u16),
                ((self.data[index_temp + 6] as u16) << 8) | (self.data[index_temp + 7] as u16)
            ]),
        }
    }

    #[inline]
    fn set(&mut self, index: (usize, usize), color: Colors) -> Result<(), IndexingError> {
        if index.0 >= self.width || index.1 >= self.height {
            Err(IndexingError::OutOfBounds)
        } else if ImageType::from(color) != self.image_type {
            Err(IndexingError::WrongColor)
        } else {
            self.set_unchecked(index, color);
            Ok(())
        }
    }

    #[inline]
    fn set_unchecked(&mut self, index: (usize, usize), color: Colors) {
        let index_temp: usize = self.index_unchecked(index);

        match color {
            Colors::GRAY8(val) => {
                self.data[index_temp] = val;
            },
            Colors::GRAYA8(val) => {
                self.data[index_temp] = val[0];
                self.data[index_temp + 1] = val[1];
            },
            Colors::GRAY16(val) => {
                self.data[index_temp] = (val >> 8) as u8;
                self.data[index_temp + 1] = val as u8;
            },
            Colors::GRAYA16(val) => {
                self.data[index_temp] = (val[0] >> 8) as u8;
                self.data[index_temp + 1] = val[0] as u8;
                self.data[index_temp + 2] = (val[1] >> 8) as u8;
                self.data[index_temp + 3] = val[1] as u8;
            },
            Colors::RGB8(val) => {
                self.data[index_temp] = val[0];
                self.data[index_temp + 1] = val[1];
                self.data[index_temp + 2] = val[2];
            },
            Colors::RGBA8(val) => {
                self.data[index_temp] = val[0];
                self.data[index_temp + 1] = val[1];
                self.data[index_temp + 2] = val[2];
                self.data[index_temp + 3] = val[3];
            },
            Colors::RGB16(val) => {
                self.data[index_temp] = (val[0] >> 8) as u8;
                self.data[index_temp + 1] = val[0] as u8;
                self.data[index_temp + 2] = (val[1] >> 8) as u8;
                self.data[index_temp + 3] = val[1] as u8;
                self.data[index_temp + 4] = (val[2] >> 8) as u8;
                self.data[index_temp + 5] = val[2] as u8;
            },
            Colors::RGBA16(val) => {
                self.data[index_temp] = (val[0] >> 8) as u8;
                self.data[index_temp + 1] = val[0] as u8;
                self.data[index_temp + 2] = (val[1] >> 8) as u8;
                self.data[index_temp + 3] = val[1] as u8;
                self.data[index_temp + 4] = (val[2] >> 8) as u8;
                self.data[index_temp + 5] = val[2] as u8;
                self.data[index_temp + 6] = (val[3] >> 8) as u8;
                self.data[index_temp + 7] = val[3] as u8;
            }
        }
    }

    #[inline]
    fn set_transparent(&mut self, index: (usize, usize), color: Colors, opacity: f64) -> Result<(), IndexingError> {
        if index.0 >= self.width || index.1 >= self.height {
            Err(IndexingError::OutOfBounds)
        } else if ImageType::from(color) != self.image_type {
            Err(IndexingError::WrongColor)
        } else if opacity.is_nan() {
            Err(IndexingError::InvalidOpacity)
        } else {
            let opacity_temp: f64 = if opacity > 1.0 {
                1.0
            } else if opacity <= 0.0 {
                return Ok(()); // Do nothing
            } else {
                opacity
            };
            self.set_transparent_unchecked(index, color, opacity_temp);
            Ok(())
        }
    }

    #[inline]
    fn set_transparent_unchecked(&mut self, index: (usize, usize), color: Colors, opacity: f64) {
        if opacity == 1.0 {
            self.set_unchecked(index, color);
            return;
        } else if opacity == 0.0 {
            return; // Do nothing
        }

        let index_temp: usize = self.index_unchecked(index);

        // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage

        match color {
            Colors::GRAY8(val) => {
                self.data[index_temp] = (self.data[index_temp] as f64 * (1.0 - opacity) + val as f64 * opacity).round() as u8;
            },
            Colors::GRAYA8(val) => {
                self.data[index_temp] = (self.data[index_temp] as f64 * (1.0 - opacity) + val[0] as f64 * opacity).round() as u8;
                self.data[index_temp + 1] = (self.data[index_temp + 1] as f64 * (1.0 - opacity) + val[1] as f64 * opacity).round() as u8;
            },
            Colors::GRAY16(val) => {
                let new_val: u16 = ((((self.data[index_temp] as u16) << 8) | (self.data[index_temp + 1] as u16)) as f64 * (1.0 - opacity) + val as f64 * opacity).round() as u16;
                self.data[index_temp] = (new_val >> 8) as u8;
                self.data[index_temp + 1] = new_val as u8;
            },
            Colors::GRAYA16(val) => {
                let mut new_val: u16 = ((((self.data[index_temp] as u16) << 8) | (self.data[index_temp + 1] as u16)) as f64 * (1.0 - opacity) + val[0] as f64 * opacity).round() as u16;
                self.data[index_temp] = (new_val >> 8) as u8;
                self.data[index_temp + 1] = new_val as u8;
                new_val = ((((self.data[index_temp + 2] as u16) << 8) | (self.data[index_temp + 3] as u16)) as f64 * (1.0 - opacity) + val[1] as f64 * opacity).round() as u16;
                self.data[index_temp + 2] = (new_val >> 8) as u8;
                self.data[index_temp + 3] = new_val as u8;
            },
            Colors::RGB8(val) => {
                self.data[index_temp] = (self.data[index_temp] as f64 * (1.0 - opacity) + val[0] as f64 * opacity).round() as u8;
                self.data[index_temp + 1] = (self.data[index_temp + 1] as f64 * (1.0 - opacity) + val[1] as f64 * opacity).round() as u8;
                self.data[index_temp + 2] = (self.data[index_temp + 2] as f64 * (1.0 - opacity) + val[2] as f64 * opacity).round() as u8;
            },
            Colors::RGBA8(val) => {
                self.data[index_temp] = (self.data[index_temp] as f64 * (1.0 - opacity) + val[0] as f64 * opacity).round() as u8;
                self.data[index_temp + 1] = (self.data[index_temp + 1] as f64 * (1.0 - opacity) + val[1] as f64 * opacity).round() as u8;
                self.data[index_temp + 2] = (self.data[index_temp + 2] as f64 * (1.0 - opacity) + val[2] as f64 * opacity).round() as u8;
                self.data[index_temp + 3] = (self.data[index_temp + 3] as f64 * (1.0 - opacity) + val[3] as f64 * opacity).round() as u8;
            },
            Colors::RGB16(val) => {
                let mut new_val: u16 = ((((self.data[index_temp] as u16) << 8) | (self.data[index_temp + 1] as u16)) as f64 * (1.0 - opacity) + val[0] as f64 * opacity).round() as u16;
                self.data[index_temp] = (new_val >> 8) as u8;
                self.data[index_temp + 1] = new_val as u8;
                new_val = ((((self.data[index_temp + 2] as u16) << 8) | (self.data[index_temp + 3] as u16)) as f64 * (1.0 - opacity) + val[1] as f64 * opacity).round() as u16;
                self.data[index_temp + 2] = (new_val >> 8) as u8;
                self.data[index_temp + 3] = new_val as u8;
                new_val = ((((self.data[index_temp + 4] as u16) << 8) | (self.data[index_temp + 5] as u16)) as f64 * (1.0 - opacity) + val[2] as f64 * opacity).round() as u16;
                self.data[index_temp + 4] = (new_val >> 8) as u8;
                self.data[index_temp + 5] = new_val as u8;
            },
            Colors::RGBA16(val) => {
                let mut new_val: u16 = ((((self.data[index_temp] as u16) << 8) | (self.data[index_temp + 1] as u16)) as f64 * (1.0 - opacity) + val[0] as f64 * opacity).round() as u16;
                self.data[index_temp] = (new_val >> 8) as u8;
                self.data[index_temp + 1] = new_val as u8;
                new_val = ((((self.data[index_temp + 2] as u16) << 8) | (self.data[index_temp + 3] as u16)) as f64 * (1.0 - opacity) + val[1] as f64 * opacity).round() as u16;
                self.data[index_temp + 2] = (new_val >> 8) as u8;
                self.data[index_temp + 3] = new_val as u8;
                new_val = ((((self.data[index_temp + 4] as u16) << 8) | (self.data[index_temp + 5] as u16)) as f64 * (1.0 - opacity) + val[2] as f64 * opacity).round() as u16;
                self.data[index_temp + 4] = (new_val >> 8) as u8;
                self.data[index_temp + 5] = new_val as u8;
                new_val = ((((self.data[index_temp + 6] as u16) << 8) | (self.data[index_temp + 7] as u16)) as f64 * (1.0 - opacity) + val[3] as f64 * opacity).round() as u16;
                self.data[index_temp + 6] = (new_val >> 8) as u8;
                self.data[index_temp + 7] = new_val as u8;
            }
        }
    }

    #[inline]
    fn fill<RX: RangeBounds<usize>, RY: RangeBounds<usize>>(&mut self, index: (RX, RY), color: Colors) -> Result<(), IndexingError> {
        let index_x_lower: usize = match index.0.start_bound() {
            Bound::Included(&x) => {
                if x >= self.width {
                    return Err(IndexingError::OutOfBounds);
                }
                x
            }
            Bound::Excluded(&x) => {
                if x + 1 >= self.width {
                    return Err(IndexingError::OutOfBounds);
                }
                x + 1
            }
            Bound::Unbounded => {
                0
            }
        };

        let index_x_upper: usize = match index.0.end_bound() {
            Bound::Included(&x) => {
                if x >= self.width {
                    return Err(IndexingError::OutOfBounds);
                }
                x + 1
            }
            Bound::Excluded(&x) => {
                if x > self.width {
                    return Err(IndexingError::OutOfBounds);
                }
                x
            }
            Bound::Unbounded => {
                self.width
            }
        };

        let index_y_lower: usize = match index.1.start_bound() {
            Bound::Included(&y) => {
                if y >= self.height {
                    return Err(IndexingError::OutOfBounds);
                }
                y
            }
            Bound::Excluded(&y) => {
                if y + 1 >= self.height {
                    return Err(IndexingError::OutOfBounds);
                }
                y + 1
            }
            Bound::Unbounded => {
                0
            }
        };

        let index_y_upper: usize = match index.1.end_bound() {
            Bound::Included(&y) => {
                if y >= self.height {
                    return Err(IndexingError::OutOfBounds);
                }
                y + 1
            }
            Bound::Excluded(&y) => {
                if y > self.height {
                    return Err(IndexingError::OutOfBounds);
                }
                y
            }
            Bound::Unbounded => {
                self.height
            }
        };

        if ImageType::from(color) != self.image_type {
            return Err(IndexingError::WrongColor);
        }

        match color {
            Colors::GRAY8(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index = self.index_unchecked((index_x_lower, y));
                    self.data[temp_index..temp_index + index_x_upper - index_x_lower].fill(val);
                }
            },
            Colors::GRAYA8(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 2;
                    for x in (temp_index_low..temp_index_high).step_by(2) {
                        self.data[x] = val[0];
                        self.data[x + 1] = val[1];
                    }
                }
            },
            Colors::GRAY16(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 2;
                    for x in (temp_index_low..temp_index_high).step_by(2) {
                        self.data[x] = (val >> 8) as u8;
                        self.data[x + 1] = val as u8;
                    }
                }
            },
            Colors::GRAYA16(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 4;
                    for x in (temp_index_low..temp_index_high).step_by(4) {
                        self.data[x] = (val[0] >> 8) as u8;
                        self.data[x + 1] = val[0] as u8;
                        self.data[x + 2] = (val[1] >> 8) as u8;
                        self.data[x + 3] = val[1] as u8;
                    }
                }
            },
            Colors::RGB8(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 3;
                    for x in (temp_index_low..temp_index_high).step_by(3) {
                        self.data[x] = val[0];
                        self.data[x + 1] = val[1];
                        self.data[x + 2] = val[2];
                    }
                }
            },
            Colors::RGBA8(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 4;
                    for x in (temp_index_low..temp_index_high).step_by(4) {
                        self.data[x] = val[0];
                        self.data[x + 1] = val[1];
                        self.data[x + 2] = val[2];
                        self.data[x + 3] = val[3];
                    }
                }
            },
            Colors::RGB16(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 6;
                    for x in (temp_index_low..temp_index_high).step_by(6) {
                        self.data[x] = (val[0] >> 8) as u8;
                        self.data[x + 1] = val[0] as u8;
                        self.data[x + 2] = (val[1] >> 8) as u8;
                        self.data[x + 3] = val[1] as u8;
                        self.data[x + 4] = (val[2] >> 8) as u8;
                        self.data[x + 5] = val[2] as u8;
                    }
                }
            },
            Colors::RGBA16(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 8;
                    for x in (temp_index_low..temp_index_high).step_by(8) {
                        self.data[x] = (val[0] >> 8) as u8;
                        self.data[x + 1] = val[0] as u8;
                        self.data[x + 2] = (val[1] >> 8) as u8;
                        self.data[x + 3] = val[1] as u8;
                        self.data[x + 4] = (val[2] >> 8) as u8;
                        self.data[x + 5] = val[2] as u8;
                        self.data[x + 6] = (val[3] >> 8) as u8;
                        self.data[x + 7] = val[3] as u8;
                    }
                }
            }
        }
        Ok(())
    }

    #[inline]
    fn fill_unchecked<RX: RangeBounds<usize>, RY: RangeBounds<usize>>(&mut self, index: (RX, RY), color: Colors) {
        let index_x_lower: usize = match index.0.start_bound() {
            Bound::Included(&x) => {
                x
            }
            Bound::Excluded(&x) => {
                x + 1
            }
            Bound::Unbounded => {
                0
            }
        };

        let index_x_upper: usize = match index.0.end_bound() {
            Bound::Included(&x) => {
                x + 1
            }
            Bound::Excluded(&x) => {
                x
            }
            Bound::Unbounded => {
                self.width
            }
        };

        let index_y_lower: usize = match index.1.start_bound() {
            Bound::Included(&y) => {
                y
            }
            Bound::Excluded(&y) => {
                y + 1
            }
            Bound::Unbounded => {
                0
            }
        };

        let index_y_upper: usize = match index.1.end_bound() {
            Bound::Included(&y) => {
                y + 1
            }
            Bound::Excluded(&y) => {
                y
            }
            Bound::Unbounded => {
                self.height
            }
        };

        match color {
            Colors::GRAY8(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index = self.index_unchecked((index_x_lower, y));
                    self.data[temp_index..temp_index + index_x_upper - index_x_lower].fill(val);
                }
            },
            Colors::GRAYA8(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 2;
                    for x in (temp_index_low..temp_index_high).step_by(2) {
                        self.data[x] = val[0];
                        self.data[x + 1] = val[1];
                    }
                }
            },
            Colors::GRAY16(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 2;
                    for x in (temp_index_low..temp_index_high).step_by(2) {
                        self.data[x] = (val >> 8) as u8;
                        self.data[x + 1] = val as u8;
                    }
                }
            },
            Colors::GRAYA16(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 4;
                    for x in (temp_index_low..temp_index_high).step_by(4) {
                        self.data[x] = (val[0] >> 8) as u8;
                        self.data[x + 1] = val[0] as u8;
                        self.data[x + 2] = (val[1] >> 8) as u8;
                        self.data[x + 3] = val[1] as u8;
                    }
                }
            },
            Colors::RGB8(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 3;
                    for x in (temp_index_low..temp_index_high).step_by(3) {
                        self.data[x] = val[0];
                        self.data[x + 1] = val[1];
                        self.data[x + 2] = val[2];
                    }
                }
            },
            Colors::RGBA8(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 4;
                    for x in (temp_index_low..temp_index_high).step_by(4) {
                        self.data[x] = val[0];
                        self.data[x + 1] = val[1];
                        self.data[x + 2] = val[2];
                        self.data[x + 3] = val[3];
                    }
                }
            },
            Colors::RGB16(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 6;
                    for x in (temp_index_low..temp_index_high).step_by(6) {
                        self.data[x] = (val[0] >> 8) as u8;
                        self.data[x + 1] = val[0] as u8;
                        self.data[x + 2] = (val[1] >> 8) as u8;
                        self.data[x + 3] = val[1] as u8;
                        self.data[x + 4] = (val[2] >> 8) as u8;
                        self.data[x + 5] = val[2] as u8;
                    }
                }
            },
            Colors::RGBA16(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 8;
                    for x in (temp_index_low..temp_index_high).step_by(8) {
                        self.data[x] = (val[0] >> 8) as u8;
                        self.data[x + 1] = val[0] as u8;
                        self.data[x + 2] = (val[1] >> 8) as u8;
                        self.data[x + 3] = val[1] as u8;
                        self.data[x + 4] = (val[2] >> 8) as u8;
                        self.data[x + 5] = val[2] as u8;
                        self.data[x + 6] = (val[3] >> 8) as u8;
                        self.data[x + 7] = val[3] as u8;
                    }
                }
            }
        }
    }

    #[inline]
    fn fill_transparent<RX: RangeBounds<usize>, RY: RangeBounds<usize>>(&mut self, index: (RX, RY), color: Colors, opacity: f64) -> Result<(), IndexingError> {
        let index_x_lower: usize = match index.0.start_bound() {
            Bound::Included(&x) => {
                if x >= self.width {
                    return Err(IndexingError::OutOfBounds);
                }
                x
            }
            Bound::Excluded(&x) => {
                if x + 1 >= self.width {
                    return Err(IndexingError::OutOfBounds);
                }
                x + 1
            }
            Bound::Unbounded => {
                0
            }
        };

        let index_x_upper: usize = match index.0.end_bound() {
            Bound::Included(&x) => {
                if x >= self.width {
                    return Err(IndexingError::OutOfBounds);
                }
                x + 1
            }
            Bound::Excluded(&x) => {
                if x > self.width {
                    return Err(IndexingError::OutOfBounds);
                }
                x
            }
            Bound::Unbounded => {
                self.width
            }
        };

        let index_y_lower: usize = match index.1.start_bound() {
            Bound::Included(&y) => {
                if y >= self.height {
                    return Err(IndexingError::OutOfBounds);
                }
                y
            }
            Bound::Excluded(&y) => {
                if y + 1 >= self.height {
                    return Err(IndexingError::OutOfBounds);
                }
                y + 1
            }
            Bound::Unbounded => {
                0
            }
        };

        let index_y_upper: usize = match index.1.end_bound() {
            Bound::Included(&y) => {
                if y >= self.height {
                    return Err(IndexingError::OutOfBounds);
                }
                y + 1
            }
            Bound::Excluded(&y) => {
                if y > self.height {
                    return Err(IndexingError::OutOfBounds);
                }
                y
            }
            Bound::Unbounded => {
                self.height
            }
        };

        if ImageType::from(color) != self.image_type {
            return Err(IndexingError::WrongColor);
        }

        // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage

        match color {
            Colors::GRAY8(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower);
                    for x in temp_index_low..temp_index_high {
                        self.data[x] = (self.data[x] as f64 * (1.0 - opacity) + val as f64 * opacity).round() as u8;
                    }
                }
            },
            Colors::GRAYA8(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 2;
                    for x in (temp_index_low..temp_index_high).step_by(2) {
                        self.data[x] = (self.data[x] as f64 * (1.0 - opacity) + val[0] as f64 * opacity).round() as u8;
                        self.data[x + 1] = (self.data[x + 1] as f64 * (1.0 - opacity) + val[1] as f64 * opacity).round() as u8;
                    }
                }
            },
            Colors::GRAY16(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 2;
                    for x in (temp_index_low..temp_index_high).step_by(2) {
                        let new_val: u16 = ((((self.data[x] as u16) << 8) | (self.data[x + 1] as u16)) as f64 * (1.0 - opacity) + val as f64 * opacity).round() as u16;
                        self.data[x] = (new_val >> 8) as u8;
                        self.data[x + 1] = new_val as u8;
                    }
                }
            },
            Colors::GRAYA16(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 4;
                    for x in (temp_index_low..temp_index_high).step_by(4) {
                        let mut new_val: u16 = ((((self.data[x] as u16) << 8) | (self.data[x + 1] as u16)) as f64 * (1.0 - opacity) + val[0] as f64 * opacity).round() as u16;
                        self.data[x] = (new_val >> 8) as u8;
                        self.data[x + 1] = new_val as u8;
                        new_val = ((((self.data[x + 2] as u16) << 8) | (self.data[x + 3] as u16)) as f64 * (1.0 - opacity) + val[1] as f64 * opacity).round() as u16;
                        self.data[x + 2] = (new_val >> 8) as u8;
                        self.data[x + 3] = new_val as u8;
                    }
                }
            },
            Colors::RGB8(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 3;
                    for x in (temp_index_low..temp_index_high).step_by(3) {
                        self.data[x] = (self.data[x] as f64 * (1.0 - opacity) + val[0] as f64 * opacity).round() as u8;
                        self.data[x + 1] = (self.data[x + 1] as f64 * (1.0 - opacity) + val[1] as f64 * opacity).round() as u8;
                        self.data[x + 2] = (self.data[x + 2] as f64 * (1.0 - opacity) + val[2] as f64 * opacity).round() as u8;
                    }
                }
            },
            Colors::RGBA8(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 4;
                    for x in (temp_index_low..temp_index_high).step_by(4) {
                        self.data[x] = (self.data[x] as f64 * (1.0 - opacity) + val[0] as f64 * opacity).round() as u8;
                        self.data[x + 1] = (self.data[x + 1] as f64 * (1.0 - opacity) + val[1] as f64 * opacity).round() as u8;
                        self.data[x + 2] = (self.data[x + 2] as f64 * (1.0 - opacity) + val[2] as f64 * opacity).round() as u8;
                        self.data[x + 3] = (self.data[x + 3] as f64 * (1.0 - opacity) + val[3] as f64 * opacity).round() as u8;
                    }
                }
            },
            Colors::RGB16(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 6;
                    for x in (temp_index_low..temp_index_high).step_by(6) {
                        let mut new_val: u16 = ((((self.data[x] as u16) << 8) | (self.data[x + 1] as u16)) as f64 * (1.0 - opacity) + val[0] as f64 * opacity).round() as u16;
                        self.data[x] = (new_val >> 8) as u8;
                        self.data[x + 1] = new_val as u8;
                        new_val = ((((self.data[x + 2] as u16) << 8) | (self.data[x + 3] as u16)) as f64 * (1.0 - opacity) + val[1] as f64 * opacity).round() as u16;
                        self.data[x + 2] = (new_val >> 8) as u8;
                        self.data[x + 3] = new_val as u8;
                        new_val = ((((self.data[x + 4] as u16) << 8) | (self.data[x + 5] as u16)) as f64 * (1.0 - opacity) + val[2] as f64 * opacity).round() as u16;
                        self.data[x + 4] = (new_val >> 8) as u8;
                        self.data[x + 5] = new_val as u8;
                    }
                }
            },
            Colors::RGBA16(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 8;
                    for x in (temp_index_low..temp_index_high).step_by(8) {
                        let mut new_val: u16 = ((((self.data[x] as u16) << 8) | (self.data[x + 1] as u16)) as f64 * (1.0 - opacity) + val[0] as f64 * opacity).round() as u16;
                        self.data[x] = (new_val >> 8) as u8;
                        self.data[x + 1] = new_val as u8;
                        new_val = ((((self.data[x + 2] as u16) << 8) | (self.data[x + 3] as u16)) as f64 * (1.0 - opacity) + val[1] as f64 * opacity).round() as u16;
                        self.data[x + 2] = (new_val >> 8) as u8;
                        self.data[x + 3] = new_val as u8;
                        new_val = ((((self.data[x + 4] as u16) << 8) | (self.data[x + 5] as u16)) as f64 * (1.0 - opacity) + val[2] as f64 * opacity).round() as u16;
                        self.data[x + 4] = (new_val >> 8) as u8;
                        self.data[x + 5] = new_val as u8;
                        new_val = ((((self.data[x + 6] as u16) << 8) | (self.data[x + 7] as u16)) as f64 * (1.0 - opacity) + val[3] as f64 * opacity).round() as u16;
                        self.data[x + 6] = (new_val >> 8) as u8;
                        self.data[x + 7] = new_val as u8;
                    }
                }
            }
        }
        Ok(())
    }

    #[inline]
    fn fill_transparent_unchecked<RX: RangeBounds<usize>, RY: RangeBounds<usize>>(&mut self, index: (RX, RY), color: Colors, opacity: f64) {
        let index_x_lower: usize = match index.0.start_bound() {
            Bound::Included(&x) => {
                x
            }
            Bound::Excluded(&x) => {
                x + 1
            }
            Bound::Unbounded => {
                0
            }
        };

        let index_x_upper: usize = match index.0.end_bound() {
            Bound::Included(&x) => {
                x + 1
            }
            Bound::Excluded(&x) => {
                x
            }
            Bound::Unbounded => {
                self.width
            }
        };

        let index_y_lower: usize = match index.1.start_bound() {
            Bound::Included(&y) => {
                y
            }
            Bound::Excluded(&y) => {
                y + 1
            }
            Bound::Unbounded => {
                0
            }
        };

        let index_y_upper: usize = match index.1.end_bound() {
            Bound::Included(&y) => {
                y + 1
            }
            Bound::Excluded(&y) => {
                y
            }
            Bound::Unbounded => {
                self.height
            }
        };

        // background color aware ===> color = color + (new_color - color) * color_percentage ===> color = color * (1 - color_percentage) + new_color * color_percentage

        match color {
            Colors::GRAY8(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower);
                    for x in temp_index_low..temp_index_high {
                        self.data[x] = (self.data[x] as f64 * (1.0 - opacity) + val as f64 * opacity).round() as u8;
                    }
                }
            },
            Colors::GRAYA8(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 2;
                    for x in (temp_index_low..temp_index_high).step_by(2) {
                        self.data[x] = (self.data[x] as f64 * (1.0 - opacity) + val[0] as f64 * opacity).round() as u8;
                        self.data[x + 1] = (self.data[x + 1] as f64 * (1.0 - opacity) + val[1] as f64 * opacity).round() as u8;
                    }
                }
            },
            Colors::GRAY16(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 2;
                    for x in (temp_index_low..temp_index_high).step_by(2) {
                        let new_val: u16 = ((((self.data[x] as u16) << 8) | (self.data[x + 1] as u16)) as f64 * (1.0 - opacity) + val as f64 * opacity).round() as u16;
                        self.data[x] = (new_val >> 8) as u8;
                        self.data[x + 1] = new_val as u8;
                    }
                }
            },
            Colors::GRAYA16(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 4;
                    for x in (temp_index_low..temp_index_high).step_by(4) {
                        let mut new_val: u16 = ((((self.data[x] as u16) << 8) | (self.data[x + 1] as u16)) as f64 * (1.0 - opacity) + val[0] as f64 * opacity).round() as u16;
                        self.data[x] = (new_val >> 8) as u8;
                        self.data[x + 1] = new_val as u8;
                        new_val = ((((self.data[x + 2] as u16) << 8) | (self.data[x + 3] as u16)) as f64 * (1.0 - opacity) + val[1] as f64 * opacity).round() as u16;
                        self.data[x + 2] = (new_val >> 8) as u8;
                        self.data[x + 3] = new_val as u8;
                    }
                }
            },
            Colors::RGB8(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 3;
                    for x in (temp_index_low..temp_index_high).step_by(3) {
                        self.data[x] = (self.data[x] as f64 * (1.0 - opacity) + val[0] as f64 * opacity).round() as u8;
                        self.data[x + 1] = (self.data[x + 1] as f64 * (1.0 - opacity) + val[1] as f64 * opacity).round() as u8;
                        self.data[x + 2] = (self.data[x + 2] as f64 * (1.0 - opacity) + val[2] as f64 * opacity).round() as u8;
                    }
                }
            },
            Colors::RGBA8(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 4;
                    for x in (temp_index_low..temp_index_high).step_by(4) {
                        self.data[x] = (self.data[x] as f64 * (1.0 - opacity) + val[0] as f64 * opacity).round() as u8;
                        self.data[x + 1] = (self.data[x + 1] as f64 * (1.0 - opacity) + val[1] as f64 * opacity).round() as u8;
                        self.data[x + 2] = (self.data[x + 2] as f64 * (1.0 - opacity) + val[2] as f64 * opacity).round() as u8;
                        self.data[x + 3] = (self.data[x + 3] as f64 * (1.0 - opacity) + val[3] as f64 * opacity).round() as u8;
                    }
                }
            },
            Colors::RGB16(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 6;
                    for x in (temp_index_low..temp_index_high).step_by(6) {
                        let mut new_val: u16 = ((((self.data[x] as u16) << 8) | (self.data[x + 1] as u16)) as f64 * (1.0 - opacity) + val[0] as f64 * opacity).round() as u16;
                        self.data[x] = (new_val >> 8) as u8;
                        self.data[x + 1] = new_val as u8;
                        new_val = ((((self.data[x + 2] as u16) << 8) | (self.data[x + 3] as u16)) as f64 * (1.0 - opacity) + val[1] as f64 * opacity).round() as u16;
                        self.data[x + 2] = (new_val >> 8) as u8;
                        self.data[x + 3] = new_val as u8;
                        new_val = ((((self.data[x + 4] as u16) << 8) | (self.data[x + 5] as u16)) as f64 * (1.0 - opacity) + val[2] as f64 * opacity).round() as u16;
                        self.data[x + 4] = (new_val >> 8) as u8;
                        self.data[x + 5] = new_val as u8;
                    }
                }
            },
            Colors::RGBA16(val) => {
                for y in index_y_lower..index_y_upper {
                    let temp_index_low = self.index_unchecked((index_x_lower, y));
                    let temp_index_high = temp_index_low + (index_x_upper - index_x_lower) * 8;
                    for x in (temp_index_low..temp_index_high).step_by(8) {
                        let mut new_val: u16 = ((((self.data[x] as u16) << 8) | (self.data[x + 1] as u16)) as f64 * (1.0 - opacity) + val[0] as f64 * opacity).round() as u16;
                        self.data[x] = (new_val >> 8) as u8;
                        self.data[x + 1] = new_val as u8;
                        new_val = ((((self.data[x + 2] as u16) << 8) | (self.data[x + 3] as u16)) as f64 * (1.0 - opacity) + val[1] as f64 * opacity).round() as u16;
                        self.data[x + 2] = (new_val >> 8) as u8;
                        self.data[x + 3] = new_val as u8;
                        new_val = ((((self.data[x + 4] as u16) << 8) | (self.data[x + 5] as u16)) as f64 * (1.0 - opacity) + val[2] as f64 * opacity).round() as u16;
                        self.data[x + 4] = (new_val >> 8) as u8;
                        self.data[x + 5] = new_val as u8;
                        new_val = ((((self.data[x + 6] as u16) << 8) | (self.data[x + 7] as u16)) as f64 * (1.0 - opacity) + val[3] as f64 * opacity).round() as u16;
                        self.data[x + 6] = (new_val >> 8) as u8;
                        self.data[x + 7] = new_val as u8;
                    }
                }
            }
        }
    }
}

impl IO for Image {

    fn from_bytes(width: usize, height: usize, image_type: ImageType, bytes: &[u8]) -> Result<Image, IOError> {
        if bytes.len() != width * height * image_type.bytes_per_pixel() || bytes.is_empty() {
            return Err(IOError::InvalidSize);
        }
        match image_type {
            ImageType::GRAY8 => {},
            ImageType::GRAYA8 | ImageType::GRAY16 => {if bytes.len() % 2 != 0 {return Err(IOError::InvalidSize);}},
            ImageType::GRAYA16 | ImageType::RGBA8 => {if bytes.len() % 4 != 0 {return Err(IOError::InvalidSize);}},
            ImageType::RGB8 => {if bytes.len() % 3 != 0 {return Err(IOError::InvalidSize);}},
            ImageType::RGB16 => {if bytes.len() % 6 != 0 {return Err(IOError::InvalidSize);}},
            ImageType::RGBA16 => {if bytes.len() % 8 != 0 {return Err(IOError::InvalidSize);}},
        }

        let data: Vec<u8> = bytes.to_vec();
        let mut same_data: bool = true;

        match image_type {
            ImageType::GRAY8 => {
                let mut data_iterator = data.iter();
                let first_element = data_iterator.next().unwrap();
                same_data = data_iterator.all(|&x| x == *first_element);
            },
            ImageType::GRAYA8 | ImageType::GRAY16 => {
                for i in 0..2 {
                    for j in (i..(data.len() - 2)).step_by(2) {
                        if data[j] != data[j + 2] {
                            same_data = false;
                            break;
                        }
                    }
                }
            },
            ImageType::GRAYA16 | ImageType::RGBA8 => {
                for i in 0..4 {
                    for j in (i..(data.len() - 4)).step_by(4) {
                        if data[j] != data[j + 4] {
                            same_data = false;
                            break;
                        }
                    }
                }
            },
            ImageType::RGB8 => {
                for i in 0..3 {
                    for j in (i..(data.len() - 3)).step_by(3) {
                        if data[j] != data[j + 3] {
                            same_data = false;
                            break;
                        }
                    }
                }
            },
            ImageType::RGB16 => {
                for i in 0..6 {
                    for j in (i..(data.len() - 6)).step_by(6) {
                        if data[j] != data[j + 6] {
                            same_data = false;
                            break;
                        }
                    }
                }
            },
            ImageType::RGBA16 => {
                for i in 0..8 {
                    for j in (i..(data.len() - 8)).step_by(8) {
                        if data[j] != data[j + 8] {
                            same_data = false;
                            break;
                        }
                    }
                }
            },
        }

        let background_data = if same_data {
            BackgroundData::Color(match image_type {
                ImageType::GRAY8 => Colors::GRAY8(data[0]),
                ImageType::GRAYA8 => Colors::GRAYA8([data[0], data[1]]),
                ImageType::GRAY16 => Colors::GRAY16((data[0] as u16) << 8 | data[1] as u16),
                ImageType::GRAYA16 => Colors::GRAYA16([(data[0] as u16) << 8 | data[1] as u16, (data[2] as u16) << 8 | data[3] as u16]),
                ImageType::RGB8 => Colors::RGB8([data[0], data[1], data[2]]),
                ImageType::RGBA8 => Colors::RGBA8([data[0], data[1], data[2], data[3]]),
                ImageType::RGB16 => Colors::RGB16([(data[0] as u16) << 8 | data[1] as u16, (data[2] as u16) << 8 | data[3] as u16, (data[4] as u16) << 8 | data[5] as u16]),
                ImageType::RGBA16 => Colors::RGBA16([(data[0] as u16) << 8 | data[1] as u16, (data[2] as u16) << 8 | data[3] as u16, (data[4] as u16) << 8 | data[5] as u16, (data[6] as u16) << 8 | data[7] as u16]),
            })
        } else {
            BackgroundData::Image(data.clone())
        };

        Ok(
            Self {
                width,
                height,
                image_type,
                data,
                background_data,
            }
        )
    }

    #[inline]
    fn to_bytes(&self) -> Vec<u8> {
        self.to_bytes_ref().to_vec()
    }

    #[inline]
    fn to_bytes_ref(&self) -> &[u8] {
        &self.data
    }

    #[inline]
    fn to_bytes_ref_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    #[cfg(feature = "file_io")]
    fn from_file(path: &str) -> Result<Image, IOError> {

        let image: DynamicImage = match (
            match ImageReader::open(path) {
                Ok(image) => image,
                Err(err_type) => return Err(IOError::from(err_type.kind())),
            }
        ).decode() {
            Ok(image) => image,
            Err(err_type) => return Err(IOError::from(err_type)),
        };

        let img_type = match image.color() {
            ColorType::L8 => ImageType::GRAY8,
            ColorType::La8 => ImageType::GRAYA8,
            ColorType::L16 => ImageType::GRAY16,
            ColorType::La16 => ImageType::GRAYA16,
            ColorType::Rgb8 => ImageType::RGB8,
            ColorType::Rgba8 => ImageType::RGBA8,
            ColorType::Rgb16 => ImageType::RGB16,
            ColorType::Rgba16 => ImageType::RGBA16,
            _ => return Err(IOError::InvalidType),
        };

        Self::from_bytes(image.width() as usize, image.height() as usize, img_type, image.as_bytes())
    }

    #[cfg(feature = "file_io")]
    fn to_file(&self, path: &str, overwrite: bool) -> Result<(), IOError> {
        let file_path = Path::new(path);
        if file_path.is_file() {
            if overwrite {
                match remove_file(file_path) {
                    Ok(_) => (),
                    Err(err_type) => return Err(IOError::from(err_type.kind())),
                }
            } else {
                return Err(IOError::FileExists);
            }
        }
        save_buffer(file_path, self.to_bytes_ref(), self.width as u32, self.height as u32, ColorType::from(self.image_type))?;
        Ok(())
    }
}

impl Utilities for Image {

    #[inline]
    fn data(&self) -> &[u8] {
        &self.data
    }

    #[inline]
    fn width(&self) -> usize {
        self.width
    }

    #[inline]
    fn height(&self) -> usize {
        self.height
    }

    #[inline]
    fn image_type(&self) -> ImageType {
        self.image_type
    }

    fn clear(&mut self) {
        match &self.background_data {
            BackgroundData::Color(background_color) => {
                let color_slice: &[u8] = match *background_color {
                    Colors::GRAY8(color) => unsafe {slice_from_raw_parts(&[color] as *const u8, 1).as_ref().expect("Shouldn't fail!")},
                    Colors::GRAYA8(color) => unsafe {slice_from_raw_parts((color).as_ptr() as *const u8, 2).as_ref().expect("Shouldn't fail!")},
                    Colors::GRAY16(color) => unsafe {slice_from_raw_parts(&[color] as *const u16 as *const u8, 2).as_ref().expect("Shouldn't fail!")},
                    Colors::GRAYA16(color) => unsafe {slice_from_raw_parts((color).as_ptr() as *const u8, 4).as_ref().expect("Shouldn't fail!")},
                    Colors::RGB8(color) => unsafe {slice_from_raw_parts((color).as_ptr() as *const u8, 3).as_ref().expect("Shouldn't fail!")},
                    Colors::RGBA8(color) => unsafe {slice_from_raw_parts((color).as_ptr() as *const u8, 4).as_ref().expect("Shouldn't fail!")},
                    Colors::RGB16(color) => unsafe {slice_from_raw_parts((color).as_ptr() as *const u8, 6).as_ref().expect("Shouldn't fail!")},
                    Colors::RGBA16(color) => unsafe {slice_from_raw_parts((color).as_ptr() as *const u8, 8).as_ref().expect("Shouldn't fail!")},
                };
                let background_len = background_color.bytes_per_pixel();
                for x in 0..self.data.len() {
                    self.data[x] = color_slice[x % background_len];
                }
            },
            BackgroundData::Image(background_bytes) => {
                self.data = background_bytes.clone();
            },
        }
    }

    fn fill_image(&mut self, color: Colors) -> Result<(), DrawingError> {
        if ImageType::from(color) != self.image_type {
            return Err(DrawingError::WrongColor);
        }
        self.fill_unchecked((.., ..), color);
        Ok(())
    }

    fn save_background(&mut self) {
        let mut same_data: bool = true;

        match self.image_type {
            ImageType::GRAY8 => {
                let mut data_iterator = self.data.iter();
                let first_element = data_iterator.next().unwrap();
                same_data = data_iterator.all(|&x| x == *first_element);
            },
            ImageType::GRAYA8 | ImageType::GRAY16 => {
                for i in 0..2 {
                    for j in (i..(self.data.len() - 2)).step_by(2) {
                        if self.data[j] != self.data[j + 2] {
                            same_data = false;
                            break;
                        }
                    }
                }
            },
            ImageType::GRAYA16 | ImageType::RGBA8 => {
                for i in 0..4 {
                    for j in (i..(self.data.len() - 4)).step_by(4) {
                        if self.data[j] != self.data[j + 4] {
                            same_data = false;
                            break;
                        }
                    }
                }
            },
            ImageType::RGB8 => {
                for i in 0..3 {
                    for j in (i..(self.data.len() - 3)).step_by(3) {
                        if self.data[j] != self.data[j + 3] {
                            same_data = false;
                            break;
                        }
                    }
                }
            },
            ImageType::RGB16 => {
                for i in 0..6 {
                    for j in (i..(self.data.len() - 6)).step_by(6) {
                        if self.data[j] != self.data[j + 6] {
                            same_data = false;
                            break;
                        }
                    }
                }
            },
            ImageType::RGBA16 => {
                for i in 0..8 {
                    for j in (i..(self.data.len() - 8)).step_by(8) {
                        if self.data[j] != self.data[j + 8] {
                            same_data = false;
                            break;
                        }
                    }
                }
            },
        };

        self.background_data =
            if same_data {
                BackgroundData::Color(match self.image_type {
                    ImageType::GRAY8 => Colors::GRAY8(self.data[0]),
                    ImageType::GRAYA8 => Colors::GRAYA8([self.data[0], self.data[1]]),
                    ImageType::GRAY16 => Colors::GRAY16((self.data[0] as u16) << 8 | self.data[1] as u16),
                    ImageType::GRAYA16 => Colors::GRAYA16([(self.data[0] as u16) << 8 | self.data[1] as u16, (self.data[2] as u16) << 8 | self.data[3] as u16]),
                    ImageType::RGB8 => Colors::RGB8([self.data[0], self.data[1], self.data[2]]),
                    ImageType::RGBA8 => Colors::RGBA8([self.data[0], self.data[1], self.data[2], self.data[3]]),
                    ImageType::RGB16 => Colors::RGB16([(self.data[0] as u16) << 8 | self.data[1] as u16, (self.data[2] as u16) << 8 | self.data[3] as u16, (self.data[4] as u16) << 8 | self.data[5] as u16]),
                    ImageType::RGBA16 => Colors::RGBA16([(self.data[0] as u16) << 8 | self.data[1] as u16, (self.data[2] as u16) << 8 | self.data[3] as u16, (self.data[4] as u16) << 8 | self.data[5] as u16, (self.data[6] as u16) << 8 | self.data[7] as u16]),
                })
            } else {
                BackgroundData::Image(self.data.clone())
            }
    }
}

impl Image {
    pub fn new(width: usize, height: usize, background: Colors) -> Self {
        let color_slice: &[u8] = match background {
            Colors::GRAY8(color) => unsafe {slice_from_raw_parts(&[color] as *const u8, 1).as_ref().expect("Shouldn't fail!")},
            Colors::GRAYA8(color) => unsafe {slice_from_raw_parts((color).as_ptr() as *const u8, 2).as_ref().expect("Shouldn't fail!")},
            Colors::GRAY16(color) => unsafe {slice_from_raw_parts(&[color] as *const u16 as *const u8, 2).as_ref().expect("Shouldn't fail!")},
            Colors::GRAYA16(color) => unsafe {slice_from_raw_parts((color).as_ptr() as *const u8, 4).as_ref().expect("Shouldn't fail!")},
            Colors::RGB8(color) => unsafe {slice_from_raw_parts((color).as_ptr() as *const u8, 3).as_ref().expect("Shouldn't fail!")},
            Colors::RGBA8(color) => unsafe {slice_from_raw_parts((color).as_ptr() as *const u8, 4).as_ref().expect("Shouldn't fail!")},
            Colors::RGB16(color) => unsafe {slice_from_raw_parts((color).as_ptr() as *const u8, 6).as_ref().expect("Shouldn't fail!")},
            Colors::RGBA16(color) => unsafe {slice_from_raw_parts((color).as_ptr() as *const u8, 8).as_ref().expect("Shouldn't fail!")},
        };
        let background_len = background.bytes_per_pixel();
        let mut data: Vec<u8> = vec![0; width * height * background_len];
        for x in 0..data.len() {
            data[x] = color_slice[x % background_len];
        }

        Self {
            data,
            width,
            height,
            image_type: ImageType::from(background),
            background_data: BackgroundData::Color(background),
        }
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drawing_rectangle() {
        let mut image = Image::new(100, 100, Colors::RGB8([255, 255, 255]));

        // test errors
        if image.draw_rectangle((0, 0), (10, 10), Colors::RGBA8([0, 0, 0, 0]), 1, 1.0).is_ok() { panic!("Should fail!") }
        if image.draw_rectangle((0, 0), (10, 10), Colors::RGB8([0, 0, 0]), 0, 1.1).is_ok() { panic!("Should fail!") }

        // test drawing
        image.draw_rectangle((0, 0), (10, 10), Colors::RGB8([0, 0, 0]), 1, 1.0).unwrap();
        image.draw_rectangle((20, 20), (31, 31), Colors::RGB8([0, 0, 0]), 1, 0.5).unwrap();
        image.draw_rectangle((40, 40), (50, 50), Colors::RGB8([0, 0, 0]), 3, 1.0).unwrap();
        image.draw_rectangle((60, 60), (70, 70), Colors::RGB8([0, 0, 0]), 3, 0.5).unwrap();
        image.draw_rectangle((80, 80), (90, 90), Colors::RGB8([0, 0, 0]), 0, 1.0).unwrap();
        image.draw_rectangle((10, 90), (20, 80), Colors::RGB8([0, 0, 0]), 0, 0.5).unwrap();
        image.draw_rectangle((30, 70), (40, 60), Colors::RGB8([0, 0, 0]), 1000000, 1.0).unwrap();
        image.draw_rectangle((80, 10), (90, 30), Colors::RGB8([0, 0, 0]), 1000000, 0.5).unwrap();

        // image.to_file("test_drawing_rectangle.png", true).unwrap();
    }

    #[test]
    fn utilities_fields() {
        let image = Image::new(100, 100, Colors::GRAY8(255));

        assert_eq!(image.data(), &vec![255; 100 * 100]);
        assert_eq!(image.data(), &image.data);

        assert_eq!(image.width(), 100);
        assert_eq!(image.width(), image.width);

        assert_eq!(image.height(), 100);
        assert_eq!(image.height(), image.height);

        assert_eq!(image.image_type(), ImageType::GRAY8);
        assert_eq!(image.image_type(), image.image_type);
    }

    #[test]
    fn utilities_background() {
        let mut image = Image::new(100, 100, Colors::RGB8([100, 120, 140]));
        let image_original = image.clone();

        if image.fill_image(Colors::GRAY8(255)).is_ok() { panic!("Should fail!") }

        image.fill_image(Colors::RGB8([0, 0, 0])).unwrap();
        assert_ne!(image, image_original);

        image.clear();
        assert_eq!(image, image_original);

        image.fill_image(Colors::RGB8([130, 150, 170])).unwrap();
        image.save_background();
        image.clear();
        assert_ne!(image, image_original);
        assert_eq!(image.background_data, BackgroundData::Color(Colors::RGB8([130, 150, 170])));

        image.fill_image(Colors::RGB8([100, 120, 140])).unwrap();
        image.save_background();
        image.clear();
        assert_eq!(image, image_original);
        assert_eq!(image.background_data, BackgroundData::Color(Colors::RGB8([100, 120, 140])));

        image.set((0, image.height - 1), Colors::RGB8([0, 0, 0])).unwrap();
        image.save_background();
        image.clear();
        assert_ne!(image, image_original);

        let mut vec_to_match: Vec<u8> = [100, 120, 140].repeat(image.width * image.height);
        vec_to_match[..3].fill(0);
        assert_eq!(image.background_data, BackgroundData::Image(vec_to_match));
    }

    #[test]
    fn io_bytes() {
        let image = Image::new(100, 100, Colors::GRAY8(255));
        let bytes = image.to_bytes();
        let image2 = Image::from_bytes(100, 100, ImageType::GRAY8, &bytes).unwrap();

        assert_eq!(image, image2);
    }

    #[test]
    #[cfg(feature = "file_io")]
    fn io_gray8() {
        let image = Image::new(100, 100, Colors::GRAY8(255));
        image.to_file("test_io_gray8.png", true).unwrap();
        let image2 = Image::from_file("test_io_gray8.png").unwrap();
        remove_file("test_io_gray8.png").unwrap();

        assert_eq!(image, image2);
    }

    #[test]
    #[cfg(feature = "file_io")]
    fn io_graya8() {
        let image = Image::new(100, 100, Colors::GRAYA8([255, 255]));
        image.to_file("test_io_graya8.png", true).unwrap();
        let image2 = Image::from_file("test_io_graya8.png").unwrap();
        remove_file("test_io_graya8.png").unwrap();

        assert_eq!(image, image2);
    }

    #[test]
    #[cfg(feature = "file_io")]
    fn io_gray16() {
        let image = Image::new(100, 100, Colors::GRAY16(65535));
        image.to_file("test_io_gray16.png", true).unwrap();
        let image2 = Image::from_file("test_io_gray16.png").unwrap();
        remove_file("test_io_gray16.png").unwrap();

        assert_eq!(image, image2);
    }

    #[test]
    #[cfg(feature = "file_io")]
    fn io_graya16() {
        let image = Image::new(100, 100, Colors::GRAYA16([65535, 65535]));
        image.to_file("test_io_graya16.png", true).unwrap();
        let image2 = Image::from_file("test_io_graya16.png").unwrap();
        remove_file("test_io_graya16.png").unwrap();

        assert_eq!(image, image2);
    }

    #[test]
    #[cfg(feature = "file_io")]
    fn io_rgb8() {
        let image = Image::new(100, 100, Colors::RGB8([255, 255, 255]));
        image.to_file("test_io_rgb8.png", true).unwrap();
        let image2 = Image::from_file("test_io_rgb8.png").unwrap();
        remove_file("test_io_rgb8.png").unwrap();

        assert_eq!(image, image2);
    }

    #[test]
    #[cfg(feature = "file_io")]
    fn io_rgba8() {
        let image = Image::new(100, 100, Colors::RGBA8([255, 255, 255, 255]));
        image.to_file("test_io_rgba8.png", true).unwrap();
        let image2 = Image::from_file("test_io_rgba8.png").unwrap();
        remove_file("test_io_rgba8.png").unwrap();

        assert_eq!(image, image2);
    }

    #[test]
    #[cfg(feature = "file_io")]
    fn io_rgb16() {
        let image = Image::new(100, 100, Colors::RGB16([65535, 65535, 65535]));
        image.to_file("test_io_rgb16.png", true).unwrap();
        let image2 = Image::from_file("test_io_rgb16.png").unwrap();
        remove_file("test_io_rgb16.png").unwrap();

        assert_eq!(image, image2);
    }

    #[test]
    #[cfg(feature = "file_io")]
    fn io_rgba16() {
        let image = Image::new(100, 100, Colors::RGBA16([65535, 65535, 65535, 65535]));
        image.to_file("test_io_rgba16.png", true).unwrap();
        let image2 = Image::from_file("test_io_rgba16.png").unwrap();
        remove_file("test_io_rgba16.png").unwrap();

        assert_eq!(image, image2);
    }
}

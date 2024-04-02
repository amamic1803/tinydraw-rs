use crate::image::{Image, ImageType};
use crate::Colors;

/// A trait for converting between different color/image types.
pub trait Conversions {
    /// Calculates the average of a slice of u8 values.
    /// # Arguments
    /// * ```values``` - The slice of u8 values.
    /// # Returns
    /// * The average of the values.
    /// # Example
    /// ```
    /// use tinydraw::{Image, Conversions};
    ///
    /// let u8_values: [u8; 3] = [0, 128, 255];
    /// let average: u8 = <Image as Conversions>::average_u8(&u8_values);
    /// assert_eq!(average, 128_u8);
    /// ```
    fn average_u8(values: &[u8]) -> u8 {
        let mut sum: f64 = 0.0;
        for value in values {
            sum += *value as f64;
        }
        (sum / values.len() as f64).round() as u8
    }

    /// Calculates the average of a slice of u16 values.
    /// # Arguments
    /// * ```values``` - The slice of u16 values.
    /// # Returns
    /// * The average of the values.
    /// # Example
    /// ```
    /// use tinydraw::{Image, Conversions};
    ///
    /// let u16_values: [u16; 3] = [0, 32768, 65535];
    /// let average: u16 = <Image as Conversions>::average_u16(&u16_values);
    /// assert_eq!(average, 32768_u16);
    /// ```
    fn average_u16(values: &[u16]) -> u16 {
        let mut sum: f64 = 0.0;
        for value in values {
            sum += *value as f64;
        }
        (sum / values.len() as f64).round() as u16
    }

    /// Converts a u8 color value to a u16 color value.
    /// # Arguments
    /// * ```value``` - The u8 color value.
    /// # Returns
    /// * The u16 color value.
    /// # Example
    /// ```
    /// use tinydraw::{Image, Conversions};
    ///
    /// let value: u8 = 255;
    /// let converted_value: u16 = <Image as Conversions>::val_u8_to_u16(value);
    /// assert_eq!(converted_value, 65535_u16);
    ///
    /// let value: u8 = 0;
    /// let converted_value: u16 = <Image as Conversions>::val_u8_to_u16(value);
    /// assert_eq!(converted_value, 0_u16);
    ///
    /// let value: u8 = 128;
    /// let converted_value: u16 = <Image as Conversions>::val_u8_to_u16(value);
    /// assert_eq!(converted_value, 32896_u16);
    /// ```
    fn val_u8_to_u16(value: u8) -> u16 {
        // to convert to u16, we multiply by 257
        value as u16 * 257_u16
    }

    /// Converts a u16 color value to a u8 color value.
    /// # Arguments
    /// * ```value``` - The u16 color value.
    /// # Returns
    /// * The u8 color value.
    /// # Example
    /// ```
    /// use tinydraw::{Image, Conversions};
    ///
    /// let value: u16 = 65535;
    /// let converted_value: u8 = <Image as Conversions>::val_u16_to_u8(value);
    /// assert_eq!(converted_value, 255_u8);
    ///
    /// let value: u16 = 0;
    /// let converted_value: u8 = <Image as Conversions>::val_u16_to_u8(value);
    /// assert_eq!(converted_value, 0_u8);
    ///
    /// let value: u16 = 32896;
    /// let converted_value: u8 = <Image as Conversions>::val_u16_to_u8(value);
    /// assert_eq!(converted_value, 128_u8);
    /// ```
    fn val_u16_to_u8(value: u16) -> u8 {
        // to convert to u8, we divide by 257
        // divide by 257 is the same as multiply by 0.003_891_050_583_657_587_6
        (value as f64 * 0.003_891_050_583_657_587_6_f64).round() as u8
    }

    /// Checks if conversion to the specified color type is lossless.
    /// # Arguments
    /// * ```image_type1``` - The source color type.
    /// * ```image_type2``` - The destination color type.
    /// # Returns
    /// * ```true``` if conversion is lossless, ```false``` otherwise.
    /// # Example
    /// ```
    /// use tinydraw::{Image, ImageType, Conversions};
    ///
    /// let image_type1: ImageType = ImageType::GRAY8;
    /// let image_type2: ImageType = ImageType::RGB8;
    /// assert!(<Image as Conversions>::cvt_is_lossless(image_type1, image_type2));
    ///
    /// let image_type1: ImageType = ImageType::RGB8;
    /// let image_type2: ImageType = ImageType::GRAY8;
    /// assert!(!<Image as Conversions>::cvt_is_lossless(image_type1, image_type2));
    /// ```
    fn cvt_is_lossless(image_type1: ImageType, image_type2: ImageType) -> bool {
        match image_type1 {
            ImageType::GRAY8 => true,
            ImageType::GRAYA8 => match image_type2 {
                ImageType::GRAY8 => false,
                ImageType::GRAYA8 => true,
                ImageType::GRAY16 => false,
                ImageType::GRAYA16 => true,
                ImageType::RGB8 => false,
                ImageType::RGBA8 => true,
                ImageType::RGB16 => false,
                ImageType::RGBA16 => true,
            },
            ImageType::GRAY16 => match image_type2 {
                ImageType::GRAY8 => false,
                ImageType::GRAYA8 => false,
                ImageType::GRAY16 => true,
                ImageType::GRAYA16 => true,
                ImageType::RGB8 => false,
                ImageType::RGBA8 => false,
                ImageType::RGB16 => true,
                ImageType::RGBA16 => true,
            },
            ImageType::GRAYA16 => match image_type2 {
                ImageType::GRAY8 => false,
                ImageType::GRAYA8 => false,
                ImageType::GRAY16 => false,
                ImageType::GRAYA16 => true,
                ImageType::RGB8 => false,
                ImageType::RGBA8 => false,
                ImageType::RGB16 => false,
                ImageType::RGBA16 => true,
            },
            ImageType::RGB8 => match image_type2 {
                ImageType::GRAY8 => false,
                ImageType::GRAYA8 => false,
                ImageType::GRAY16 => false,
                ImageType::GRAYA16 => false,
                ImageType::RGB8 => true,
                ImageType::RGBA8 => true,
                ImageType::RGB16 => true,
                ImageType::RGBA16 => true,
            },
            ImageType::RGBA8 => match image_type2 {
                ImageType::GRAY8 => false,
                ImageType::GRAYA8 => false,
                ImageType::GRAY16 => false,
                ImageType::GRAYA16 => false,
                ImageType::RGB8 => false,
                ImageType::RGBA8 => true,
                ImageType::RGB16 => false,
                ImageType::RGBA16 => true,
            },
            ImageType::RGB16 => match image_type2 {
                ImageType::GRAY8 => false,
                ImageType::GRAYA8 => false,
                ImageType::GRAY16 => false,
                ImageType::GRAYA16 => false,
                ImageType::RGB8 => false,
                ImageType::RGBA8 => false,
                ImageType::RGB16 => true,
                ImageType::RGBA16 => true,
            },
            ImageType::RGBA16 => {
                matches!(image_type2, ImageType::RGBA16)
            }
        }
    }

    /// Converts the bytes from one color type to another.
    /// Not intended to be used directly.
    /// Use the ```convert``` method instead.
    /// # Arguments
    /// * ```img_bytes``` - The bytes of the image.
    /// * ```img_type``` - The color type of the image.
    /// * ```img_type_new``` - The color type to which the image will be converted.
    fn convert_bytes(data: &mut Vec<u8>, img_type: ImageType, img_type_new: ImageType) {
        match img_type {
            ImageType::GRAY8 => {
                match img_type_new {
                    ImageType::GRAY8 => {} // do nothing (same type)
                    ImageType::GRAYA8 => {
                        let original_len = data.len();

                        data.reserve_exact(original_len);
                        for _ in 0..original_len {
                            data.push(255);
                        }

                        for i in (0..original_len).rev() {
                            data[i << 1] = data[i];
                        }
                        for i in (1..original_len).step_by(2) {
                            data[i] = 255;
                        }
                    }
                    ImageType::GRAY16 => {
                        let original_len = data.len();

                        data.reserve_exact(original_len);
                        for _ in 0..original_len {
                            data.push(0);
                        }

                        for i in (0..original_len).rev() {
                            let new_val: u16 = Self::val_u8_to_u16(data[i]);
                            let new_loc: usize = i << 1;
                            data[new_loc] = (new_val >> 8) as u8;
                            data[new_loc + 1] = new_val as u8;
                        }
                    }
                    ImageType::GRAYA16 => {
                        let original_len = data.len();

                        let reserve_amount = original_len * 3;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..(reserve_amount) {
                            data.push(255);
                        }

                        for i in (0..original_len).rev() {
                            let new_val: u16 = Self::val_u8_to_u16(data[i]);
                            let new_loc: usize = i << 2;
                            data[new_loc] = (new_val >> 8) as u8;
                            data[new_loc + 1] = new_val as u8;
                        }

                        for i in (2..original_len).step_by(4) {
                            data[i] = 255;
                            data[i + 1] = 255;
                        }
                    }
                    ImageType::RGB8 => {
                        let original_len = data.len();

                        let reserve_amount = original_len * 2;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..(reserve_amount) {
                            data.push(0);
                        }

                        for i in (0..original_len).rev() {
                            let new_loc: usize = i * 3;
                            data[new_loc] = data[i];
                            data[new_loc + 1] = data[i];
                            data[new_loc + 2] = data[i];
                        }
                    }
                    ImageType::RGBA8 => {
                        let original_len = data.len();

                        let reserve_amount = original_len * 3;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..(reserve_amount) {
                            data.push(255);
                        }

                        for i in (0..original_len).rev() {
                            let new_loc: usize = i << 2;
                            data[new_loc] = data[i];
                            data[new_loc + 1] = data[i];
                            data[new_loc + 2] = data[i];
                        }

                        for i in (3..original_len).step_by(4) {
                            data[i] = 255;
                        }
                    }
                    ImageType::RGB16 => {
                        let original_len = data.len();

                        let reserve_amount = original_len * 5;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..(reserve_amount) {
                            data.push(0);
                        }

                        for i in (0..original_len).rev() {
                            let new_val: u16 = Self::val_u8_to_u16(data[i]);
                            let new_loc: usize = i * 6;
                            data[new_loc] = (new_val >> 8) as u8;
                            data[new_loc + 1] = new_val as u8;
                            data[new_loc + 2] = (new_val >> 8) as u8;
                            data[new_loc + 3] = new_val as u8;
                            data[new_loc + 4] = (new_val >> 8) as u8;
                            data[new_loc + 5] = new_val as u8;
                        }
                    }
                    ImageType::RGBA16 => {
                        let original_len = data.len();

                        let reserve_amount = original_len * 7;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..(reserve_amount) {
                            data.push(255);
                        }

                        for i in (0..original_len).rev() {
                            let new_val: u16 = Self::val_u8_to_u16(data[i]);
                            let new_loc: usize = i << 3;
                            data[new_loc] = (new_val >> 8) as u8;
                            data[new_loc + 1] = new_val as u8;
                            data[new_loc + 2] = (new_val >> 8) as u8;
                            data[new_loc + 3] = new_val as u8;
                            data[new_loc + 4] = (new_val >> 8) as u8;
                            data[new_loc + 5] = new_val as u8;
                        }

                        for i in (6..original_len).step_by(8) {
                            data[i] = 255;
                            data[i + 1] = 255;
                        }
                    }
                }
            }
            ImageType::GRAYA8 => {
                match img_type_new {
                    ImageType::GRAY8 => {
                        // bit shift to the right by 1 == divide by 2

                        for i in (0..data.len()).step_by(2) {
                            data[i >> 1] = data[i];
                        }

                        data.truncate(data.len() >> 1);
                        data.shrink_to_fit();
                    }
                    ImageType::GRAYA8 => {} // do nothing (same type)
                    ImageType::GRAY16 => {
                        for i in (0..data.len()).step_by(2) {
                            let new_val: u16 = Self::val_u8_to_u16(data[i]);
                            data[i] = (new_val >> 8) as u8;
                            data[i + 1] = new_val as u8;
                        }
                    }
                    ImageType::GRAYA16 => {
                        let original_len = data.len();

                        data.reserve_exact(original_len);
                        for _ in 0..(original_len) {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(2).rev() {
                            let new_val: u16 = Self::val_u8_to_u16(data[i]);
                            let new_transparency: u16 = Self::val_u8_to_u16(data[i + 1]);
                            let new_loc: usize = i << 1;
                            data[new_loc] = (new_val >> 8) as u8;
                            data[new_loc + 1] = new_val as u8;
                            data[new_loc + 2] = (new_transparency >> 8) as u8;
                            data[new_loc + 3] = new_transparency as u8;
                        }
                    }
                    ImageType::RGB8 => {
                        let original_len = data.len();

                        let reserve_amount = original_len >> 1;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..(reserve_amount) {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(2).rev() {
                            let new_loc: usize = i + (i >> 1);
                            data[new_loc] = data[i];
                            data[new_loc + 1] = data[i];
                            data[new_loc + 2] = data[i];
                        }
                    }
                    ImageType::RGBA8 => {
                        let original_len = data.len();

                        data.reserve_exact(original_len);
                        for _ in 0..(original_len) {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(2).rev() {
                            let new_loc: usize = i << 1;
                            data[new_loc + 3] = data[i + 1];

                            data[new_loc] = data[i];
                            data[new_loc + 1] = data[i];
                            data[new_loc + 2] = data[i];
                        }
                    }
                    ImageType::RGB16 => {
                        let original_len = data.len();

                        let reserve_amount = original_len << 1;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..(reserve_amount) {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(2).rev() {
                            let new_val: u16 = Self::val_u8_to_u16(data[i]);
                            let new_loc: usize = i * 3;
                            data[new_loc] = (new_val >> 8) as u8;
                            data[new_loc + 1] = new_val as u8;
                            data[new_loc + 2] = (new_val >> 8) as u8;
                            data[new_loc + 3] = new_val as u8;
                            data[new_loc + 4] = (new_val >> 8) as u8;
                            data[new_loc + 5] = new_val as u8;
                        }
                    }
                    ImageType::RGBA16 => {
                        let original_len = data.len();

                        let reserve_amount = original_len * 3;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..(reserve_amount) {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(2).rev() {
                            let new_val: u16 = Self::val_u8_to_u16(data[i]);
                            let new_transparency: u16 = Self::val_u8_to_u16(data[i + 1]);
                            let new_loc: usize = i << 2;
                            data[new_loc] = (new_val >> 8) as u8;
                            data[new_loc + 1] = new_val as u8;
                            data[new_loc + 2] = (new_val >> 8) as u8;
                            data[new_loc + 3] = new_val as u8;
                            data[new_loc + 4] = (new_val >> 8) as u8;
                            data[new_loc + 5] = new_val as u8;
                            data[new_loc + 6] = (new_transparency >> 8) as u8;
                            data[new_loc + 7] = new_transparency as u8;
                        }
                    }
                }
            }
            ImageType::GRAY16 => {
                match img_type_new {
                    ImageType::GRAY8 => {
                        for i in (0..data.len()).step_by(2) {
                            data[i >> 1] =
                                Self::val_u16_to_u8(((data[i] as u16) << 8) | (data[i + 1] as u16));
                        }

                        data.truncate(data.len() >> 1);
                        data.shrink_to_fit();
                    }
                    ImageType::GRAYA8 => {
                        for i in (0..data.len()).step_by(2) {
                            data[i] =
                                Self::val_u16_to_u8(((data[i] as u16) << 8) | (data[i + 1] as u16));
                            data[i + 1] = 255;
                        }
                    }
                    ImageType::GRAY16 => {} // do nothing (same type)
                    ImageType::GRAYA16 => {
                        let original_len = data.len();

                        data.reserve_exact(original_len);
                        for _ in 0..(original_len) {
                            data.push(255);
                        }

                        for i in (0..original_len).step_by(2).rev() {
                            let new_loc: usize = i << 1;
                            data[new_loc] = data[i];
                            data[new_loc + 1] = data[i + 1];
                        }

                        for i in (2..original_len).step_by(4) {
                            data[i] = 255;
                            data[i + 1] = 255;
                        }
                    }
                    ImageType::RGB8 => {
                        let original_len = data.len();

                        let reserve_amount = original_len >> 1;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..(reserve_amount) {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(2).rev() {
                            let new_val: u8 =
                                Self::val_u16_to_u8(((data[i] as u16) << 8) | (data[i + 1] as u16));
                            let new_loc: usize = i + (i >> 1);
                            data[new_loc] = new_val;
                            data[new_loc + 1] = new_val;
                            data[new_loc + 2] = new_val;
                        }
                    }
                    ImageType::RGBA8 => {
                        let original_len = data.len();

                        data.reserve_exact(original_len);
                        for _ in 0..(original_len) {
                            data.push(255);
                        }

                        for i in (0..original_len).step_by(2).rev() {
                            let new_val: u8 =
                                Self::val_u16_to_u8(((data[i] as u16) << 8) | (data[i + 1] as u16));
                            let new_loc: usize = i << 1;
                            data[new_loc] = new_val;
                            data[new_loc + 1] = new_val;
                            data[new_loc + 2] = new_val;
                        }

                        for i in (3..original_len).step_by(4) {
                            data[i] = 255;
                        }
                    }
                    ImageType::RGB16 => {
                        let original_len = data.len();

                        let reserve_amount = original_len << 1;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..(reserve_amount) {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(2).rev() {
                            let new_loc: usize = i * 3;
                            data[new_loc] = data[i];
                            data[new_loc + 1] = data[i + 1];
                            data[new_loc + 2] = data[i];
                            data[new_loc + 3] = data[i + 1];
                            data[new_loc + 4] = data[i];
                            data[new_loc + 5] = data[i + 1];
                        }
                    }
                    ImageType::RGBA16 => {
                        let original_len = data.len();

                        let reserve_amount = original_len * 3;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..(reserve_amount) {
                            data.push(255);
                        }

                        for i in (0..original_len).step_by(2).rev() {
                            let new_loc: usize = i << 2;
                            data[new_loc] = data[i];
                            data[new_loc + 1] = data[i + 1];
                            data[new_loc + 2] = data[i];
                            data[new_loc + 3] = data[i + 1];
                            data[new_loc + 4] = data[i];
                            data[new_loc + 5] = data[i + 1];
                        }

                        for i in (6..original_len).step_by(8) {
                            data[i] = 255;
                            data[i + 1] = 255;
                        }
                    }
                }
            }
            ImageType::GRAYA16 => {
                match img_type_new {
                    ImageType::GRAY8 => {
                        // bit shift to the right by 2 == divide by 4
                        for i in (0..data.len()).step_by(4) {
                            data[i >> 2] =
                                Self::val_u16_to_u8(((data[i] as u16) << 8) | (data[i + 1] as u16));
                        }

                        data.truncate(data.len() >> 2);
                        data.shrink_to_fit();
                    }
                    ImageType::GRAYA8 => {
                        for i in (0..data.len()).step_by(2) {
                            data[i >> 1] =
                                Self::val_u16_to_u8(((data[i] as u16) << 8) | (data[i + 1] as u16));
                        }

                        data.truncate(data.len() >> 1);
                        data.shrink_to_fit();
                    }
                    ImageType::GRAY16 => {
                        for i in (0..data.len()).step_by(4) {
                            let new_loc: usize = i >> 1;
                            data[new_loc] = data[i];
                            data[new_loc + 1] = data[i + 1];
                        }

                        data.truncate(data.len() >> 1);
                        data.shrink_to_fit();
                    }
                    ImageType::GRAYA16 => {} // do nothing (same type)
                    ImageType::RGB8 => {
                        for i in (0..data.len()).step_by(4) {
                            let new_val: u8 =
                                Self::val_u16_to_u8(((data[i] as u16) << 8) | (data[i + 1] as u16));
                            let new_loc: usize = (i >> 1) + (i >> 2);
                            data[new_loc] = new_val;
                            data[new_loc + 1] = new_val;
                            data[new_loc + 2] = new_val;
                        }

                        data.truncate((data.len() >> 1) + (data.len() >> 2));
                        data.shrink_to_fit();
                    }
                    ImageType::RGBA8 => {
                        for i in (0..data.len()).step_by(4) {
                            let new_val: u8 =
                                Self::val_u16_to_u8(((data[i] as u16) << 8) | (data[i + 1] as u16));
                            let new_transparency: u8 = Self::val_u16_to_u8(
                                ((data[i + 2] as u16) << 8) | (data[i + 3] as u16),
                            );
                            data[i] = new_val;
                            data[i + 1] = new_val;
                            data[i + 2] = new_val;
                            data[i + 3] = new_transparency;
                        }
                    }
                    ImageType::RGB16 => {
                        let original_len = data.len();

                        let reserve_amount = original_len >> 1;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..(reserve_amount) {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(4).rev() {
                            let new_loc: usize = i + (i >> 1);
                            data[new_loc] = data[i];
                            data[new_loc + 1] = data[i + 1];
                            data[new_loc + 2] = data[i];
                            data[new_loc + 3] = data[i + 1];
                            data[new_loc + 4] = data[i];
                            data[new_loc + 5] = data[i + 1];
                        }
                    }
                    ImageType::RGBA16 => {
                        let original_len = data.len();

                        data.reserve_exact(original_len);
                        for _ in 0..original_len {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(4).rev() {
                            let new_loc: usize = i << 1;
                            data[new_loc + 6] = data[i + 2];
                            data[new_loc + 7] = data[i + 3];

                            data[new_loc] = data[i];
                            data[new_loc + 1] = data[i + 1];
                            data[new_loc + 2] = data[i];
                            data[new_loc + 3] = data[i + 1];
                            data[new_loc + 4] = data[i];
                            data[new_loc + 5] = data[i + 1];
                        }
                    }
                }
            }
            ImageType::RGB8 => {
                match img_type_new {
                    ImageType::GRAY8 => {
                        for i in (0..data.len()).step_by(3) {
                            data[i / 3] = Self::average_u8(&data[i..(i + 3)]);
                        }

                        data.truncate(data.len() / 3);
                        data.shrink_to_fit();
                    }
                    ImageType::GRAYA8 => {
                        for i in (0..data.len()).step_by(3) {
                            let new_loc = (i / 3) << 1; // multiply by 2/3
                            data[new_loc] = Self::average_u8(&data[i..(i + 3)]);
                            data[new_loc + 1] = 255;
                        }

                        data.truncate((data.len() / 3) << 1);
                        data.shrink_to_fit();
                    }
                    ImageType::GRAY16 => {
                        for i in (0..data.len()).step_by(3) {
                            let new_loc = (i / 3) << 1; // multiply by 2/3
                            let new_val = Self::val_u8_to_u16(Self::average_u8(&data[i..(i + 3)]));
                            data[new_loc] = (new_val >> 8) as u8;
                            data[new_loc + 1] = new_val as u8;
                        }

                        data.truncate((data.len() / 3) << 1);
                        data.shrink_to_fit();
                    }
                    ImageType::GRAYA16 => {
                        let original_len = data.len();

                        let reserve_amount = original_len / 3;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..reserve_amount {
                            data.push(255);
                        }

                        for i in (0..original_len).step_by(3).rev() {
                            let new_loc = (i / 3) << 2; // multiply by 4/3
                            let new_val = Self::val_u8_to_u16(Self::average_u8(&data[i..(i + 3)]));
                            data[new_loc] = (new_val >> 8) as u8;
                            data[new_loc + 1] = new_val as u8;
                        }

                        for i in (2..original_len).step_by(4) {
                            data[i] = 255;
                            data[i + 1] = 255;
                        }
                    }
                    ImageType::RGB8 => {} // do nothing (same type)
                    ImageType::RGBA8 => {
                        let original_len = data.len();

                        let reserve_amount = original_len / 3;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..reserve_amount {
                            data.push(255);
                        }

                        for i in (0..original_len).step_by(3).rev() {
                            let new_loc = (i / 3) << 2; // multiply by 4/3
                            data[new_loc + 2] = data[i + 2];
                            data[new_loc + 1] = data[i + 1];
                            data[new_loc] = data[i];
                        }

                        for i in (3..original_len).step_by(4) {
                            data[i] = 255;
                        }
                    }
                    ImageType::RGB16 => {
                        let original_len = data.len();

                        data.reserve_exact(original_len);
                        for _ in 0..original_len {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(3).rev() {
                            let new_loc = i << 1; // multiply by 2

                            let new_val = Self::val_u8_to_u16(data[i + 2]);
                            data[new_loc + 4] = (new_val >> 8) as u8;
                            data[new_loc + 5] = new_val as u8;

                            let new_val = Self::val_u8_to_u16(data[i + 1]);
                            data[new_loc + 2] = (new_val >> 8) as u8;
                            data[new_loc + 3] = new_val as u8;

                            let new_val = Self::val_u8_to_u16(data[i]);
                            data[new_loc] = (new_val >> 8) as u8;
                            data[new_loc + 1] = new_val as u8;
                        }
                    }
                    ImageType::RGBA16 => {
                        let original_len = data.len();

                        let reserve_amount = 5 * (original_len / 3);
                        data.reserve_exact(reserve_amount);
                        for _ in 0..reserve_amount {
                            data.push(255);
                        }

                        for i in (0..original_len).step_by(3).rev() {
                            let new_loc = (i / 3) * 8; // multiply by 8/3

                            let new_val = Self::val_u8_to_u16(data[i + 2]);
                            data[new_loc + 4] = (new_val >> 8) as u8;
                            data[new_loc + 5] = new_val as u8;

                            let new_val = Self::val_u8_to_u16(data[i + 1]);
                            data[new_loc + 2] = (new_val >> 8) as u8;
                            data[new_loc + 3] = new_val as u8;

                            let new_val = Self::val_u8_to_u16(data[i]);
                            data[new_loc] = (new_val >> 8) as u8;
                            data[new_loc + 1] = new_val as u8;
                        }

                        for i in (6..original_len).step_by(8) {
                            data[i] = 255;
                            data[i + 1] = 255;
                        }
                    }
                }
            }
            ImageType::RGBA8 => {
                match img_type_new {
                    ImageType::GRAY8 => {
                        for i in (0..data.len()).step_by(4) {
                            data[i >> 2] = Self::average_u8(&data[i..(i + 3)]);
                        }

                        data.truncate(data.len() >> 2);
                        data.shrink_to_fit();
                    }
                    ImageType::GRAYA8 => {
                        for i in (0..data.len()).step_by(4) {
                            let new_loc = i >> 1; // divide by 2
                            data[new_loc] = Self::average_u8(&data[i..(i + 3)]);
                            data[new_loc + 1] = data[i + 3];
                        }

                        data.truncate(data.len() >> 1);
                        data.shrink_to_fit();
                    }
                    ImageType::GRAY16 => {
                        for i in (0..data.len()).step_by(4) {
                            let new_loc = i >> 1; // divide by 2
                            let new_val = Self::val_u8_to_u16(Self::average_u8(&data[i..(i + 3)]));
                            data[new_loc] = (new_val >> 8) as u8;
                            data[new_loc + 1] = new_val as u8;
                        }

                        data.truncate(data.len() >> 1);
                        data.shrink_to_fit();
                    }
                    ImageType::GRAYA16 => {
                        for i in (0..data.len()).step_by(4) {
                            let new_val = Self::val_u8_to_u16(Self::average_u8(&data[i..(i + 3)]));
                            let new_transparency = Self::val_u8_to_u16(data[i + 3]);
                            data[i] = (new_val >> 8) as u8;
                            data[i + 1] = new_val as u8;
                            data[i + 2] = (new_transparency >> 8) as u8;
                            data[i + 3] = new_transparency as u8;
                        }
                    }
                    ImageType::RGB8 => {
                        for i in (0..data.len()).step_by(4) {
                            let new_loc = (i >> 1) + (i >> 2); // multiply by 3/4
                            data[new_loc] = data[i];
                            data[new_loc + 1] = data[i + 1];
                            data[new_loc + 2] = data[i + 2];
                        }

                        data.truncate((data.len() >> 1) + (data.len() >> 2));
                        data.shrink_to_fit();
                    }
                    ImageType::RGBA8 => {} // do nothing (same type)
                    ImageType::RGB16 => {
                        let original_len = data.len();

                        let reserve_amount = original_len >> 1;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..reserve_amount {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(4).rev() {
                            let new_loc = i + (i >> 1); // multiply by 3/2

                            let new_val = Self::val_u8_to_u16(data[i + 2]);
                            data[new_loc + 4] = (new_val >> 8) as u8;
                            data[new_loc + 5] = new_val as u8;

                            let new_val = Self::val_u8_to_u16(data[i + 1]);
                            data[new_loc + 2] = (new_val >> 8) as u8;
                            data[new_loc + 3] = new_val as u8;

                            let new_val = Self::val_u8_to_u16(data[i]);
                            data[new_loc] = (new_val >> 8) as u8;
                            data[new_loc + 1] = new_val as u8;
                        }
                    }
                    ImageType::RGBA16 => {
                        let original_len = data.len();

                        data.reserve_exact(original_len);
                        for _ in 0..original_len {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(4).rev() {
                            let new_loc = i << 1; // multiply by 2

                            let new_val = Self::val_u8_to_u16(data[i + 3]);
                            data[new_loc + 6] = (new_val >> 8) as u8;
                            data[new_loc + 7] = new_val as u8;

                            let new_val = Self::val_u8_to_u16(data[i + 2]);
                            data[new_loc + 4] = (new_val >> 8) as u8;
                            data[new_loc + 5] = new_val as u8;

                            let new_val = Self::val_u8_to_u16(data[i + 1]);
                            data[new_loc + 2] = (new_val >> 8) as u8;
                            data[new_loc + 3] = new_val as u8;

                            let new_val = Self::val_u8_to_u16(data[i]);
                            data[new_loc] = (new_val >> 8) as u8;
                            data[new_loc + 1] = new_val as u8;
                        }
                    }
                }
            }
            ImageType::RGB16 => {
                match img_type_new {
                    ImageType::GRAY8 => {
                        for i in (0..data.len()).step_by(6) {
                            data[i / 6] = Self::val_u16_to_u8(Self::average_u16(&[
                                (data[i] as u16) << 8 | data[i + 1] as u16,
                                (data[i + 2] as u16) << 8 | data[i + 3] as u16,
                                (data[i + 4] as u16) << 8 | data[i + 5] as u16,
                            ]));
                        }

                        data.truncate(data.len() / 6);
                        data.shrink_to_fit();
                    }
                    ImageType::GRAYA8 => {
                        for i in (0..data.len()).step_by(6) {
                            let new_loc = i / 3;
                            data[new_loc] = Self::val_u16_to_u8(Self::average_u16(&[
                                (data[i] as u16) << 8 | data[i + 1] as u16,
                                (data[i + 2] as u16) << 8 | data[i + 3] as u16,
                                (data[i + 4] as u16) << 8 | data[i + 5] as u16,
                            ]));
                            data[new_loc + 1] = 255;
                        }

                        data.truncate(data.len() / 3);
                        data.shrink_to_fit();
                    }
                    ImageType::GRAY16 => {
                        for i in (0..data.len()).step_by(6) {
                            let new_val = Self::average_u16(&[
                                (data[i] as u16) << 8 | data[i + 1] as u16,
                                (data[i + 2] as u16) << 8 | data[i + 3] as u16,
                                (data[i + 4] as u16) << 8 | data[i + 5] as u16,
                            ]);
                            let new_loc = i / 3;
                            data[new_loc] = (new_val >> 8) as u8;
                            data[new_loc + 1] = new_val as u8;
                        }

                        data.truncate(data.len() / 3);
                        data.shrink_to_fit();
                    }
                    ImageType::GRAYA16 => {
                        for i in (0..data.len()).step_by(6) {
                            let new_val = Self::average_u16(&[
                                (data[i] as u16) << 8 | data[i + 1] as u16,
                                (data[i + 2] as u16) << 8 | data[i + 3] as u16,
                                (data[i + 4] as u16) << 8 | data[i + 5] as u16,
                            ]);
                            let new_loc = (i / 3) << 1;
                            data[new_loc] = (new_val >> 8) as u8;
                            data[new_loc + 1] = new_val as u8;
                            data[new_loc + 2] = 255;
                            data[new_loc + 3] = 255;
                        }

                        data.truncate((data.len() / 3) << 1);
                        data.shrink_to_fit();
                    }
                    ImageType::RGB8 => {
                        for i in (0..data.len()).step_by(2) {
                            data[i >> 1] =
                                Self::val_u16_to_u8((data[i] as u16) << 8 | data[i + 1] as u16);
                        }

                        data.truncate(data.len() >> 1);
                        data.shrink_to_fit();
                    }
                    ImageType::RGBA8 => {
                        for i in (0..data.len()).step_by(6) {
                            let new_loc = (i / 3) << 1; // multiply by 2/3
                            data[new_loc] =
                                Self::val_u16_to_u8((data[i] as u16) << 8 | data[i + 1] as u16);
                            data[new_loc + 1] =
                                Self::val_u16_to_u8((data[i + 2] as u16) << 8 | data[i + 3] as u16);
                            data[new_loc + 2] =
                                Self::val_u16_to_u8((data[i + 4] as u16) << 8 | data[i + 5] as u16);
                            data[new_loc + 3] = 255;
                        }

                        data.truncate((data.len() / 3) << 1);
                        data.shrink_to_fit();
                    }
                    ImageType::RGB16 => {} // do nothing (same type)
                    ImageType::RGBA16 => {
                        let original_len = data.len();

                        let reserve_len = original_len / 3;
                        data.reserve_exact(reserve_len);
                        for _ in 0..(reserve_len) {
                            data.push(255);
                        }

                        for i in (0..original_len).step_by(6).rev() {
                            let new_loc = (i / 3) << 2; // multiply by 4/3
                            data[new_loc + 5] = data[i + 5];
                            data[new_loc + 4] = data[i + 4];
                            data[new_loc + 3] = data[i + 3];
                            data[new_loc + 2] = data[i + 2];
                            data[new_loc + 1] = data[i + 1];
                            data[new_loc] = data[i];
                        }

                        for i in (6..original_len).step_by(8) {
                            data[i] = 255;
                            data[i + 1] = 255;
                        }
                    }
                }
            }
            ImageType::RGBA16 => {
                match img_type_new {
                    ImageType::GRAY8 => {
                        for i in (0..data.len()).step_by(8) {
                            data[i >> 3] = Self::val_u16_to_u8(Self::average_u16(&[
                                (data[i] as u16) << 8 | data[i + 1] as u16,
                                (data[i + 2] as u16) << 8 | data[i + 3] as u16,
                                (data[i + 4] as u16) << 8 | data[i + 5] as u16,
                            ]));
                        }

                        data.truncate(data.len() >> 3);
                        data.shrink_to_fit();
                    }
                    ImageType::GRAYA8 => {
                        for i in (0..data.len()).step_by(8) {
                            let new_loc = i >> 2; // divide by 4
                            data[new_loc] = Self::val_u16_to_u8(Self::average_u16(&[
                                (data[i] as u16) << 8 | data[i + 1] as u16,
                                (data[i + 2] as u16) << 8 | data[i + 3] as u16,
                                (data[i + 4] as u16) << 8 | data[i + 5] as u16,
                            ]));
                            data[new_loc + 1] =
                                Self::val_u16_to_u8((data[i + 6] as u16) << 8 | data[i + 7] as u16);
                        }

                        data.truncate(data.len() >> 2);
                        data.shrink_to_fit();
                    }
                    ImageType::GRAY16 => {
                        for i in (0..data.len()).step_by(8) {
                            let new_loc = i >> 2; // divide by 4
                            let new_val = Self::average_u16(&[
                                (data[i] as u16) << 8 | data[i + 1] as u16,
                                (data[i + 2] as u16) << 8 | data[i + 3] as u16,
                                (data[i + 4] as u16) << 8 | data[i + 5] as u16,
                            ]);
                            data[new_loc] = (new_val >> 8) as u8;
                            data[new_loc + 1] = new_val as u8;
                        }

                        data.truncate(data.len() >> 2);
                        data.shrink_to_fit();
                    }
                    ImageType::GRAYA16 => {
                        for i in (0..data.len()).step_by(8) {
                            let new_loc = i >> 1; // divide by 2
                            let new_val = Self::average_u16(&[
                                (data[i] as u16) << 8 | data[i + 1] as u16,
                                (data[i + 2] as u16) << 8 | data[i + 3] as u16,
                                (data[i + 4] as u16) << 8 | data[i + 5] as u16,
                            ]);
                            data[new_loc] = (new_val >> 8) as u8;
                            data[new_loc + 1] = new_val as u8;
                            data[new_loc + 2] = data[i + 6];
                            data[new_loc + 3] = data[i + 7];
                        }

                        data.truncate(data.len() >> 1);
                        data.shrink_to_fit();
                    }
                    ImageType::RGB8 => {
                        for i in (0..data.len()).step_by(8) {
                            let new_loc = (i >> 3) + (i >> 2); // multiply by 3/8
                            data[new_loc] =
                                Self::val_u16_to_u8((data[i] as u16) << 8 | data[i + 1] as u16);
                            data[new_loc + 1] =
                                Self::val_u16_to_u8((data[i + 2] as u16) << 8 | data[i + 3] as u16);
                            data[new_loc + 2] =
                                Self::val_u16_to_u8((data[i + 4] as u16) << 8 | data[i + 5] as u16);
                        }

                        data.truncate((data.len() >> 3) + (data.len() >> 2));
                        data.shrink_to_fit();
                    }
                    ImageType::RGBA8 => {
                        for i in (0..data.len()).step_by(8) {
                            let new_loc = i >> 1; // divide by 2
                            data[new_loc] =
                                Self::val_u16_to_u8((data[i] as u16) << 8 | data[i + 1] as u16);
                            data[new_loc + 1] =
                                Self::val_u16_to_u8((data[i + 2] as u16) << 8 | data[i + 3] as u16);
                            data[new_loc + 2] =
                                Self::val_u16_to_u8((data[i + 4] as u16) << 8 | data[i + 5] as u16);
                            data[new_loc + 3] =
                                Self::val_u16_to_u8((data[i + 6] as u16) << 8 | data[i + 7] as u16);
                        }

                        data.truncate(data.len() >> 1);
                        data.shrink_to_fit();
                    }
                    ImageType::RGB16 => {
                        for i in (0..data.len()).step_by(8) {
                            let new_loc = (i >> 1) + (i >> 2); // multiply by 3/4
                            data[new_loc] = data[i];
                            data[new_loc + 1] = data[i + 1];
                            data[new_loc + 2] = data[i + 2];
                            data[new_loc + 3] = data[i + 3];
                            data[new_loc + 4] = data[i + 4];
                            data[new_loc + 5] = data[i + 5];
                        }

                        data.truncate((data.len() >> 1) + (data.len() >> 2));
                        data.shrink_to_fit();
                    }
                    ImageType::RGBA16 => {} // do nothing (same type)
                }
            }
        }
    }

    /// Converts the image to the specified color type.
    /// # Arguments
    /// * ```image_type``` - The color type to which the image will be converted.
    fn convert(&mut self, image_type: ImageType);
}

impl Conversions for Image {
    fn convert(&mut self, image_type: ImageType) {
        if self.image_type != image_type {
            // convert image data
            Self::convert_bytes(&mut self.data, self.image_type, image_type);

            // convert background data
            match &mut self.background_data {
                crate::image::BackgroundData::Color(colors_enum) => {
                    match *colors_enum {
                        Colors::GRAY8(color) => {
                            match image_type {
                                ImageType::GRAY8 => {} // do nothing (same type)
                                ImageType::GRAYA8 => *colors_enum = Colors::GRAYA8([color, 255]),
                                ImageType::GRAY16 => {
                                    *colors_enum = Colors::GRAY16(Self::val_u8_to_u16(color))
                                }
                                ImageType::GRAYA16 => {
                                    *colors_enum =
                                        Colors::GRAYA16([Self::val_u8_to_u16(color), 65535])
                                }
                                ImageType::RGB8 => {
                                    *colors_enum = Colors::RGB8([color, color, color])
                                }
                                ImageType::RGBA8 => {
                                    *colors_enum = Colors::RGBA8([color, color, color, 255])
                                }
                                ImageType::RGB16 => {
                                    let new_val = Self::val_u8_to_u16(color);
                                    *colors_enum = Colors::RGB16([new_val, new_val, new_val])
                                }
                                ImageType::RGBA16 => {
                                    let new_val = Self::val_u8_to_u16(color);
                                    *colors_enum =
                                        Colors::RGBA16([new_val, new_val, new_val, 65535])
                                }
                            }
                        }
                        Colors::GRAYA8(color) => {
                            match image_type {
                                ImageType::GRAY8 => *colors_enum = Colors::GRAY8(color[0]),
                                ImageType::GRAYA8 => {} // do nothing (same type)
                                ImageType::GRAY16 => {
                                    *colors_enum = Colors::GRAY16(Self::val_u8_to_u16(color[0]))
                                }
                                ImageType::GRAYA16 => {
                                    *colors_enum = Colors::GRAYA16([
                                        Self::val_u8_to_u16(color[0]),
                                        Self::val_u8_to_u16(color[1]),
                                    ])
                                }
                                ImageType::RGB8 => {
                                    *colors_enum = Colors::RGB8([color[0], color[0], color[0]])
                                }
                                ImageType::RGBA8 => {
                                    *colors_enum =
                                        Colors::RGBA8([color[0], color[0], color[0], color[1]])
                                }
                                ImageType::RGB16 => {
                                    let new_val = Self::val_u8_to_u16(color[0]);
                                    *colors_enum = Colors::RGB16([new_val, new_val, new_val])
                                }
                                ImageType::RGBA16 => {
                                    let new_val = Self::val_u8_to_u16(color[0]);
                                    *colors_enum = Colors::RGBA16([
                                        new_val,
                                        new_val,
                                        new_val,
                                        Self::val_u8_to_u16(color[1]),
                                    ])
                                }
                            }
                        }
                        Colors::GRAY16(color) => {
                            match image_type {
                                ImageType::GRAY8 => {
                                    *colors_enum = Colors::GRAY8(Self::val_u16_to_u8(color))
                                }
                                ImageType::GRAYA8 => {
                                    *colors_enum = Colors::GRAYA8([Self::val_u16_to_u8(color), 255])
                                }
                                ImageType::GRAY16 => {} // do nothing (same type)
                                ImageType::GRAYA16 => {
                                    *colors_enum = Colors::GRAYA16([color, 65535])
                                }
                                ImageType::RGB8 => {
                                    let new_val = Self::val_u16_to_u8(color);
                                    *colors_enum = Colors::RGB8([new_val, new_val, new_val])
                                }
                                ImageType::RGBA8 => {
                                    let new_val = Self::val_u16_to_u8(color);
                                    *colors_enum = Colors::RGBA8([new_val, new_val, new_val, 255])
                                }
                                ImageType::RGB16 => {
                                    *colors_enum = Colors::RGB16([color, color, color])
                                }
                                ImageType::RGBA16 => {
                                    *colors_enum = Colors::RGBA16([color, color, color, 65535])
                                }
                            }
                        }
                        Colors::GRAYA16(color) => {
                            match image_type {
                                ImageType::GRAY8 => {
                                    *colors_enum = Colors::GRAY8(Self::val_u16_to_u8(color[0]))
                                }
                                ImageType::GRAYA8 => {
                                    *colors_enum = Colors::GRAYA8([
                                        Self::val_u16_to_u8(color[0]),
                                        Self::val_u16_to_u8(color[1]),
                                    ])
                                }
                                ImageType::GRAY16 => *colors_enum = Colors::GRAY16(color[0]),
                                ImageType::GRAYA16 => {} // do nothing (same type)
                                ImageType::RGB8 => {
                                    let new_val = Self::val_u16_to_u8(color[0]);
                                    *colors_enum = Colors::RGB8([new_val, new_val, new_val])
                                }
                                ImageType::RGBA8 => {
                                    let new_val = Self::val_u16_to_u8(color[0]);
                                    *colors_enum = Colors::RGBA8([
                                        new_val,
                                        new_val,
                                        new_val,
                                        Self::val_u16_to_u8(color[1]),
                                    ])
                                }
                                ImageType::RGB16 => {
                                    *colors_enum = Colors::RGB16([color[0], color[0], color[0]])
                                }
                                ImageType::RGBA16 => {
                                    *colors_enum =
                                        Colors::RGBA16([color[0], color[0], color[0], color[1]])
                                }
                            }
                        }
                        Colors::RGB8(color) => {
                            match image_type {
                                ImageType::GRAY8 => {
                                    *colors_enum = Colors::GRAY8(Self::average_u8(&color))
                                }
                                ImageType::GRAYA8 => {
                                    *colors_enum = Colors::GRAYA8([Self::average_u8(&color), 255])
                                }
                                ImageType::GRAY16 => {
                                    *colors_enum = Colors::GRAY16(Self::val_u8_to_u16(
                                        Self::average_u8(&color),
                                    ))
                                }
                                ImageType::GRAYA16 => {
                                    *colors_enum = Colors::GRAYA16([
                                        Self::val_u8_to_u16(Self::average_u8(&color)),
                                        65535,
                                    ])
                                }
                                ImageType::RGB8 => {} // do nothing (same type)
                                ImageType::RGBA8 => {
                                    *colors_enum =
                                        Colors::RGBA8([color[0], color[1], color[2], 255])
                                }
                                ImageType::RGB16 => {
                                    *colors_enum = Colors::RGB16([
                                        Self::val_u8_to_u16(color[0]),
                                        Self::val_u8_to_u16(color[1]),
                                        Self::val_u8_to_u16(color[2]),
                                    ])
                                }
                                ImageType::RGBA16 => {
                                    *colors_enum = Colors::RGBA16([
                                        Self::val_u8_to_u16(color[0]),
                                        Self::val_u8_to_u16(color[1]),
                                        Self::val_u8_to_u16(color[2]),
                                        65535,
                                    ])
                                }
                            }
                        }
                        Colors::RGBA8(color) => {
                            match image_type {
                                ImageType::GRAY8 => {
                                    *colors_enum = Colors::GRAY8(Self::average_u8(&color[..3]))
                                }
                                ImageType::GRAYA8 => {
                                    *colors_enum =
                                        Colors::GRAYA8([Self::average_u8(&color[..3]), color[3]])
                                }
                                ImageType::GRAY16 => {
                                    *colors_enum = Colors::GRAY16(Self::val_u8_to_u16(
                                        Self::average_u8(&color[..3]),
                                    ))
                                }
                                ImageType::GRAYA16 => {
                                    *colors_enum = Colors::GRAYA16([
                                        Self::val_u8_to_u16(Self::average_u8(&color[..3])),
                                        Self::val_u8_to_u16(color[3]),
                                    ])
                                }
                                ImageType::RGB8 => {
                                    *colors_enum = Colors::RGB8([color[0], color[1], color[2]])
                                }
                                ImageType::RGBA8 => {} // do nothing (same type)
                                ImageType::RGB16 => {
                                    *colors_enum = Colors::RGB16([
                                        Self::val_u8_to_u16(color[0]),
                                        Self::val_u8_to_u16(color[1]),
                                        Self::val_u8_to_u16(color[2]),
                                    ])
                                }
                                ImageType::RGBA16 => {
                                    *colors_enum = Colors::RGBA16([
                                        Self::val_u8_to_u16(color[0]),
                                        Self::val_u8_to_u16(color[1]),
                                        Self::val_u8_to_u16(color[2]),
                                        Self::val_u8_to_u16(color[3]),
                                    ])
                                }
                            }
                        }
                        Colors::RGB16(color) => {
                            match image_type {
                                ImageType::GRAY8 => {
                                    *colors_enum = Colors::GRAY8(Self::val_u16_to_u8(
                                        Self::average_u16(&color),
                                    ))
                                }
                                ImageType::GRAYA8 => {
                                    *colors_enum = Colors::GRAYA8([
                                        Self::val_u16_to_u8(Self::average_u16(&color)),
                                        255,
                                    ])
                                }
                                ImageType::GRAY16 => {
                                    *colors_enum = Colors::GRAY16(Self::average_u16(&color))
                                }
                                ImageType::GRAYA16 => {
                                    *colors_enum =
                                        Colors::GRAYA16([Self::average_u16(&color), 65535])
                                }
                                ImageType::RGB8 => {
                                    *colors_enum = Colors::RGB8([
                                        Self::val_u16_to_u8(color[0]),
                                        Self::val_u16_to_u8(color[1]),
                                        Self::val_u16_to_u8(color[2]),
                                    ])
                                }
                                ImageType::RGBA8 => {
                                    *colors_enum = Colors::RGBA8([
                                        Self::val_u16_to_u8(color[0]),
                                        Self::val_u16_to_u8(color[1]),
                                        Self::val_u16_to_u8(color[2]),
                                        255,
                                    ])
                                }
                                ImageType::RGB16 => {} // do nothing (same type)
                                ImageType::RGBA16 => {
                                    *colors_enum =
                                        Colors::RGBA16([color[0], color[1], color[2], 65535])
                                }
                            }
                        }
                        Colors::RGBA16(color) => {
                            match image_type {
                                ImageType::GRAY8 => {
                                    *colors_enum = Colors::GRAY8(Self::val_u16_to_u8(
                                        Self::average_u16(&color[..3]),
                                    ))
                                }
                                ImageType::GRAYA8 => {
                                    *colors_enum = Colors::GRAYA8([
                                        Self::val_u16_to_u8(Self::average_u16(&color[..3])),
                                        Self::val_u16_to_u8(color[3]),
                                    ])
                                }
                                ImageType::GRAY16 => {
                                    *colors_enum = Colors::GRAY16(Self::average_u16(&color[..3]))
                                }
                                ImageType::GRAYA16 => {
                                    *colors_enum =
                                        Colors::GRAYA16([Self::average_u16(&color[..3]), color[3]])
                                }
                                ImageType::RGB8 => {
                                    *colors_enum = Colors::RGB8([
                                        Self::val_u16_to_u8(color[0]),
                                        Self::val_u16_to_u8(color[1]),
                                        Self::val_u16_to_u8(color[2]),
                                    ])
                                }
                                ImageType::RGBA8 => {
                                    *colors_enum = Colors::RGBA8([
                                        Self::val_u16_to_u8(color[0]),
                                        Self::val_u16_to_u8(color[1]),
                                        Self::val_u16_to_u8(color[2]),
                                        Self::val_u16_to_u8(color[3]),
                                    ])
                                }
                                ImageType::RGB16 => {
                                    *colors_enum = Colors::RGB16([color[0], color[1], color[2]])
                                }
                                ImageType::RGBA16 => {} // do nothing (same type)
                            }
                        }
                    }
                }
                crate::image::BackgroundData::Image(data) => {
                    Self::convert_bytes(data, self.image_type, image_type);
                }
            }

            // change image type
            self.image_type = image_type;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Colors, Image, ImageType, Indexing, Utilities};

    fn conversion_test(img1_colors: (Colors, Colors), img2_colors: (Colors, Colors)) {
        assert_eq!(
            ImageType::from(img1_colors.0),
            ImageType::from(img1_colors.1)
        );
        assert_eq!(
            ImageType::from(img2_colors.0),
            ImageType::from(img2_colors.1)
        );

        let new_image_type = ImageType::from(img2_colors.0);

        // simple conversion
        let mut img1 = Image::new(100, 100, img1_colors.0);
        let img2 = Image::new(100, 100, img2_colors.0);
        img1.convert(new_image_type);
        assert_eq!(img1, img2);
        assert_eq!(img1.image_type, new_image_type);

        // more complex conversion
        let mut img1 = Image::new(100, 100, img1_colors.0);
        img1.set((0, 0), img1_colors.1).unwrap();
        img1.save_background();
        let mut img2 = Image::new(100, 100, img2_colors.0);
        img2.set((0, 0), img2_colors.1).unwrap();
        img2.save_background();
        img1.convert(new_image_type);
        assert_eq!(img1, img2);
        assert_eq!(img1.image_type, new_image_type);
    }

    #[test]
    fn gray8_to_gray8() {
        conversion_test(
            (Colors::GRAY8(120), Colors::GRAY8(140)),
            (Colors::GRAY8(120), Colors::GRAY8(140)),
        );
    }

    #[test]
    fn gray8_to_graya8() {
        conversion_test(
            (Colors::GRAY8(120), Colors::GRAY8(140)),
            (Colors::GRAYA8([120, 255]), Colors::GRAYA8([140, 255])),
        );
    }

    #[test]
    fn gray8_to_gray16() {
        conversion_test(
            (Colors::GRAY8(120), Colors::GRAY8(140)),
            (Colors::GRAY16(30_840), Colors::GRAY16(35_980)),
        );
    }

    #[test]
    fn gray8_to_graya16() {
        conversion_test(
            (Colors::GRAY8(120), Colors::GRAY8(140)),
            (
                Colors::GRAYA16([30_840, 65_535]),
                Colors::GRAYA16([35_980, 65_535]),
            ),
        );
    }

    #[test]
    fn gray8_to_rgb8() {
        conversion_test(
            (Colors::GRAY8(120), Colors::GRAY8(140)),
            (Colors::RGB8([120, 120, 120]), Colors::RGB8([140, 140, 140])),
        );
    }

    #[test]
    fn gray8_to_rgba8() {
        conversion_test(
            (Colors::GRAY8(120), Colors::GRAY8(140)),
            (
                Colors::RGBA8([120, 120, 120, 255]),
                Colors::RGBA8([140, 140, 140, 255]),
            ),
        );
    }

    #[test]
    fn gray8_to_rgb16() {
        conversion_test(
            (Colors::GRAY8(120), Colors::GRAY8(140)),
            (
                Colors::RGB16([30_840, 30_840, 30_840]),
                Colors::RGB16([35_980, 35_980, 35_980]),
            ),
        );
    }

    #[test]
    fn gray8_to_rgba16() {
        conversion_test(
            (Colors::GRAY8(120), Colors::GRAY8(140)),
            (
                Colors::RGBA16([30_840, 30_840, 30_840, 65_535]),
                Colors::RGBA16([35_980, 35_980, 35_980, 65_535]),
            ),
        );
    }

    #[test]
    fn graya8_to_gray8() {
        conversion_test(
            (Colors::GRAYA8([120, 255]), Colors::GRAYA8([140, 255])),
            (Colors::GRAY8(120), Colors::GRAY8(140)),
        );
    }

    #[test]
    fn graya8_to_graya8() {
        conversion_test(
            (Colors::GRAYA8([120, 255]), Colors::GRAYA8([140, 255])),
            (Colors::GRAYA8([120, 255]), Colors::GRAYA8([140, 255])),
        );
    }

    #[test]
    fn graya8_to_gray16() {
        conversion_test(
            (Colors::GRAYA8([120, 255]), Colors::GRAYA8([140, 255])),
            (Colors::GRAY16(30_840), Colors::GRAY16(35_980)),
        );
    }

    #[test]
    fn graya8_to_graya16() {
        conversion_test(
            (Colors::GRAYA8([120, 255]), Colors::GRAYA8([140, 255])),
            (
                Colors::GRAYA16([30_840, 65_535]),
                Colors::GRAYA16([35_980, 65_535]),
            ),
        );
    }

    #[test]
    fn graya8_to_rgb8() {
        conversion_test(
            (Colors::GRAYA8([120, 255]), Colors::GRAYA8([140, 255])),
            (Colors::RGB8([120, 120, 120]), Colors::RGB8([140, 140, 140])),
        );
    }

    #[test]
    fn graya8_to_rgba8() {
        conversion_test(
            (Colors::GRAYA8([120, 255]), Colors::GRAYA8([140, 255])),
            (
                Colors::RGBA8([120, 120, 120, 255]),
                Colors::RGBA8([140, 140, 140, 255]),
            ),
        );
    }

    #[test]
    fn graya8_to_rgb16() {
        conversion_test(
            (Colors::GRAYA8([120, 255]), Colors::GRAYA8([140, 255])),
            (
                Colors::RGB16([30_840, 30_840, 30_840]),
                Colors::RGB16([35_980, 35_980, 35_980]),
            ),
        );
    }

    #[test]
    fn graya8_to_rgba16() {
        conversion_test(
            (Colors::GRAYA8([120, 255]), Colors::GRAYA8([140, 255])),
            (
                Colors::RGBA16([30_840, 30_840, 30_840, 65_535]),
                Colors::RGBA16([35_980, 35_980, 35_980, 65_535]),
            ),
        );
    }

    #[test]
    fn gray16_to_gray8() {
        conversion_test(
            (Colors::GRAY16(30_840), Colors::GRAY16(35_980)),
            (Colors::GRAY8(120), Colors::GRAY8(140)),
        );
    }

    #[test]
    fn gray16_to_graya8() {
        conversion_test(
            (Colors::GRAY16(30_840), Colors::GRAY16(35_980)),
            (Colors::GRAYA8([120, 255]), Colors::GRAYA8([140, 255])),
        );
    }

    #[test]
    fn gray16_to_gray16() {
        conversion_test(
            (Colors::GRAY16(30_840), Colors::GRAY16(35_980)),
            (Colors::GRAY16(30_840), Colors::GRAY16(35_980)),
        );
    }

    #[test]
    fn gray16_to_graya16() {
        conversion_test(
            (Colors::GRAY16(30_840), Colors::GRAY16(35_980)),
            (
                Colors::GRAYA16([30_840, 65_535]),
                Colors::GRAYA16([35_980, 65_535]),
            ),
        );
    }

    #[test]
    fn gray16_to_rgb8() {
        conversion_test(
            (Colors::GRAY16(30_840), Colors::GRAY16(35_980)),
            (Colors::RGB8([120, 120, 120]), Colors::RGB8([140, 140, 140])),
        );
    }

    #[test]
    fn gray16_to_rgba8() {
        conversion_test(
            (Colors::GRAY16(30_840), Colors::GRAY16(35_980)),
            (
                Colors::RGBA8([120, 120, 120, 255]),
                Colors::RGBA8([140, 140, 140, 255]),
            ),
        );
    }

    #[test]
    fn gray16_to_rgb16() {
        conversion_test(
            (Colors::GRAY16(30_840), Colors::GRAY16(35_980)),
            (
                Colors::RGB16([30_840, 30_840, 30_840]),
                Colors::RGB16([35_980, 35_980, 35_980]),
            ),
        );
    }

    #[test]
    fn gray16_to_rgba16() {
        conversion_test(
            (Colors::GRAY16(30_840), Colors::GRAY16(35_980)),
            (
                Colors::RGBA16([30_840, 30_840, 30_840, 65_535]),
                Colors::RGBA16([35_980, 35_980, 35_980, 65_535]),
            ),
        );
    }

    #[test]
    fn graya16_to_gray8() {
        conversion_test(
            (
                Colors::GRAYA16([30_840, 65_535]),
                Colors::GRAYA16([35_980, 65_535]),
            ),
            (Colors::GRAY8(120), Colors::GRAY8(140)),
        );
    }

    #[test]
    fn graya16_to_graya8() {
        conversion_test(
            (
                Colors::GRAYA16([30_840, 65_535]),
                Colors::GRAYA16([35_980, 65_535]),
            ),
            (Colors::GRAYA8([120, 255]), Colors::GRAYA8([140, 255])),
        );
    }

    #[test]
    fn graya16_to_gray16() {
        conversion_test(
            (
                Colors::GRAYA16([30_840, 65_535]),
                Colors::GRAYA16([35_980, 65_535]),
            ),
            (Colors::GRAY16(30_840), Colors::GRAY16(35_980)),
        );
    }

    #[test]
    fn graya16_to_graya16() {
        conversion_test(
            (
                Colors::GRAYA16([30_840, 65_535]),
                Colors::GRAYA16([35_980, 65_535]),
            ),
            (
                Colors::GRAYA16([30_840, 65_535]),
                Colors::GRAYA16([35_980, 65_535]),
            ),
        );
    }

    #[test]
    fn graya16_to_rgb8() {
        conversion_test(
            (
                Colors::GRAYA16([30_840, 65_535]),
                Colors::GRAYA16([35_980, 65_535]),
            ),
            (Colors::RGB8([120, 120, 120]), Colors::RGB8([140, 140, 140])),
        );
    }

    #[test]
    fn graya16_to_rgba8() {
        conversion_test(
            (
                Colors::GRAYA16([30_840, 65_535]),
                Colors::GRAYA16([35_980, 65_535]),
            ),
            (
                Colors::RGBA8([120, 120, 120, 255]),
                Colors::RGBA8([140, 140, 140, 255]),
            ),
        );
    }

    #[test]
    fn graya16_to_rgb16() {
        conversion_test(
            (
                Colors::GRAYA16([30_840, 65_535]),
                Colors::GRAYA16([35_980, 65_535]),
            ),
            (
                Colors::RGB16([30_840, 30_840, 30_840]),
                Colors::RGB16([35_980, 35_980, 35_980]),
            ),
        );
    }

    #[test]
    fn graya16_to_rgba16() {
        conversion_test(
            (
                Colors::GRAYA16([30_840, 65_535]),
                Colors::GRAYA16([35_980, 65_535]),
            ),
            (
                Colors::RGBA16([30_840, 30_840, 30_840, 65_535]),
                Colors::RGBA16([35_980, 35_980, 35_980, 65_535]),
            ),
        );
    }

    #[test]
    fn rgb8_to_gray8() {
        conversion_test(
            (Colors::RGB8([110, 120, 130]), Colors::RGB8([140, 150, 160])),
            (Colors::GRAY8(120), Colors::GRAY8(150)),
        );
    }

    #[test]
    fn rgb8_to_graya8() {
        conversion_test(
            (Colors::RGB8([110, 120, 130]), Colors::RGB8([140, 150, 160])),
            (Colors::GRAYA8([120, 255]), Colors::GRAYA8([150, 255])),
        );
    }

    #[test]
    fn rgb8_to_gray16() {
        conversion_test(
            (Colors::RGB8([110, 120, 130]), Colors::RGB8([140, 150, 160])),
            (Colors::GRAY16(30_840), Colors::GRAY16(38_550)),
        );
    }

    #[test]
    fn rgb8_to_graya16() {
        conversion_test(
            (Colors::RGB8([110, 120, 130]), Colors::RGB8([140, 150, 160])),
            (
                Colors::GRAYA16([30_840, 65_535]),
                Colors::GRAYA16([38_550, 65_535]),
            ),
        );
    }

    #[test]
    fn rgb8_to_rgb8() {
        conversion_test(
            (Colors::RGB8([110, 120, 130]), Colors::RGB8([140, 150, 160])),
            (Colors::RGB8([110, 120, 130]), Colors::RGB8([140, 150, 160])),
        );
    }

    #[test]
    fn rgb8_to_rgba8() {
        conversion_test(
            (Colors::RGB8([110, 120, 130]), Colors::RGB8([140, 150, 160])),
            (
                Colors::RGBA8([110, 120, 130, 255]),
                Colors::RGBA8([140, 150, 160, 255]),
            ),
        );
    }

    #[test]
    fn rgb8_to_rgb16() {
        conversion_test(
            (Colors::RGB8([110, 120, 130]), Colors::RGB8([140, 150, 160])),
            (
                Colors::RGB16([28_270, 30_840, 33_410]),
                Colors::RGB16([35_980, 38_550, 41_120]),
            ),
        );
    }

    #[test]
    fn rgb8_to_rgba16() {
        conversion_test(
            (Colors::RGB8([110, 120, 130]), Colors::RGB8([140, 150, 160])),
            (
                Colors::RGBA16([28_270, 30_840, 33_410, 65_535]),
                Colors::RGBA16([35_980, 38_550, 41_120, 65_535]),
            ),
        );
    }

    #[test]
    fn rgba8_to_gray8() {
        conversion_test(
            (
                Colors::RGBA8([110, 120, 130, 140]),
                Colors::RGBA8([140, 150, 160, 170]),
            ),
            (Colors::GRAY8(120), Colors::GRAY8(150)),
        );
    }

    #[test]
    fn rgba8_to_graya8() {
        conversion_test(
            (
                Colors::RGBA8([110, 120, 130, 140]),
                Colors::RGBA8([140, 150, 160, 170]),
            ),
            (Colors::GRAYA8([120, 140]), Colors::GRAYA8([150, 170])),
        );
    }

    #[test]
    fn rgba8_to_gray16() {
        conversion_test(
            (
                Colors::RGBA8([110, 120, 130, 140]),
                Colors::RGBA8([140, 150, 160, 170]),
            ),
            (Colors::GRAY16(30_840), Colors::GRAY16(38_550)),
        );
    }

    #[test]
    fn rgba8_to_graya16() {
        conversion_test(
            (
                Colors::RGBA8([110, 120, 130, 140]),
                Colors::RGBA8([140, 150, 160, 170]),
            ),
            (
                Colors::GRAYA16([30_840, 35_980]),
                Colors::GRAYA16([38_550, 43_690]),
            ),
        );
    }

    #[test]
    fn rgba8_to_rgb8() {
        conversion_test(
            (
                Colors::RGBA8([110, 120, 130, 140]),
                Colors::RGBA8([140, 150, 160, 170]),
            ),
            (Colors::RGB8([110, 120, 130]), Colors::RGB8([140, 150, 160])),
        );
    }

    #[test]
    fn rgba8_to_rgba8() {
        conversion_test(
            (
                Colors::RGBA8([110, 120, 130, 140]),
                Colors::RGBA8([140, 150, 160, 170]),
            ),
            (
                Colors::RGBA8([110, 120, 130, 140]),
                Colors::RGBA8([140, 150, 160, 170]),
            ),
        );
    }

    #[test]
    fn rgba8_to_rgb16() {
        conversion_test(
            (
                Colors::RGBA8([110, 120, 130, 140]),
                Colors::RGBA8([140, 150, 160, 170]),
            ),
            (
                Colors::RGB16([28_270, 30_840, 33_410]),
                Colors::RGB16([35_980, 38_550, 41_120]),
            ),
        );
    }

    #[test]
    fn rgba8_to_rgba16() {
        conversion_test(
            (
                Colors::RGBA8([110, 120, 130, 140]),
                Colors::RGBA8([140, 150, 160, 170]),
            ),
            (
                Colors::RGBA16([28_270, 30_840, 33_410, 35_980]),
                Colors::RGBA16([35_980, 38_550, 41_120, 43_690]),
            ),
        );
    }

    #[test]
    fn rgb16_to_gray8() {
        conversion_test(
            (
                Colors::RGB16([28_270, 30_840, 33_410]),
                Colors::RGB16([35_980, 38_550, 41_120]),
            ),
            (Colors::GRAY8(120), Colors::GRAY8(150)),
        );
    }

    #[test]
    fn rgb16_to_graya8() {
        conversion_test(
            (
                Colors::RGB16([28_270, 30_840, 33_410]),
                Colors::RGB16([35_980, 38_550, 41_120]),
            ),
            (Colors::GRAYA8([120, 255]), Colors::GRAYA8([150, 255])),
        );
    }

    #[test]
    fn rgb16_to_gray16() {
        conversion_test(
            (
                Colors::RGB16([28_270, 30_840, 33_410]),
                Colors::RGB16([35_980, 38_550, 41_120]),
            ),
            (Colors::GRAY16(30_840), Colors::GRAY16(38_550)),
        );
    }

    #[test]
    fn rgb16_to_graya16() {
        conversion_test(
            (
                Colors::RGB16([28_270, 30_840, 33_410]),
                Colors::RGB16([35_980, 38_550, 41_120]),
            ),
            (
                Colors::GRAYA16([30_840, 65_535]),
                Colors::GRAYA16([38_550, 65_535]),
            ),
        );
    }

    #[test]
    fn rgb16_to_rgb8() {
        conversion_test(
            (
                Colors::RGB16([28_270, 30_840, 33_410]),
                Colors::RGB16([35_980, 38_550, 41_120]),
            ),
            (Colors::RGB8([110, 120, 130]), Colors::RGB8([140, 150, 160])),
        );
    }

    #[test]
    fn rgb16_to_rgba8() {
        conversion_test(
            (
                Colors::RGB16([28_270, 30_840, 33_410]),
                Colors::RGB16([35_980, 38_550, 41_120]),
            ),
            (
                Colors::RGBA8([110, 120, 130, 255]),
                Colors::RGBA8([140, 150, 160, 255]),
            ),
        );
    }

    #[test]
    fn rgb16_to_rgb16() {
        conversion_test(
            (
                Colors::RGB16([28_270, 30_840, 33_410]),
                Colors::RGB16([35_980, 38_550, 41_120]),
            ),
            (
                Colors::RGB16([28_270, 30_840, 33_410]),
                Colors::RGB16([35_980, 38_550, 41_120]),
            ),
        );
    }

    #[test]
    fn rgb16_to_rgba16() {
        conversion_test(
            (
                Colors::RGB16([28_270, 30_840, 33_410]),
                Colors::RGB16([35_980, 38_550, 41_120]),
            ),
            (
                Colors::RGBA16([28_270, 30_840, 33_410, 65_535]),
                Colors::RGBA16([35_980, 38_550, 41_120, 65_535]),
            ),
        );
    }

    #[test]
    fn rgba16_to_gray8() {
        conversion_test(
            (
                Colors::RGBA16([28_270, 30_840, 33_410, 65_535]),
                Colors::RGBA16([35_980, 38_550, 41_120, 65_535]),
            ),
            (Colors::GRAY8(120), Colors::GRAY8(150)),
        );
    }

    #[test]
    fn rgba16_to_graya8() {
        conversion_test(
            (
                Colors::RGBA16([28_270, 30_840, 33_410, 65_535]),
                Colors::RGBA16([35_980, 38_550, 41_120, 65_535]),
            ),
            (Colors::GRAYA8([120, 255]), Colors::GRAYA8([150, 255])),
        );
    }

    #[test]
    fn rgba16_to_gray16() {
        conversion_test(
            (
                Colors::RGBA16([28_270, 30_840, 33_410, 65_535]),
                Colors::RGBA16([35_980, 38_550, 41_120, 65_535]),
            ),
            (Colors::GRAY16(30_840), Colors::GRAY16(38_550)),
        );
    }

    #[test]
    fn rgba16_to_graya16() {
        conversion_test(
            (
                Colors::RGBA16([28_270, 30_840, 33_410, 65_535]),
                Colors::RGBA16([35_980, 38_550, 41_120, 65_535]),
            ),
            (
                Colors::GRAYA16([30_840, 65_535]),
                Colors::GRAYA16([38_550, 65_535]),
            ),
        );
    }

    #[test]
    fn rgba16_to_rgb8() {
        conversion_test(
            (
                Colors::RGBA16([28_270, 30_840, 33_410, 65_535]),
                Colors::RGBA16([35_980, 38_550, 41_120, 65_535]),
            ),
            (Colors::RGB8([110, 120, 130]), Colors::RGB8([140, 150, 160])),
        );
    }

    #[test]
    fn rgba16_to_rgba8() {
        conversion_test(
            (
                Colors::RGBA16([28_270, 30_840, 33_410, 65_535]),
                Colors::RGBA16([35_980, 38_550, 41_120, 65_535]),
            ),
            (
                Colors::RGBA8([110, 120, 130, 255]),
                Colors::RGBA8([140, 150, 160, 255]),
            ),
        );
    }

    #[test]
    fn rgba16_to_rgb16() {
        conversion_test(
            (
                Colors::RGBA16([28_270, 30_840, 33_410, 65_535]),
                Colors::RGBA16([35_980, 38_550, 41_120, 65_535]),
            ),
            (
                Colors::RGB16([28_270, 30_840, 33_410]),
                Colors::RGB16([35_980, 38_550, 41_120]),
            ),
        );
    }

    #[test]
    fn rgba16_to_rgba16() {
        conversion_test(
            (
                Colors::RGBA16([28_270, 30_840, 33_410, 65_535]),
                Colors::RGBA16([35_980, 38_550, 41_120, 65_535]),
            ),
            (
                Colors::RGBA16([28_270, 30_840, 33_410, 65_535]),
                Colors::RGBA16([35_980, 38_550, 41_120, 65_535]),
            ),
        );
    }
}

use crate::colors::{Color, ColorType};
use crate::image::{BackgroundData, Image};

/// A trait for converting between different color/image types.
pub trait Conversions {
    /// Calculates the average of a slice of u16 values.
    /// # Arguments
    /// * ```values``` - The slice of values.
    /// # Returns
    /// * The average of the values.
    /// # Example
    /// ```
    /// use tinydraw::{Image, Conversions};
    ///
    /// let u8_values: [u8; 3] = [0, 128, 255];
    /// let average: u8 = <Image as Conversions>::average(&u8_values);
    /// assert_eq!(average, 128_u8);
    ///
    /// let u16_values: [u16; 3] = [0, 32768, 65535];
    /// let average: u16 = <Image as Conversions>::average(&u16_values);
    /// assert_eq!(average, 32768_u16);
    /// ```
    fn average<T>(values: &[T]) -> T
    where
        T: Copy + TryFrom<u64> + Into<f64>,
    {
        // this is safe since the average of a set of numbers is always a number in the set
        unsafe { T::try_from((values.iter().map(|val| (*val).into()).sum::<f64>() / (values.len() as f64)).round() as u64).unwrap_unchecked() }
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
    /// use tinydraw::{Image, ColorType, Conversions};
    ///
    /// let image_type1 = ColorType::GRAY8;
    /// let image_type2 = ColorType::RGB8;
    /// assert!(<Image as Conversions>::cvt_is_lossless(image_type1, image_type2));
    ///
    /// let image_type1 = ColorType::RGB8;
    /// let image_type2 = ColorType::GRAY8;
    /// assert!(!<Image as Conversions>::cvt_is_lossless(image_type1, image_type2));
    /// ```
    fn cvt_is_lossless(image_type1: ColorType, image_type2: ColorType) -> bool {
        match image_type1 {
            ColorType::GRAY8 => true,
            ColorType::GRAYA8 => match image_type2 {
                ColorType::GRAY8 => false,
                ColorType::GRAYA8 => true,
                ColorType::GRAY16 => false,
                ColorType::GRAYA16 => true,
                ColorType::RGB8 => false,
                ColorType::RGBA8 => true,
                ColorType::RGB16 => false,
                ColorType::RGBA16 => true,
            },
            ColorType::GRAY16 => match image_type2 {
                ColorType::GRAY8 => false,
                ColorType::GRAYA8 => false,
                ColorType::GRAY16 => true,
                ColorType::GRAYA16 => true,
                ColorType::RGB8 => false,
                ColorType::RGBA8 => false,
                ColorType::RGB16 => true,
                ColorType::RGBA16 => true,
            },
            ColorType::GRAYA16 => match image_type2 {
                ColorType::GRAY8 => false,
                ColorType::GRAYA8 => false,
                ColorType::GRAY16 => false,
                ColorType::GRAYA16 => true,
                ColorType::RGB8 => false,
                ColorType::RGBA8 => false,
                ColorType::RGB16 => false,
                ColorType::RGBA16 => true,
            },
            ColorType::RGB8 => match image_type2 {
                ColorType::GRAY8 => false,
                ColorType::GRAYA8 => false,
                ColorType::GRAY16 => false,
                ColorType::GRAYA16 => false,
                ColorType::RGB8 => true,
                ColorType::RGBA8 => true,
                ColorType::RGB16 => true,
                ColorType::RGBA16 => true,
            },
            ColorType::RGBA8 => match image_type2 {
                ColorType::GRAY8 => false,
                ColorType::GRAYA8 => false,
                ColorType::GRAY16 => false,
                ColorType::GRAYA16 => false,
                ColorType::RGB8 => false,
                ColorType::RGBA8 => true,
                ColorType::RGB16 => false,
                ColorType::RGBA16 => true,
            },
            ColorType::RGB16 => match image_type2 {
                ColorType::GRAY8 => false,
                ColorType::GRAYA8 => false,
                ColorType::GRAY16 => false,
                ColorType::GRAYA16 => false,
                ColorType::RGB8 => false,
                ColorType::RGBA8 => false,
                ColorType::RGB16 => true,
                ColorType::RGBA16 => true,
            },
            ColorType::RGBA16 => {
                matches!(image_type2, ColorType::RGBA16)
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
    fn convert_bytes(data: &mut Vec<u8>, img_type: ColorType, img_type_new: ColorType) {
        match img_type {
            ColorType::GRAY8 => {
                match img_type_new {
                    ColorType::GRAY8 => {} // do nothing (same type)
                    ColorType::GRAYA8 => {
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
                    ColorType::GRAY16 => {
                        let original_len = data.len();

                        data.reserve_exact(original_len);
                        for _ in 0..original_len {
                            data.push(0);
                        }

                        for i in (0..original_len).rev() {
                            let new_val = Self::val_u8_to_u16(data[i]).to_ne_bytes();
                            let new_loc = i << 1;
                            data[new_loc] = new_val[0];
                            data[new_loc + 1] = new_val[1];
                        }
                    }
                    ColorType::GRAYA16 => {
                        let original_len = data.len();

                        let reserve_amount = original_len * 3;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..reserve_amount {
                            data.push(255);
                        }

                        for i in (0..original_len).rev() {
                            let new_val = Self::val_u8_to_u16(data[i]).to_ne_bytes();
                            let new_loc = i << 2;
                            data[new_loc] = new_val[0];
                            data[new_loc + 1] = new_val[1];
                        }

                        for i in (2..original_len).step_by(4) {
                            data[i] = 255;
                            data[i + 1] = 255;
                        }
                    }
                    ColorType::RGB8 => {
                        let original_len = data.len();

                        let reserve_amount = original_len * 2;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..reserve_amount {
                            data.push(0);
                        }

                        for i in (0..original_len).rev() {
                            let new_loc = i * 3;
                            data[new_loc] = data[i];
                            data[new_loc + 1] = data[i];
                            data[new_loc + 2] = data[i];
                        }
                    }
                    ColorType::RGBA8 => {
                        let original_len = data.len();

                        let reserve_amount = original_len * 3;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..reserve_amount {
                            data.push(255);
                        }

                        for i in (0..original_len).rev() {
                            let new_loc = i << 2;
                            data[new_loc] = data[i];
                            data[new_loc + 1] = data[i];
                            data[new_loc + 2] = data[i];
                        }

                        for i in (3..original_len).step_by(4) {
                            data[i] = 255;
                        }
                    }
                    ColorType::RGB16 => {
                        let original_len = data.len();

                        let reserve_amount = original_len * 5;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..reserve_amount {
                            data.push(0);
                        }

                        for i in (0..original_len).rev() {
                            let new_val = Self::val_u8_to_u16(data[i]).to_ne_bytes();
                            let new_loc = i * 6;
                            data[new_loc] = new_val[0];
                            data[new_loc + 1] = new_val[1];
                            data[new_loc + 2] = new_val[0];
                            data[new_loc + 3] = new_val[1];
                            data[new_loc + 4] = new_val[0];
                            data[new_loc + 5] = new_val[1];
                        }
                    }
                    ColorType::RGBA16 => {
                        let original_len = data.len();

                        let reserve_amount = original_len * 7;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..reserve_amount {
                            data.push(255);
                        }

                        for i in (0..original_len).rev() {
                            let new_val = Self::val_u8_to_u16(data[i]).to_ne_bytes();
                            let new_loc = i << 3;
                            data[new_loc] = new_val[0];
                            data[new_loc + 1] = new_val[1];
                            data[new_loc + 2] = new_val[0];
                            data[new_loc + 3] = new_val[1];
                            data[new_loc + 4] = new_val[0];
                            data[new_loc + 5] = new_val[1];
                        }

                        for i in (6..original_len).step_by(8) {
                            data[i] = 255;
                            data[i + 1] = 255;
                        }
                    }
                }
            }
            ColorType::GRAYA8 => {
                match img_type_new {
                    ColorType::GRAY8 => {
                        // bit shift to the right by 1 == divide by 2

                        for i in (0..data.len()).step_by(2) {
                            data[i >> 1] = data[i];
                        }

                        data.truncate(data.len() >> 1);
                        data.shrink_to_fit();
                    }
                    ColorType::GRAYA8 => {} // do nothing (same type)
                    ColorType::GRAY16 => {
                        for i in (0..data.len()).step_by(2) {
                            let new_val = Self::val_u8_to_u16(data[i]).to_ne_bytes();
                            data[i] = new_val[0];
                            data[i + 1] = new_val[1];
                        }
                    }
                    ColorType::GRAYA16 => {
                        let original_len = data.len();

                        data.reserve_exact(original_len);
                        for _ in 0..original_len {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(2).rev() {
                            let new_val = Self::val_u8_to_u16(data[i]).to_ne_bytes();
                            let new_transparency = Self::val_u8_to_u16(data[i + 1]).to_ne_bytes();
                            let new_loc = i << 1;
                            data[new_loc] = new_val[0];
                            data[new_loc + 1] = new_val[1];
                            data[new_loc + 2] = new_transparency[0];
                            data[new_loc + 3] = new_transparency[1];
                        }
                    }
                    ColorType::RGB8 => {
                        let original_len = data.len();

                        let reserve_amount = original_len >> 1;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..reserve_amount {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(2).rev() {
                            let new_loc = i + (i >> 1);
                            data[new_loc] = data[i];
                            data[new_loc + 1] = data[i];
                            data[new_loc + 2] = data[i];
                        }
                    }
                    ColorType::RGBA8 => {
                        let original_len = data.len();

                        data.reserve_exact(original_len);
                        for _ in 0..original_len {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(2).rev() {
                            let new_loc = i << 1;
                            data[new_loc + 3] = data[i + 1];

                            data[new_loc] = data[i];
                            data[new_loc + 1] = data[i];
                            data[new_loc + 2] = data[i];
                        }
                    }
                    ColorType::RGB16 => {
                        let original_len = data.len();

                        let reserve_amount = original_len << 1;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..reserve_amount {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(2).rev() {
                            let new_val = Self::val_u8_to_u16(data[i]).to_ne_bytes();
                            let new_loc = i * 3;
                            data[new_loc] = new_val[0];
                            data[new_loc + 1] = new_val[1];
                            data[new_loc + 2] = new_val[0];
                            data[new_loc + 3] = new_val[1];
                            data[new_loc + 4] = new_val[0];
                            data[new_loc + 5] = new_val[1];
                        }
                    }
                    ColorType::RGBA16 => {
                        let original_len = data.len();

                        let reserve_amount = original_len * 3;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..reserve_amount {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(2).rev() {
                            let new_val = Self::val_u8_to_u16(data[i]).to_ne_bytes();
                            let new_transparency = Self::val_u8_to_u16(data[i + 1]).to_ne_bytes();
                            let new_loc = i << 2;
                            data[new_loc] = new_val[0];
                            data[new_loc + 1] = new_val[1];
                            data[new_loc + 2] = new_val[0];
                            data[new_loc + 3] = new_val[1];
                            data[new_loc + 4] = new_val[0];
                            data[new_loc + 5] = new_val[1];
                            data[new_loc + 6] = new_transparency[0];
                            data[new_loc + 7] = new_transparency[1];
                        }
                    }
                }
            }
            ColorType::GRAY16 => {
                match img_type_new {
                    ColorType::GRAY8 => {
                        for i in (0..data.len()).step_by(2) {
                            data[i >> 1] = Self::val_u16_to_u8(u16::from_ne_bytes([data[i], data[i + 1]]));
                        }

                        data.truncate(data.len() >> 1);
                        data.shrink_to_fit();
                    }
                    ColorType::GRAYA8 => {
                        for i in (0..data.len()).step_by(2) {
                            data[i] = Self::val_u16_to_u8(u16::from_ne_bytes([data[i], data[i + 1]]));
                            data[i + 1] = 255;
                        }
                    }
                    ColorType::GRAY16 => {} // do nothing (same type)
                    ColorType::GRAYA16 => {
                        let original_len = data.len();

                        data.reserve_exact(original_len);
                        for _ in 0..original_len {
                            data.push(255);
                        }

                        for i in (0..original_len).step_by(2).rev() {
                            let new_loc = i << 1;
                            data[new_loc] = data[i];
                            data[new_loc + 1] = data[i + 1];
                        }

                        for i in (2..original_len).step_by(4) {
                            data[i] = 255;
                            data[i + 1] = 255;
                        }
                    }
                    ColorType::RGB8 => {
                        let original_len = data.len();

                        let reserve_amount = original_len >> 1;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..reserve_amount {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(2).rev() {
                            let new_val = Self::val_u16_to_u8(u16::from_ne_bytes([data[i], data[i + 1]]));
                            let new_loc = i + (i >> 1);
                            data[new_loc] = new_val;
                            data[new_loc + 1] = new_val;
                            data[new_loc + 2] = new_val;
                        }
                    }
                    ColorType::RGBA8 => {
                        let original_len = data.len();

                        data.reserve_exact(original_len);
                        for _ in 0..original_len {
                            data.push(255);
                        }

                        for i in (0..original_len).step_by(2).rev() {
                            let new_val = Self::val_u16_to_u8(u16::from_ne_bytes([data[i], data[i + 1]]));
                            let new_loc = i << 1;
                            data[new_loc] = new_val;
                            data[new_loc + 1] = new_val;
                            data[new_loc + 2] = new_val;
                        }

                        for i in (3..original_len).step_by(4) {
                            data[i] = 255;
                        }
                    }
                    ColorType::RGB16 => {
                        let original_len = data.len();

                        let reserve_amount = original_len << 1;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..reserve_amount {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(2).rev() {
                            let new_loc = i * 3;
                            data[new_loc] = data[i];
                            data[new_loc + 1] = data[i + 1];
                            data[new_loc + 2] = data[i];
                            data[new_loc + 3] = data[i + 1];
                            data[new_loc + 4] = data[i];
                            data[new_loc + 5] = data[i + 1];
                        }
                    }
                    ColorType::RGBA16 => {
                        let original_len = data.len();

                        let reserve_amount = original_len * 3;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..reserve_amount {
                            data.push(255);
                        }

                        for i in (0..original_len).step_by(2).rev() {
                            let new_loc = i << 2;
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
            ColorType::GRAYA16 => {
                match img_type_new {
                    ColorType::GRAY8 => {
                        // bit shift to the right by 2 == divide by 4
                        for i in (0..data.len()).step_by(4) {
                            data[i >> 2] = Self::val_u16_to_u8(u16::from_ne_bytes([data[i], data[i + 1]]));
                        }

                        data.truncate(data.len() >> 2);
                        data.shrink_to_fit();
                    }
                    ColorType::GRAYA8 => {
                        for i in (0..data.len()).step_by(2) {
                            data[i >> 1] = Self::val_u16_to_u8(u16::from_ne_bytes([data[i], data[i + 1]]));
                        }

                        data.truncate(data.len() >> 1);
                        data.shrink_to_fit();
                    }
                    ColorType::GRAY16 => {
                        for i in (0..data.len()).step_by(4) {
                            let new_loc = i >> 1;
                            data[new_loc] = data[i];
                            data[new_loc + 1] = data[i + 1];
                        }

                        data.truncate(data.len() >> 1);
                        data.shrink_to_fit();
                    }
                    ColorType::GRAYA16 => {} // do nothing (same type)
                    ColorType::RGB8 => {
                        for i in (0..data.len()).step_by(4) {
                            let new_val = Self::val_u16_to_u8(u16::from_ne_bytes([data[i], data[i + 1]]));
                            let new_loc = (i >> 1) + (i >> 2);
                            data[new_loc] = new_val;
                            data[new_loc + 1] = new_val;
                            data[new_loc + 2] = new_val;
                        }

                        data.truncate((data.len() >> 1) + (data.len() >> 2));
                        data.shrink_to_fit();
                    }
                    ColorType::RGBA8 => {
                        for i in (0..data.len()).step_by(4) {
                            let new_val = Self::val_u16_to_u8(u16::from_ne_bytes([data[i], data[i + 1]]));
                            let new_transparency = Self::val_u16_to_u8(u16::from_ne_bytes([data[i + 2], data[i + 3]]));
                            data[i] = new_val;
                            data[i + 1] = new_val;
                            data[i + 2] = new_val;
                            data[i + 3] = new_transparency;
                        }
                    }
                    ColorType::RGB16 => {
                        let original_len = data.len();

                        let reserve_amount = original_len >> 1;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..reserve_amount {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(4).rev() {
                            let new_loc = i + (i >> 1);
                            data[new_loc] = data[i];
                            data[new_loc + 1] = data[i + 1];
                            data[new_loc + 2] = data[i];
                            data[new_loc + 3] = data[i + 1];
                            data[new_loc + 4] = data[i];
                            data[new_loc + 5] = data[i + 1];
                        }
                    }
                    ColorType::RGBA16 => {
                        let original_len = data.len();

                        data.reserve_exact(original_len);
                        for _ in 0..original_len {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(4).rev() {
                            let new_loc = i << 1;
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
            ColorType::RGB8 => {
                match img_type_new {
                    ColorType::GRAY8 => {
                        for i in (0..data.len()).step_by(3) {
                            data[i / 3] = Self::average(&data[i..(i + 3)]);
                        }

                        data.truncate(data.len() / 3);
                        data.shrink_to_fit();
                    }
                    ColorType::GRAYA8 => {
                        for i in (0..data.len()).step_by(3) {
                            let new_loc = (i / 3) << 1; // multiply by 2/3
                            data[new_loc] = Self::average(&data[i..(i + 3)]);
                            data[new_loc + 1] = 255;
                        }

                        data.truncate((data.len() / 3) << 1);
                        data.shrink_to_fit();
                    }
                    ColorType::GRAY16 => {
                        for i in (0..data.len()).step_by(3) {
                            let new_loc = (i / 3) << 1; // multiply by 2/3
                            let new_val = Self::val_u8_to_u16(Self::average(&data[i..(i + 3)])).to_ne_bytes();
                            data[new_loc] = new_val[0];
                            data[new_loc + 1] = new_val[1];
                        }

                        data.truncate((data.len() / 3) << 1);
                        data.shrink_to_fit();
                    }
                    ColorType::GRAYA16 => {
                        let original_len = data.len();

                        let reserve_amount = original_len / 3;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..reserve_amount {
                            data.push(255);
                        }

                        for i in (0..original_len).step_by(3).rev() {
                            let new_loc = (i / 3) << 2; // multiply by 4/3
                            let new_val = Self::val_u8_to_u16(Self::average(&data[i..(i + 3)])).to_ne_bytes();
                            data[new_loc] = new_val[0];
                            data[new_loc + 1] = new_val[1];
                        }

                        for i in (2..original_len).step_by(4) {
                            data[i] = 255;
                            data[i + 1] = 255;
                        }
                    }
                    ColorType::RGB8 => {} // do nothing (same type)
                    ColorType::RGBA8 => {
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
                    ColorType::RGB16 => {
                        let original_len = data.len();

                        data.reserve_exact(original_len);
                        for _ in 0..original_len {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(3).rev() {
                            let new_loc = i << 1; // multiply by 2

                            let new_val = Self::val_u8_to_u16(data[i + 2]).to_ne_bytes();
                            data[new_loc + 4] = new_val[0];
                            data[new_loc + 5] = new_val[1];

                            let new_val = Self::val_u8_to_u16(data[i + 1]).to_ne_bytes();
                            data[new_loc + 2] = new_val[0];
                            data[new_loc + 3] = new_val[1];

                            let new_val = Self::val_u8_to_u16(data[i]).to_ne_bytes();
                            data[new_loc] = new_val[0];
                            data[new_loc + 1] = new_val[1];
                        }
                    }
                    ColorType::RGBA16 => {
                        let original_len = data.len();

                        let reserve_amount = 5 * (original_len / 3);
                        data.reserve_exact(reserve_amount);
                        for _ in 0..reserve_amount {
                            data.push(255);
                        }

                        for i in (0..original_len).step_by(3).rev() {
                            let new_loc = (i / 3) * 8; // multiply by 8/3

                            let new_val = Self::val_u8_to_u16(data[i + 2]).to_ne_bytes();
                            data[new_loc + 4] = new_val[0];
                            data[new_loc + 5] = new_val[1];

                            let new_val = Self::val_u8_to_u16(data[i + 1]).to_ne_bytes();
                            data[new_loc + 2] = new_val[0];
                            data[new_loc + 3] = new_val[1];

                            let new_val = Self::val_u8_to_u16(data[i]).to_ne_bytes();
                            data[new_loc] = new_val[0];
                            data[new_loc + 1] = new_val[1];
                        }

                        for i in (6..original_len).step_by(8) {
                            data[i] = 255;
                            data[i + 1] = 255;
                        }
                    }
                }
            }
            ColorType::RGBA8 => {
                match img_type_new {
                    ColorType::GRAY8 => {
                        for i in (0..data.len()).step_by(4) {
                            data[i >> 2] = Self::average(&data[i..(i + 3)]);
                        }

                        data.truncate(data.len() >> 2);
                        data.shrink_to_fit();
                    }
                    ColorType::GRAYA8 => {
                        for i in (0..data.len()).step_by(4) {
                            let new_loc = i >> 1; // divide by 2
                            data[new_loc] = Self::average(&data[i..(i + 3)]);
                            data[new_loc + 1] = data[i + 3];
                        }

                        data.truncate(data.len() >> 1);
                        data.shrink_to_fit();
                    }
                    ColorType::GRAY16 => {
                        for i in (0..data.len()).step_by(4) {
                            let new_loc = i >> 1; // divide by 2
                            let new_val = Self::val_u8_to_u16(Self::average(&data[i..(i + 3)])).to_ne_bytes();
                            data[new_loc] = new_val[0];
                            data[new_loc + 1] = new_val[1];
                        }

                        data.truncate(data.len() >> 1);
                        data.shrink_to_fit();
                    }
                    ColorType::GRAYA16 => {
                        for i in (0..data.len()).step_by(4) {
                            let new_val = Self::val_u8_to_u16(Self::average(&data[i..(i + 3)])).to_ne_bytes();
                            let new_transparency = Self::val_u8_to_u16(data[i + 3]).to_ne_bytes();
                            data[i] = new_val[0];
                            data[i + 1] = new_val[1];
                            data[i + 2] = new_transparency[0];
                            data[i + 3] = new_transparency[1];
                        }
                    }
                    ColorType::RGB8 => {
                        for i in (0..data.len()).step_by(4) {
                            let new_loc = (i >> 1) + (i >> 2); // multiply by 3/4
                            data[new_loc] = data[i];
                            data[new_loc + 1] = data[i + 1];
                            data[new_loc + 2] = data[i + 2];
                        }

                        data.truncate((data.len() >> 1) + (data.len() >> 2));
                        data.shrink_to_fit();
                    }
                    ColorType::RGBA8 => {} // do nothing (same type)
                    ColorType::RGB16 => {
                        let original_len = data.len();

                        let reserve_amount = original_len >> 1;
                        data.reserve_exact(reserve_amount);
                        for _ in 0..reserve_amount {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(4).rev() {
                            let new_loc = i + (i >> 1); // multiply by 3/2

                            let new_val = Self::val_u8_to_u16(data[i + 2]).to_ne_bytes();
                            data[new_loc + 4] = new_val[0];
                            data[new_loc + 5] = new_val[1];

                            let new_val = Self::val_u8_to_u16(data[i + 1]).to_ne_bytes();
                            data[new_loc + 2] = new_val[0];
                            data[new_loc + 3] = new_val[1];

                            let new_val = Self::val_u8_to_u16(data[i]).to_ne_bytes();
                            data[new_loc] = new_val[0];
                            data[new_loc + 1] = new_val[1];
                        }
                    }
                    ColorType::RGBA16 => {
                        let original_len = data.len();

                        data.reserve_exact(original_len);
                        for _ in 0..original_len {
                            data.push(0);
                        }

                        for i in (0..original_len).step_by(4).rev() {
                            let new_loc = i << 1; // multiply by 2

                            let new_val = Self::val_u8_to_u16(data[i + 3]).to_ne_bytes();
                            data[new_loc + 6] = new_val[0];
                            data[new_loc + 7] = new_val[1];

                            let new_val = Self::val_u8_to_u16(data[i + 2]).to_ne_bytes();
                            data[new_loc + 4] = new_val[0];
                            data[new_loc + 5] = new_val[1];

                            let new_val = Self::val_u8_to_u16(data[i + 1]).to_ne_bytes();
                            data[new_loc + 2] = new_val[0];
                            data[new_loc + 3] = new_val[1];

                            let new_val = Self::val_u8_to_u16(data[i]).to_ne_bytes();
                            data[new_loc] = new_val[0];
                            data[new_loc + 1] = new_val[1];
                        }
                    }
                }
            }
            ColorType::RGB16 => {
                match img_type_new {
                    ColorType::GRAY8 => {
                        for i in (0..data.len()).step_by(6) {
                            data[i / 6] = Self::val_u16_to_u8(Self::average(&[
                                u16::from_ne_bytes([data[i], data[i + 1]]),
                                u16::from_ne_bytes([data[i + 2], data[i + 3]]),
                                u16::from_ne_bytes([data[i + 4], data[i + 5]]),
                            ]));
                        }

                        data.truncate(data.len() / 6);
                        data.shrink_to_fit();
                    }
                    ColorType::GRAYA8 => {
                        for i in (0..data.len()).step_by(6) {
                            let new_loc = i / 3;
                            data[new_loc] = Self::val_u16_to_u8(Self::average(&[
                                u16::from_ne_bytes([data[i], data[i + 1]]),
                                u16::from_ne_bytes([data[i + 2], data[i + 3]]),
                                u16::from_ne_bytes([data[i + 4], data[i + 5]]),
                            ]));
                            data[new_loc + 1] = 255;
                        }

                        data.truncate(data.len() / 3);
                        data.shrink_to_fit();
                    }
                    ColorType::GRAY16 => {
                        for i in (0..data.len()).step_by(6) {
                            let new_val = Self::average(&[
                                u16::from_ne_bytes([data[i], data[i + 1]]),
                                u16::from_ne_bytes([data[i + 2], data[i + 3]]),
                                u16::from_ne_bytes([data[i + 4], data[i + 5]]),
                            ])
                            .to_ne_bytes();
                            let new_loc = i / 3;
                            data[new_loc] = new_val[0];
                            data[new_loc + 1] = new_val[1];
                        }

                        data.truncate(data.len() / 3);
                        data.shrink_to_fit();
                    }
                    ColorType::GRAYA16 => {
                        for i in (0..data.len()).step_by(6) {
                            let new_val = Self::average(&[
                                u16::from_ne_bytes([data[i], data[i + 1]]),
                                u16::from_ne_bytes([data[i + 2], data[i + 3]]),
                                u16::from_ne_bytes([data[i + 4], data[i + 5]]),
                            ])
                            .to_ne_bytes();
                            let new_loc = (i / 3) << 1;
                            data[new_loc] = new_val[0];
                            data[new_loc + 1] = new_val[1];
                            data[new_loc + 2] = 255;
                            data[new_loc + 3] = 255;
                        }

                        data.truncate((data.len() / 3) << 1);
                        data.shrink_to_fit();
                    }
                    ColorType::RGB8 => {
                        for i in (0..data.len()).step_by(2) {
                            data[i >> 1] = Self::val_u16_to_u8(u16::from_ne_bytes([data[i], data[i + 1]]));
                        }

                        data.truncate(data.len() >> 1);
                        data.shrink_to_fit();
                    }
                    ColorType::RGBA8 => {
                        for i in (0..data.len()).step_by(6) {
                            let new_loc = (i / 3) << 1; // multiply by 2/3
                            data[new_loc] = Self::val_u16_to_u8(u16::from_ne_bytes([data[i], data[i + 1]]));
                            data[new_loc + 1] = Self::val_u16_to_u8(u16::from_ne_bytes([data[i + 2], data[i + 3]]));
                            data[new_loc + 2] = Self::val_u16_to_u8(u16::from_ne_bytes([data[i + 4], data[i + 5]]));
                            data[new_loc + 3] = 255;
                        }

                        data.truncate((data.len() / 3) << 1);
                        data.shrink_to_fit();
                    }
                    ColorType::RGB16 => {} // do nothing (same type)
                    ColorType::RGBA16 => {
                        let original_len = data.len();

                        let reserve_len = original_len / 3;
                        data.reserve_exact(reserve_len);
                        for _ in 0..reserve_len {
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
            ColorType::RGBA16 => {
                match img_type_new {
                    ColorType::GRAY8 => {
                        for i in (0..data.len()).step_by(8) {
                            data[i >> 3] = Self::val_u16_to_u8(Self::average(&[
                                u16::from_ne_bytes([data[i], data[i + 1]]),
                                u16::from_ne_bytes([data[i + 2], data[i + 3]]),
                                u16::from_ne_bytes([data[i + 4], data[i + 5]]),
                            ]));
                        }

                        data.truncate(data.len() >> 3);
                        data.shrink_to_fit();
                    }
                    ColorType::GRAYA8 => {
                        for i in (0..data.len()).step_by(8) {
                            let new_loc = i >> 2; // divide by 4
                            data[new_loc] = Self::val_u16_to_u8(Self::average(&[
                                u16::from_ne_bytes([data[i], data[i + 1]]),
                                u16::from_ne_bytes([data[i + 2], data[i + 3]]),
                                u16::from_ne_bytes([data[i + 4], data[i + 5]]),
                            ]));
                            data[new_loc + 1] = Self::val_u16_to_u8(u16::from_ne_bytes([data[i + 6], data[i + 7]]));
                        }

                        data.truncate(data.len() >> 2);
                        data.shrink_to_fit();
                    }
                    ColorType::GRAY16 => {
                        for i in (0..data.len()).step_by(8) {
                            let new_loc = i >> 2; // divide by 4
                            let new_val = Self::average(&[
                                u16::from_ne_bytes([data[i], data[i + 1]]),
                                u16::from_ne_bytes([data[i + 2], data[i + 3]]),
                                u16::from_ne_bytes([data[i + 4], data[i + 5]]),
                            ])
                            .to_ne_bytes();
                            data[new_loc] = new_val[0];
                            data[new_loc + 1] = new_val[1];
                        }

                        data.truncate(data.len() >> 2);
                        data.shrink_to_fit();
                    }
                    ColorType::GRAYA16 => {
                        for i in (0..data.len()).step_by(8) {
                            let new_loc = i >> 1; // divide by 2
                            let new_val = Self::average(&[
                                u16::from_ne_bytes([data[i], data[i + 1]]),
                                u16::from_ne_bytes([data[i + 2], data[i + 3]]),
                                u16::from_ne_bytes([data[i + 4], data[i + 5]]),
                            ])
                            .to_ne_bytes();
                            data[new_loc] = new_val[0];
                            data[new_loc + 1] = new_val[1];
                            data[new_loc + 2] = data[i + 6];
                            data[new_loc + 3] = data[i + 7];
                        }

                        data.truncate(data.len() >> 1);
                        data.shrink_to_fit();
                    }
                    ColorType::RGB8 => {
                        for i in (0..data.len()).step_by(8) {
                            let new_loc = (i >> 3) + (i >> 2); // multiply by 3/8
                            data[new_loc] = Self::val_u16_to_u8(u16::from_ne_bytes([data[i], data[i + 1]]));
                            data[new_loc + 1] = Self::val_u16_to_u8(u16::from_ne_bytes([data[i + 2], data[i + 3]]));
                            data[new_loc + 2] = Self::val_u16_to_u8(u16::from_ne_bytes([data[i + 4], data[i + 5]]));
                        }

                        data.truncate((data.len() >> 3) + (data.len() >> 2));
                        data.shrink_to_fit();
                    }
                    ColorType::RGBA8 => {
                        for i in (0..data.len()).step_by(8) {
                            let new_loc = i >> 1; // divide by 2
                            data[new_loc] = Self::val_u16_to_u8(u16::from_ne_bytes([data[i], data[i + 1]]));
                            data[new_loc + 1] = Self::val_u16_to_u8(u16::from_ne_bytes([data[i + 2], data[i + 3]]));
                            data[new_loc + 2] = Self::val_u16_to_u8(u16::from_ne_bytes([data[i + 4], data[i + 5]]));
                            data[new_loc + 3] = Self::val_u16_to_u8(u16::from_ne_bytes([data[i + 6], data[i + 7]]));
                        }

                        data.truncate(data.len() >> 1);
                        data.shrink_to_fit();
                    }
                    ColorType::RGB16 => {
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
                    ColorType::RGBA16 => {} // do nothing (same type)
                }
            }
        }
    }

    /// Converts the image to the specified color type.
    /// # Arguments
    /// * ```image_type``` - The color type to which the image will be converted.
    fn convert(&mut self, image_type: ColorType);
}

impl Conversions for Image {
    fn convert(&mut self, image_type: ColorType) {
        if self.image_type != image_type {
            // convert image data
            Self::convert_bytes(&mut self.data, self.image_type, image_type);

            // convert background data
            match &mut self.background_data {
                BackgroundData::Color(current_color) => {
                    match current_color {
                        Color::GRAY8(color) => {
                            match image_type {
                                ColorType::GRAY8 => {} // do nothing (same type)
                                ColorType::GRAYA8 => *current_color = Color::GRAYA8([*color, 255]),
                                ColorType::GRAY16 => *current_color = Color::GRAY16(Self::val_u8_to_u16(*color)),
                                ColorType::GRAYA16 => *current_color = Color::GRAYA16([Self::val_u8_to_u16(*color), 65535]),
                                ColorType::RGB8 => *current_color = Color::RGB8([*color, *color, *color]),
                                ColorType::RGBA8 => *current_color = Color::RGBA8([*color, *color, *color, 255]),
                                ColorType::RGB16 => {
                                    let new_val = Self::val_u8_to_u16(*color);
                                    *current_color = Color::RGB16([new_val, new_val, new_val])
                                }
                                ColorType::RGBA16 => {
                                    let new_val = Self::val_u8_to_u16(*color);
                                    *current_color = Color::RGBA16([new_val, new_val, new_val, 65535])
                                }
                            }
                        }
                        Color::GRAYA8(color) => {
                            match image_type {
                                ColorType::GRAY8 => *current_color = Color::GRAY8(color[0]),
                                ColorType::GRAYA8 => {} // do nothing (same type)
                                ColorType::GRAY16 => *current_color = Color::GRAY16(Self::val_u8_to_u16(color[0])),
                                ColorType::GRAYA16 => *current_color = Color::GRAYA16([Self::val_u8_to_u16(color[0]), Self::val_u8_to_u16(color[1])]),
                                ColorType::RGB8 => *current_color = Color::RGB8([color[0], color[0], color[0]]),
                                ColorType::RGBA8 => *current_color = Color::RGBA8([color[0], color[0], color[0], color[1]]),
                                ColorType::RGB16 => {
                                    let new_val = Self::val_u8_to_u16(color[0]);
                                    *current_color = Color::RGB16([new_val, new_val, new_val])
                                }
                                ColorType::RGBA16 => {
                                    let new_val = Self::val_u8_to_u16(color[0]);
                                    *current_color = Color::RGBA16([new_val, new_val, new_val, Self::val_u8_to_u16(color[1])])
                                }
                            }
                        }
                        Color::GRAY16(color) => {
                            match image_type {
                                ColorType::GRAY8 => *current_color = Color::GRAY8(Self::val_u16_to_u8(*color)),
                                ColorType::GRAYA8 => *current_color = Color::GRAYA8([Self::val_u16_to_u8(*color), 255]),
                                ColorType::GRAY16 => {} // do nothing (same type)
                                ColorType::GRAYA16 => *current_color = Color::GRAYA16([*color, 65535]),
                                ColorType::RGB8 => {
                                    let new_val = Self::val_u16_to_u8(*color);
                                    *current_color = Color::RGB8([new_val, new_val, new_val])
                                }
                                ColorType::RGBA8 => {
                                    let new_val = Self::val_u16_to_u8(*color);
                                    *current_color = Color::RGBA8([new_val, new_val, new_val, 255])
                                }
                                ColorType::RGB16 => *current_color = Color::RGB16([*color, *color, *color]),
                                ColorType::RGBA16 => *current_color = Color::RGBA16([*color, *color, *color, 65535]),
                            }
                        }
                        Color::GRAYA16(color) => {
                            match image_type {
                                ColorType::GRAY8 => *current_color = Color::GRAY8(Self::val_u16_to_u8(color[0])),
                                ColorType::GRAYA8 => *current_color = Color::GRAYA8([Self::val_u16_to_u8(color[0]), Self::val_u16_to_u8(color[1])]),
                                ColorType::GRAY16 => *current_color = Color::GRAY16(color[0]),
                                ColorType::GRAYA16 => {} // do nothing (same type)
                                ColorType::RGB8 => {
                                    let new_val = Self::val_u16_to_u8(color[0]);
                                    *current_color = Color::RGB8([new_val, new_val, new_val])
                                }
                                ColorType::RGBA8 => {
                                    let new_val = Self::val_u16_to_u8(color[0]);
                                    *current_color = Color::RGBA8([new_val, new_val, new_val, Self::val_u16_to_u8(color[1])])
                                }
                                ColorType::RGB16 => *current_color = Color::RGB16([color[0], color[0], color[0]]),
                                ColorType::RGBA16 => *current_color = Color::RGBA16([color[0], color[0], color[0], color[1]]),
                            }
                        }
                        Color::RGB8(color) => {
                            match image_type {
                                ColorType::GRAY8 => *current_color = Color::GRAY8(Self::average(color as &[u8])),
                                ColorType::GRAYA8 => *current_color = Color::GRAYA8([Self::average(color as &[u8]), 255]),
                                ColorType::GRAY16 => *current_color = Color::GRAY16(Self::val_u8_to_u16(Self::average(color as &[u8]))),
                                ColorType::GRAYA16 => *current_color = Color::GRAYA16([Self::val_u8_to_u16(Self::average(color as &[u8])), 65535]),
                                ColorType::RGB8 => {} // do nothing (same type)
                                ColorType::RGBA8 => *current_color = Color::RGBA8([color[0], color[1], color[2], 255]),
                                ColorType::RGB16 => {
                                    *current_color = Color::RGB16([Self::val_u8_to_u16(color[0]), Self::val_u8_to_u16(color[1]), Self::val_u8_to_u16(color[2])])
                                }
                                ColorType::RGBA16 => {
                                    *current_color = Color::RGBA16([Self::val_u8_to_u16(color[0]), Self::val_u8_to_u16(color[1]), Self::val_u8_to_u16(color[2]), 65535])
                                }
                            }
                        }
                        Color::RGBA8(color) => {
                            match image_type {
                                ColorType::GRAY8 => *current_color = Color::GRAY8(Self::average(&color[..3])),
                                ColorType::GRAYA8 => *current_color = Color::GRAYA8([Self::average(&color[..3]), color[3]]),
                                ColorType::GRAY16 => *current_color = Color::GRAY16(Self::val_u8_to_u16(Self::average(&color[..3]))),
                                ColorType::GRAYA16 => *current_color = Color::GRAYA16([Self::val_u8_to_u16(Self::average(&color[..3])), Self::val_u8_to_u16(color[3])]),
                                ColorType::RGB8 => *current_color = Color::RGB8([color[0], color[1], color[2]]),
                                ColorType::RGBA8 => {} // do nothing (same type)
                                ColorType::RGB16 => {
                                    *current_color = Color::RGB16([Self::val_u8_to_u16(color[0]), Self::val_u8_to_u16(color[1]), Self::val_u8_to_u16(color[2])])
                                }
                                ColorType::RGBA16 => {
                                    *current_color = Color::RGBA16([
                                        Self::val_u8_to_u16(color[0]),
                                        Self::val_u8_to_u16(color[1]),
                                        Self::val_u8_to_u16(color[2]),
                                        Self::val_u8_to_u16(color[3]),
                                    ])
                                }
                            }
                        }
                        Color::RGB16(color) => {
                            match image_type {
                                ColorType::GRAY8 => *current_color = Color::GRAY8(Self::val_u16_to_u8(Self::average(color as &[u16]))),
                                ColorType::GRAYA8 => *current_color = Color::GRAYA8([Self::val_u16_to_u8(Self::average(color as &[u16])), 255]),
                                ColorType::GRAY16 => *current_color = Color::GRAY16(Self::average(color as &[u16])),
                                ColorType::GRAYA16 => *current_color = Color::GRAYA16([Self::average(color as &[u16]), 65535]),
                                ColorType::RGB8 => *current_color = Color::RGB8([Self::val_u16_to_u8(color[0]), Self::val_u16_to_u8(color[1]), Self::val_u16_to_u8(color[2])]),
                                ColorType::RGBA8 => {
                                    *current_color = Color::RGBA8([Self::val_u16_to_u8(color[0]), Self::val_u16_to_u8(color[1]), Self::val_u16_to_u8(color[2]), 255])
                                }
                                ColorType::RGB16 => {} // do nothing (same type)
                                ColorType::RGBA16 => *current_color = Color::RGBA16([color[0], color[1], color[2], 65535]),
                            }
                        }
                        Color::RGBA16(color) => {
                            match image_type {
                                ColorType::GRAY8 => *current_color = Color::GRAY8(Self::val_u16_to_u8(Self::average(&color[..3]))),
                                ColorType::GRAYA8 => *current_color = Color::GRAYA8([Self::val_u16_to_u8(Self::average(&color[..3])), Self::val_u16_to_u8(color[3])]),
                                ColorType::GRAY16 => *current_color = Color::GRAY16(Self::average(&color[..3])),
                                ColorType::GRAYA16 => *current_color = Color::GRAYA16([Self::average(&color[..3]), color[3]]),
                                ColorType::RGB8 => *current_color = Color::RGB8([Self::val_u16_to_u8(color[0]), Self::val_u16_to_u8(color[1]), Self::val_u16_to_u8(color[2])]),
                                ColorType::RGBA8 => {
                                    *current_color = Color::RGBA8([
                                        Self::val_u16_to_u8(color[0]),
                                        Self::val_u16_to_u8(color[1]),
                                        Self::val_u16_to_u8(color[2]),
                                        Self::val_u16_to_u8(color[3]),
                                    ])
                                }
                                ColorType::RGB16 => *current_color = Color::RGB16([color[0], color[1], color[2]]),
                                ColorType::RGBA16 => {} // do nothing (same type)
                            }
                        }
                    }
                }
                BackgroundData::Image(data) => {
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
    use crate::colors::{Color, ColorType};
    use crate::image::{Image, Indexing, Utilities};

    fn conversion_test(img1_colors: (Color, Color), img2_colors: (Color, Color)) {
        assert_eq!(ColorType::from(img1_colors.0), ColorType::from(img1_colors.1));
        assert_eq!(ColorType::from(img2_colors.0), ColorType::from(img2_colors.1));

        let new_image_type = ColorType::from(img2_colors.0);

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
        conversion_test((Color::GRAY8(120), Color::GRAY8(140)), (Color::GRAY8(120), Color::GRAY8(140)));
    }

    #[test]
    fn gray8_to_graya8() {
        conversion_test((Color::GRAY8(120), Color::GRAY8(140)), (Color::GRAYA8([120, 255]), Color::GRAYA8([140, 255])));
    }

    #[test]
    fn gray8_to_gray16() {
        conversion_test((Color::GRAY8(120), Color::GRAY8(140)), (Color::GRAY16(30_840), Color::GRAY16(35_980)));
    }

    #[test]
    fn gray8_to_graya16() {
        conversion_test((Color::GRAY8(120), Color::GRAY8(140)), (Color::GRAYA16([30_840, 65_535]), Color::GRAYA16([35_980, 65_535])));
    }

    #[test]
    fn gray8_to_rgb8() {
        conversion_test((Color::GRAY8(120), Color::GRAY8(140)), (Color::RGB8([120, 120, 120]), Color::RGB8([140, 140, 140])));
    }

    #[test]
    fn gray8_to_rgba8() {
        conversion_test(
            (Color::GRAY8(120), Color::GRAY8(140)),
            (Color::RGBA8([120, 120, 120, 255]), Color::RGBA8([140, 140, 140, 255])),
        );
    }

    #[test]
    fn gray8_to_rgb16() {
        conversion_test(
            (Color::GRAY8(120), Color::GRAY8(140)),
            (Color::RGB16([30_840, 30_840, 30_840]), Color::RGB16([35_980, 35_980, 35_980])),
        );
    }

    #[test]
    fn gray8_to_rgba16() {
        conversion_test(
            (Color::GRAY8(120), Color::GRAY8(140)),
            (Color::RGBA16([30_840, 30_840, 30_840, 65_535]), Color::RGBA16([35_980, 35_980, 35_980, 65_535])),
        );
    }

    #[test]
    fn graya8_to_gray8() {
        conversion_test((Color::GRAYA8([120, 255]), Color::GRAYA8([140, 255])), (Color::GRAY8(120), Color::GRAY8(140)));
    }

    #[test]
    fn graya8_to_graya8() {
        conversion_test(
            (Color::GRAYA8([120, 255]), Color::GRAYA8([140, 255])),
            (Color::GRAYA8([120, 255]), Color::GRAYA8([140, 255])),
        );
    }

    #[test]
    fn graya8_to_gray16() {
        conversion_test((Color::GRAYA8([120, 255]), Color::GRAYA8([140, 255])), (Color::GRAY16(30_840), Color::GRAY16(35_980)));
    }

    #[test]
    fn graya8_to_graya16() {
        conversion_test(
            (Color::GRAYA8([120, 255]), Color::GRAYA8([140, 255])),
            (Color::GRAYA16([30_840, 65_535]), Color::GRAYA16([35_980, 65_535])),
        );
    }

    #[test]
    fn graya8_to_rgb8() {
        conversion_test(
            (Color::GRAYA8([120, 255]), Color::GRAYA8([140, 255])),
            (Color::RGB8([120, 120, 120]), Color::RGB8([140, 140, 140])),
        );
    }

    #[test]
    fn graya8_to_rgba8() {
        conversion_test(
            (Color::GRAYA8([120, 255]), Color::GRAYA8([140, 255])),
            (Color::RGBA8([120, 120, 120, 255]), Color::RGBA8([140, 140, 140, 255])),
        );
    }

    #[test]
    fn graya8_to_rgb16() {
        conversion_test(
            (Color::GRAYA8([120, 255]), Color::GRAYA8([140, 255])),
            (Color::RGB16([30_840, 30_840, 30_840]), Color::RGB16([35_980, 35_980, 35_980])),
        );
    }

    #[test]
    fn graya8_to_rgba16() {
        conversion_test(
            (Color::GRAYA8([120, 255]), Color::GRAYA8([140, 255])),
            (Color::RGBA16([30_840, 30_840, 30_840, 65_535]), Color::RGBA16([35_980, 35_980, 35_980, 65_535])),
        );
    }

    #[test]
    fn gray16_to_gray8() {
        conversion_test((Color::GRAY16(30_840), Color::GRAY16(35_980)), (Color::GRAY8(120), Color::GRAY8(140)));
    }

    #[test]
    fn gray16_to_graya8() {
        conversion_test((Color::GRAY16(30_840), Color::GRAY16(35_980)), (Color::GRAYA8([120, 255]), Color::GRAYA8([140, 255])));
    }

    #[test]
    fn gray16_to_gray16() {
        conversion_test((Color::GRAY16(30_840), Color::GRAY16(35_980)), (Color::GRAY16(30_840), Color::GRAY16(35_980)));
    }

    #[test]
    fn gray16_to_graya16() {
        conversion_test(
            (Color::GRAY16(30_840), Color::GRAY16(35_980)),
            (Color::GRAYA16([30_840, 65_535]), Color::GRAYA16([35_980, 65_535])),
        );
    }

    #[test]
    fn gray16_to_rgb8() {
        conversion_test((Color::GRAY16(30_840), Color::GRAY16(35_980)), (Color::RGB8([120, 120, 120]), Color::RGB8([140, 140, 140])));
    }

    #[test]
    fn gray16_to_rgba8() {
        conversion_test(
            (Color::GRAY16(30_840), Color::GRAY16(35_980)),
            (Color::RGBA8([120, 120, 120, 255]), Color::RGBA8([140, 140, 140, 255])),
        );
    }

    #[test]
    fn gray16_to_rgb16() {
        conversion_test(
            (Color::GRAY16(30_840), Color::GRAY16(35_980)),
            (Color::RGB16([30_840, 30_840, 30_840]), Color::RGB16([35_980, 35_980, 35_980])),
        );
    }

    #[test]
    fn gray16_to_rgba16() {
        conversion_test(
            (Color::GRAY16(30_840), Color::GRAY16(35_980)),
            (Color::RGBA16([30_840, 30_840, 30_840, 65_535]), Color::RGBA16([35_980, 35_980, 35_980, 65_535])),
        );
    }

    #[test]
    fn graya16_to_gray8() {
        conversion_test((Color::GRAYA16([30_840, 65_535]), Color::GRAYA16([35_980, 65_535])), (Color::GRAY8(120), Color::GRAY8(140)));
    }

    #[test]
    fn graya16_to_graya8() {
        conversion_test(
            (Color::GRAYA16([30_840, 65_535]), Color::GRAYA16([35_980, 65_535])),
            (Color::GRAYA8([120, 255]), Color::GRAYA8([140, 255])),
        );
    }

    #[test]
    fn graya16_to_gray16() {
        conversion_test(
            (Color::GRAYA16([30_840, 65_535]), Color::GRAYA16([35_980, 65_535])),
            (Color::GRAY16(30_840), Color::GRAY16(35_980)),
        );
    }

    #[test]
    fn graya16_to_graya16() {
        conversion_test(
            (Color::GRAYA16([30_840, 65_535]), Color::GRAYA16([35_980, 65_535])),
            (Color::GRAYA16([30_840, 65_535]), Color::GRAYA16([35_980, 65_535])),
        );
    }

    #[test]
    fn graya16_to_rgb8() {
        conversion_test(
            (Color::GRAYA16([30_840, 65_535]), Color::GRAYA16([35_980, 65_535])),
            (Color::RGB8([120, 120, 120]), Color::RGB8([140, 140, 140])),
        );
    }

    #[test]
    fn graya16_to_rgba8() {
        conversion_test(
            (Color::GRAYA16([30_840, 65_535]), Color::GRAYA16([35_980, 65_535])),
            (Color::RGBA8([120, 120, 120, 255]), Color::RGBA8([140, 140, 140, 255])),
        );
    }

    #[test]
    fn graya16_to_rgb16() {
        conversion_test(
            (Color::GRAYA16([30_840, 65_535]), Color::GRAYA16([35_980, 65_535])),
            (Color::RGB16([30_840, 30_840, 30_840]), Color::RGB16([35_980, 35_980, 35_980])),
        );
    }

    #[test]
    fn graya16_to_rgba16() {
        conversion_test(
            (Color::GRAYA16([30_840, 65_535]), Color::GRAYA16([35_980, 65_535])),
            (Color::RGBA16([30_840, 30_840, 30_840, 65_535]), Color::RGBA16([35_980, 35_980, 35_980, 65_535])),
        );
    }

    #[test]
    fn rgb8_to_gray8() {
        conversion_test((Color::RGB8([110, 120, 130]), Color::RGB8([140, 150, 160])), (Color::GRAY8(120), Color::GRAY8(150)));
    }

    #[test]
    fn rgb8_to_graya8() {
        conversion_test(
            (Color::RGB8([110, 120, 130]), Color::RGB8([140, 150, 160])),
            (Color::GRAYA8([120, 255]), Color::GRAYA8([150, 255])),
        );
    }

    #[test]
    fn rgb8_to_gray16() {
        conversion_test((Color::RGB8([110, 120, 130]), Color::RGB8([140, 150, 160])), (Color::GRAY16(30_840), Color::GRAY16(38_550)));
    }

    #[test]
    fn rgb8_to_graya16() {
        conversion_test(
            (Color::RGB8([110, 120, 130]), Color::RGB8([140, 150, 160])),
            (Color::GRAYA16([30_840, 65_535]), Color::GRAYA16([38_550, 65_535])),
        );
    }

    #[test]
    fn rgb8_to_rgb8() {
        conversion_test(
            (Color::RGB8([110, 120, 130]), Color::RGB8([140, 150, 160])),
            (Color::RGB8([110, 120, 130]), Color::RGB8([140, 150, 160])),
        );
    }

    #[test]
    fn rgb8_to_rgba8() {
        conversion_test(
            (Color::RGB8([110, 120, 130]), Color::RGB8([140, 150, 160])),
            (Color::RGBA8([110, 120, 130, 255]), Color::RGBA8([140, 150, 160, 255])),
        );
    }

    #[test]
    fn rgb8_to_rgb16() {
        conversion_test(
            (Color::RGB8([110, 120, 130]), Color::RGB8([140, 150, 160])),
            (Color::RGB16([28_270, 30_840, 33_410]), Color::RGB16([35_980, 38_550, 41_120])),
        );
    }

    #[test]
    fn rgb8_to_rgba16() {
        conversion_test(
            (Color::RGB8([110, 120, 130]), Color::RGB8([140, 150, 160])),
            (Color::RGBA16([28_270, 30_840, 33_410, 65_535]), Color::RGBA16([35_980, 38_550, 41_120, 65_535])),
        );
    }

    #[test]
    fn rgba8_to_gray8() {
        conversion_test(
            (Color::RGBA8([110, 120, 130, 140]), Color::RGBA8([140, 150, 160, 170])),
            (Color::GRAY8(120), Color::GRAY8(150)),
        );
    }

    #[test]
    fn rgba8_to_graya8() {
        conversion_test(
            (Color::RGBA8([110, 120, 130, 140]), Color::RGBA8([140, 150, 160, 170])),
            (Color::GRAYA8([120, 140]), Color::GRAYA8([150, 170])),
        );
    }

    #[test]
    fn rgba8_to_gray16() {
        conversion_test(
            (Color::RGBA8([110, 120, 130, 140]), Color::RGBA8([140, 150, 160, 170])),
            (Color::GRAY16(30_840), Color::GRAY16(38_550)),
        );
    }

    #[test]
    fn rgba8_to_graya16() {
        conversion_test(
            (Color::RGBA8([110, 120, 130, 140]), Color::RGBA8([140, 150, 160, 170])),
            (Color::GRAYA16([30_840, 35_980]), Color::GRAYA16([38_550, 43_690])),
        );
    }

    #[test]
    fn rgba8_to_rgb8() {
        conversion_test(
            (Color::RGBA8([110, 120, 130, 140]), Color::RGBA8([140, 150, 160, 170])),
            (Color::RGB8([110, 120, 130]), Color::RGB8([140, 150, 160])),
        );
    }

    #[test]
    fn rgba8_to_rgba8() {
        conversion_test(
            (Color::RGBA8([110, 120, 130, 140]), Color::RGBA8([140, 150, 160, 170])),
            (Color::RGBA8([110, 120, 130, 140]), Color::RGBA8([140, 150, 160, 170])),
        );
    }

    #[test]
    fn rgba8_to_rgb16() {
        conversion_test(
            (Color::RGBA8([110, 120, 130, 140]), Color::RGBA8([140, 150, 160, 170])),
            (Color::RGB16([28_270, 30_840, 33_410]), Color::RGB16([35_980, 38_550, 41_120])),
        );
    }

    #[test]
    fn rgba8_to_rgba16() {
        conversion_test(
            (Color::RGBA8([110, 120, 130, 140]), Color::RGBA8([140, 150, 160, 170])),
            (Color::RGBA16([28_270, 30_840, 33_410, 35_980]), Color::RGBA16([35_980, 38_550, 41_120, 43_690])),
        );
    }

    #[test]
    fn rgb16_to_gray8() {
        conversion_test(
            (Color::RGB16([28_270, 30_840, 33_410]), Color::RGB16([35_980, 38_550, 41_120])),
            (Color::GRAY8(120), Color::GRAY8(150)),
        );
    }

    #[test]
    fn rgb16_to_graya8() {
        conversion_test(
            (Color::RGB16([28_270, 30_840, 33_410]), Color::RGB16([35_980, 38_550, 41_120])),
            (Color::GRAYA8([120, 255]), Color::GRAYA8([150, 255])),
        );
    }

    #[test]
    fn rgb16_to_gray16() {
        conversion_test(
            (Color::RGB16([28_270, 30_840, 33_410]), Color::RGB16([35_980, 38_550, 41_120])),
            (Color::GRAY16(30_840), Color::GRAY16(38_550)),
        );
    }

    #[test]
    fn rgb16_to_graya16() {
        conversion_test(
            (Color::RGB16([28_270, 30_840, 33_410]), Color::RGB16([35_980, 38_550, 41_120])),
            (Color::GRAYA16([30_840, 65_535]), Color::GRAYA16([38_550, 65_535])),
        );
    }

    #[test]
    fn rgb16_to_rgb8() {
        conversion_test(
            (Color::RGB16([28_270, 30_840, 33_410]), Color::RGB16([35_980, 38_550, 41_120])),
            (Color::RGB8([110, 120, 130]), Color::RGB8([140, 150, 160])),
        );
    }

    #[test]
    fn rgb16_to_rgba8() {
        conversion_test(
            (Color::RGB16([28_270, 30_840, 33_410]), Color::RGB16([35_980, 38_550, 41_120])),
            (Color::RGBA8([110, 120, 130, 255]), Color::RGBA8([140, 150, 160, 255])),
        );
    }

    #[test]
    fn rgb16_to_rgb16() {
        conversion_test(
            (Color::RGB16([28_270, 30_840, 33_410]), Color::RGB16([35_980, 38_550, 41_120])),
            (Color::RGB16([28_270, 30_840, 33_410]), Color::RGB16([35_980, 38_550, 41_120])),
        );
    }

    #[test]
    fn rgb16_to_rgba16() {
        conversion_test(
            (Color::RGB16([28_270, 30_840, 33_410]), Color::RGB16([35_980, 38_550, 41_120])),
            (Color::RGBA16([28_270, 30_840, 33_410, 65_535]), Color::RGBA16([35_980, 38_550, 41_120, 65_535])),
        );
    }

    #[test]
    fn rgba16_to_gray8() {
        conversion_test(
            (Color::RGBA16([28_270, 30_840, 33_410, 65_535]), Color::RGBA16([35_980, 38_550, 41_120, 65_535])),
            (Color::GRAY8(120), Color::GRAY8(150)),
        );
    }

    #[test]
    fn rgba16_to_graya8() {
        conversion_test(
            (Color::RGBA16([28_270, 30_840, 33_410, 65_535]), Color::RGBA16([35_980, 38_550, 41_120, 65_535])),
            (Color::GRAYA8([120, 255]), Color::GRAYA8([150, 255])),
        );
    }

    #[test]
    fn rgba16_to_gray16() {
        conversion_test(
            (Color::RGBA16([28_270, 30_840, 33_410, 65_535]), Color::RGBA16([35_980, 38_550, 41_120, 65_535])),
            (Color::GRAY16(30_840), Color::GRAY16(38_550)),
        );
    }

    #[test]
    fn rgba16_to_graya16() {
        conversion_test(
            (Color::RGBA16([28_270, 30_840, 33_410, 65_535]), Color::RGBA16([35_980, 38_550, 41_120, 65_535])),
            (Color::GRAYA16([30_840, 65_535]), Color::GRAYA16([38_550, 65_535])),
        );
    }

    #[test]
    fn rgba16_to_rgb8() {
        conversion_test(
            (Color::RGBA16([28_270, 30_840, 33_410, 65_535]), Color::RGBA16([35_980, 38_550, 41_120, 65_535])),
            (Color::RGB8([110, 120, 130]), Color::RGB8([140, 150, 160])),
        );
    }

    #[test]
    fn rgba16_to_rgba8() {
        conversion_test(
            (Color::RGBA16([28_270, 30_840, 33_410, 65_535]), Color::RGBA16([35_980, 38_550, 41_120, 65_535])),
            (Color::RGBA8([110, 120, 130, 255]), Color::RGBA8([140, 150, 160, 255])),
        );
    }

    #[test]
    fn rgba16_to_rgb16() {
        conversion_test(
            (Color::RGBA16([28_270, 30_840, 33_410, 65_535]), Color::RGBA16([35_980, 38_550, 41_120, 65_535])),
            (Color::RGB16([28_270, 30_840, 33_410]), Color::RGB16([35_980, 38_550, 41_120])),
        );
    }

    #[test]
    fn rgba16_to_rgba16() {
        conversion_test(
            (Color::RGBA16([28_270, 30_840, 33_410, 65_535]), Color::RGBA16([35_980, 38_550, 41_120, 65_535])),
            (Color::RGBA16([28_270, 30_840, 33_410, 65_535]), Color::RGBA16([35_980, 38_550, 41_120, 65_535])),
        );
    }
}

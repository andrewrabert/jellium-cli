//! A pure Rust implementation of [woltapp/blurhash][1].
//!
//! ### Decoding
//!
//! ```
//! use blurhash::decode;
//!
//! let pixels = decode("LBAdAqof00WCqZj[PDay0.WB}pof", 50, 50, 1.0).unwrap();
//! assert_eq!(pixels.len(), 50 * 50 * 4);
//! ```
//! [1]: https://github.com/woltapp/blurhash
mod ac;
mod base83;
mod dc;
mod error;
mod util;

pub use error::Error;

use std::f32::consts::PI;
use util::{linear_to_srgb, srgb_to_linear};

/// Calculates the blurhash for an image using the given x and y component counts.
pub fn encode(
    components_x: u32,
    components_y: u32,
    width: u32,
    height: u32,
    rgba_image: &[u8],
) -> Result<String, Error> {
    if !(1..=9).contains(&components_x) || !(1..=9).contains(&components_y) {
        return Err(Error::ComponentsOutOfRange);
    }

    let mut factors: Vec<[f32; 3]> =
        Vec::with_capacity(components_x as usize * components_y as usize);

    for y in 0..components_y {
        for x in 0..components_x {
            let factor = multiply_basis_function(x, y, width, height, rgba_image);
            factors.push(factor);
        }
    }

    let dc = factors[0];
    let ac = &factors[1..];

    let mut blurhash = String::with_capacity(
        // 1 byte for size flag
        1
        // 1 byte for maximum value
        + 1
        // 4 bytes for DC
        + 4
        // 2 bytes for each AC
        + 2 * ac.len(),
    );

    let size_flag = (components_x - 1) + (components_y - 1) * 9;
    base83::encode_into(size_flag, 1, &mut blurhash);

    let maximum_value: f32;
    if !ac.is_empty() {
        let actualmaximum_value = ac
            .iter()
            .flatten()
            .map(|x| f32::abs(*x))
            .reduce(f32::max)
            .unwrap_or(0.0);

        let quantised_maximum_value =
            f32::floor(actualmaximum_value * 166. - 0.5).clamp(0., 82.) as u32;

        maximum_value = (quantised_maximum_value + 1) as f32 / 166.;
        base83::encode_into(quantised_maximum_value, 1, &mut blurhash);
    } else {
        maximum_value = 1.;
        base83::encode_into(0, 1, &mut blurhash);
    }

    base83::encode_into(dc::encode(dc), 4, &mut blurhash);

    for i in 0..components_y * components_x - 1 {
        base83::encode_into(ac::encode(ac[i as usize], maximum_value), 2, &mut blurhash);
    }

    Ok(blurhash)
}

fn multiply_basis_function(
    component_x: u32,
    component_y: u32,
    width: u32,
    height: u32,
    rgb: &[u8],
) -> [f32; 3] {
    let mut r = 0.;
    let mut g = 0.;
    let mut b = 0.;
    let normalisation = match (component_x, component_y) {
        (0, 0) => 1.,
        _ => 2.,
    };

    let bytes_per_row = width * 4;

    let pi_cx_over_width = PI * component_x as f32 / width as f32;
    let pi_cy_over_height = PI * component_y as f32 / height as f32;

    let mut cos_pi_cx_over_width = vec![0.; width as usize];
    for x in 0..width {
        cos_pi_cx_over_width[x as usize] = f32::cos(pi_cx_over_width * x as f32);
    }

    let mut cos_pi_cy_over_height = vec![0.; height as usize];
    for y in 0..height {
        cos_pi_cy_over_height[y as usize] = f32::cos(pi_cy_over_height * y as f32);
    }

    for y in 0..height {
        for x in 0..width {
            let basis = cos_pi_cx_over_width[x as usize] * cos_pi_cy_over_height[y as usize];
            r += basis * srgb_to_linear(rgb[(4 * x + y * bytes_per_row) as usize]);
            g += basis * srgb_to_linear(rgb[(4 * x + 1 + y * bytes_per_row) as usize]);
            b += basis * srgb_to_linear(rgb[(4 * x + 2 + y * bytes_per_row) as usize]);
        }
    }

    let scale = normalisation / (width * height) as f32;

    [r * scale, g * scale, b * scale]
}

/// Decodes the given blurhash to an image of the specified size into an existing buffer.
///
/// The punch parameter can be used to de- or increase the contrast of the
/// resulting image.
pub fn decode_into(
    pixels: &mut [u8],
    blurhash: &str,
    width: u32,
    height: u32,
    punch: f32,
) -> Result<(), Error> {
    if !blurhash.is_ascii() {
        return Err(Error::InvalidAscii);
    }

    let (num_x, num_y) = components(blurhash)?;

    assert_eq!(
        (width * height * 4) as usize,
        pixels.len(),
        "buffer length equals 4 * width * height"
    );

    let quantised_maximum_value = base83::decode(&blurhash[1..2])?;
    let maximum_value = (quantised_maximum_value + 1) as f32 / 166.;

    let mut colors = vec![[0.; 3]; num_x * num_y];

    for i in 0..colors.len() {
        if i == 0 {
            let value = base83::decode(&blurhash[2..6])?;
            colors[i] = dc::decode(value as u32);
        } else {
            let value = base83::decode(&blurhash[4 + i * 2..6 + i * 2])?;
            colors[i] = ac::decode(value as u32, maximum_value * punch);
        }
    }

    let colors: Vec<_> = colors.chunks(num_x).collect();

    let bytes_per_row = width as usize * 4;

    let pi_over_height = PI / height as f32;
    let pi_over_width = PI / width as f32;

    // Precompute the cosines
    let mut cos_i_pi_x_over_width = vec![0.; width as usize * num_x];
    let mut cos_j_pi_y_over_height = vec![0.; height as usize * num_y];

    for x in 0..width {
        let pi_x_over_width = x as f32 * pi_over_width;
        for i in 0..num_x {
            cos_i_pi_x_over_width[x as usize * num_x + i] = f32::cos(pi_x_over_width * i as f32);
        }
    }

    for y in 0..height {
        let pi_y_over_height = y as f32 * pi_over_height;
        for j in 0..num_y {
            cos_j_pi_y_over_height[y as usize * num_y + j] = f32::cos(j as f32 * pi_y_over_height);
        }
    }

    // Hint to the optimizer that the length of the slices is correct
    assert!(height as usize * num_y == cos_j_pi_y_over_height.len());
    assert!(width as usize * num_x == cos_i_pi_x_over_width.len());

    for y in 0..height as usize {
        let pixels = &mut pixels[y * bytes_per_row..][..bytes_per_row];

        // More optimizer hints.
        assert!(y * num_y + num_y <= cos_j_pi_y_over_height.len());

        for x in 0..width as usize {
            let mut pixel = [0.; 3];

            let cos_j_pi_y_over_height = &cos_j_pi_y_over_height[y * num_y..][..num_y];
            let cos_i_pi_x_over_width = &cos_i_pi_x_over_width[x * num_x..][..num_x];

            assert_eq!(cos_j_pi_y_over_height.len(), colors.len());
            assert_eq!(cos_j_pi_y_over_height.len(), num_y);

            for (cos_j, colors) in cos_j_pi_y_over_height.iter().zip(colors.iter()) {
                assert_eq!(cos_i_pi_x_over_width.len(), colors.len());
                assert_eq!(cos_i_pi_x_over_width.len(), num_x);

                for (cos_i, color) in cos_i_pi_x_over_width.iter().zip(colors.iter()) {
                    let basis = cos_i * cos_j;

                    pixel[0] += color[0] * basis;
                    pixel[1] += color[1] * basis;
                    pixel[2] += color[2] * basis;
                }
            }

            let int_r = linear_to_srgb(pixel[0]);
            let int_g = linear_to_srgb(pixel[1]);
            let int_b = linear_to_srgb(pixel[2]);

            let pixels = &mut pixels[4 * x..][..4];

            pixels[0] = int_r;
            pixels[1] = int_g;
            pixels[2] = int_b;
            pixels[3] = 255u8;
        }
    }
    Ok(())
}

/// Decodes the given blurhash to an image of the specified size.
///
/// The punch parameter can be used to de- or increase the contrast of the
/// resulting image.
pub fn decode(blurhash: &str, width: u32, height: u32, punch: f32) -> Result<Vec<u8>, Error> {
    let bytes_per_row = width * 4;
    let mut pixels = vec![0; (bytes_per_row * height) as usize];
    decode_into(&mut pixels, blurhash, width, height, punch).map(|()| pixels)
}

fn components(blurhash: &str) -> Result<(usize, usize), Error> {
    if blurhash.len() < 6 {
        return Err(Error::HashTooShort);
    }

    let size_flag = base83::decode(&blurhash[0..1])?;
    let num_y = (f32::floor(size_flag as f32 / 9.) + 1.) as usize;
    let num_x = ((size_flag % 9) + 1) as usize;

    let expected = 4 + 2 * num_x * num_y;
    if blurhash.len() != expected {
        return Err(Error::LengthMismatch {
            expected,
            actual: blurhash.len(),
        });
    }

    Ok((num_x, num_y))
}

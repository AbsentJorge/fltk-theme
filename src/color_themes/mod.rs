mod black;
mod dark;
pub mod fleet;
mod gray;
pub mod jewel;
mod shake;
mod tan;

pub use black::*;
pub use dark::*;
pub use gray::*;
pub use shake::*;
pub use tan::*;

use crate::{cmap, ColorMap};

pub(crate) const fn gray_ramp(dark: u8, light: u8, n: u8) -> u8 {
    let l = light as u16;
    let d = dark as u16;
    (d + ((l - d) * n as u16 + 11) / 23) as u8
}

pub(crate) const fn gray_ramp_inv(light: u8, dark: u8, n: u8) -> u8 {
    let l = light as u16;
    let d = dark as u16;
    (l - ((l - d) * n as u16 + 11) / 23) as u8
}

pub(crate) const fn cube_chan(i: u8, steps: u8, max: u8) -> u8 {
    ((i as u16 * max as u16 + ((steps - 1) / 2) as u16) / (steps as u16 - 1)) as u8
}

pub(crate) const fn make_dark_theme(
    overrides: &[(u8, u8, u8, u8)],
    ramp: (u8, u8),
    cube_max: u8,
) -> [ColorMap; 256] {
    let (light, dark) = ramp;
    let mut map = [cmap!(255, 255, 255, 255); 256];

    let mut i = 32;
    while i <= 55 {
        let v = gray_ramp_inv(light, dark, (i - 32) as u8);
        let idx8 = i as u8;
        map[i] = cmap!(idx8, v, v, v);
        i += 1;
    }
    i = 56;
    while i < 256 {
        let n = (i - 56) as u8;
        let b = n / 40;
        let r = (n / 8) % 5;
        let g = n % 8;
        let rr = cube_chan(r, 5, cube_max);
        let gg = cube_chan(g, 8, cube_max);
        let bb = cube_chan(b, 5, cube_max);
        let idx8 = i as u8;
        map[i] = cmap!(idx8, rr, gg, bb);
        i += 1;
    }

    let mut j = 0;
    while j < overrides.len() {
        let (idx, r, g, b) = overrides[j];
        map[idx as usize] = cmap!(idx, r, g, b);
        j += 1;
    }
    map
}

pub(crate) const fn make_light_theme(
    overrides: &'static [(u8, u8, u8, u8)],
    ramp: (u8, u8),
    cube_max: u8,
) -> [ColorMap; 256] {
    let (dark, light) = ramp;
    let mut map = [cmap!(255, 255, 255, 255); 256];

    let mut i = 32;
    while i <= 55 {
        let v = gray_ramp(dark, light, (i - 32) as u8);
        let idx8 = i as u8;
        map[i] = cmap!(idx8, v, v, v);
        i += 1;
    }
    i = 56;
    while i < 256 {
        let n = (i - 56) as u8;
        let b = n / 40;
        let r = (n / 8) % 5;
        let g = n % 8;
        let rr = cube_chan(r, 5, cube_max);
        let gg = cube_chan(g, 8, cube_max);
        let bb = cube_chan(b, 5, cube_max);
        let idx8 = i as u8;
        map[i] = cmap!(idx8, rr, gg, bb);
        i += 1;
    }

    let mut j = 0;
    while j < overrides.len() {
        let (idx, r, g, b) = overrides[j];
        map[idx as usize] = cmap!(idx, r, g, b);
        j += 1;
    }
    map
}

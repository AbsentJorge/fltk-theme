use super::{make_dark_theme, make_light_theme, ColorMap};

const DARK_BG_COLOR: (u8, u8, u8, u8) = (49, 24, 24, 24);
const DARK_BG2_COLOR: (u8, u8, u8, u8) = (7, 31, 31, 31);
const DARK_FG_COLOR: (u8, u8, u8, u8) = (0, 255, 255, 255);

const LIGHT_BG_COLOR: (u8, u8, u8, u8) = (49, 255, 255, 255);
const LIGHT_BG2_COLOR: (u8, u8, u8, u8) = (7, 240, 240, 240);
const LIGHT_FG_COLOR: (u8, u8, u8, u8) = (0, 0, 0, 0);

// Dark themes:

pub const DIAMOND_DARK: [ColorMap; 256] = make_dark_theme(
    &[
        DARK_BG_COLOR,
        DARK_BG2_COLOR,
        DARK_FG_COLOR,
        (39, 0, 42, 75),    // Dark3
        (45, 0, 64, 113),   // Dark2
        (47, 0, 96, 170),   // Dark1
        (50, 36, 132, 207), // Light1
        (52, 60, 124, 174), // Light2
        (54, 76, 119, 152), // Light3
        (15, 0, 144, 255),  // Selection
        (16, 0, 144, 255),  // Accent
    ],
    (0xFF, 0x00),
    255,
);

pub const EMERALD_DARK: [ColorMap; 256] = make_dark_theme(
    &[
        DARK_BG_COLOR,
        DARK_BG2_COLOR,
        DARK_FG_COLOR,
        (39, 0, 75, 9),    // Dark3
        (45, 0, 113, 14),  // Dark2
        (47, 0, 170, 21),  // Dark1
        (50, 36, 207, 57), // Light1
        (52, 60, 174, 74), // Light2
        (54, 76, 152, 85), // Light3
        (15, 0, 255, 32),  // Selection
        (16, 0, 255, 32),  // Accent
    ],
    (0xFF, 0x00),
    255,
);

pub const AMETHYST_DARK: [ColorMap; 256] = make_dark_theme(
    &[
        DARK_BG_COLOR,
        DARK_BG2_COLOR,
        DARK_FG_COLOR,
        (39, 63, 35, 75),    // Dark3
        (45, 95, 53, 113),   // Dark2
        (47, 142, 80, 170),  // Dark1
        (50, 178, 116, 207), // Light1
        (52, 155, 114, 174), // Light2
        (54, 140, 112, 152), // Light3
        (15, 212, 120, 255),
        (16, 212, 120, 255),
    ],
    (0xFF, 0x00),
    255,
);

pub const CITRINE_DARK: [ColorMap; 256] = make_dark_theme(
    &[
        DARK_BG_COLOR,
        DARK_BG2_COLOR,
        DARK_FG_COLOR,
        (39, 75, 58, 14),
        (45, 113, 87, 21),
        (47, 170, 131, 32),
        (50, 255, 215, 116),
        (52, 255, 228, 161),
        (54, 255, 236, 192),
        (15, 255, 196, 48),
        (16, 255, 196, 48),
    ],
    (0xFF, 0x00),
    255,
);

pub const RUBY_DARK: [ColorMap; 256] = make_dark_theme(
    &[
        DARK_BG_COLOR,
        DARK_BG2_COLOR,
        DARK_FG_COLOR,
        (39, 75, 14, 14),
        (45, 113, 21, 21),
        (47, 170, 32, 32),
        (50, 255, 116, 116),
        (52, 255, 161, 161),
        (54, 255, 192, 192),
        (15, 255, 48, 48),
        (16, 255, 48, 48),
    ],
    (0xFF, 0x00),
    255,
);

pub const PEARL_DARK: [ColorMap; 256] = make_dark_theme(
    &[
        DARK_BG_COLOR,
        DARK_BG2_COLOR,
        DARK_FG_COLOR,
        (39, 67, 67, 67),
        (45, 100, 100, 100),
        (47, 150, 150, 150),
        (50, 234, 234, 234),
        (52, 240, 240, 240),
        (54, 244, 244, 244),
        (15, 224, 224, 224),
        (16, 224, 224, 224),
    ],
    (0xFF, 0x00),
    255,
);

// Light themes:

pub const DIAMOND_LIGHT: [ColorMap; 256] = make_light_theme(
    &[
        LIGHT_BG_COLOR,
        LIGHT_BG2_COLOR,
        LIGHT_FG_COLOR,
        (39, 0, 42, 75),    // Dark3
        (45, 0, 64, 113),   // Dark2
        (47, 0, 96, 170),   // Dark1
        (50, 36, 132, 207), // Light1
        (52, 60, 124, 174), // Light2
        (54, 76, 119, 152), // Light3
        (15, 0, 144, 255),  // Selection
        (16, 0, 144, 255),  // Accent
    ],
    (0x00, 0xFF),
    255,
);

pub const EMERALD_LIGHT: [ColorMap; 256] = make_light_theme(
    &[
        LIGHT_BG_COLOR,
        LIGHT_BG2_COLOR,
        LIGHT_FG_COLOR,
        (39, 0, 75, 9),    // Dark3
        (45, 0, 113, 14),  // Dark2
        (47, 0, 170, 21),  // Dark1
        (50, 36, 207, 57), // Light1
        (52, 60, 174, 74), // Light2
        (54, 76, 152, 85), // Light3
        (15, 0, 255, 32),  // Selection
        (16, 0, 255, 32),  // Accent
    ],
    (0x00, 0xFF),
    255,
);

pub const AMETHYST_LIGHT: [ColorMap; 256] = make_light_theme(
    &[
        LIGHT_BG_COLOR,
        LIGHT_BG2_COLOR,
        LIGHT_FG_COLOR,
        (39, 63, 35, 75),    // Dark3
        (45, 95, 53, 113),   // Dark2
        (47, 142, 80, 170),  // Dark1
        (50, 178, 116, 207), // Light1
        (52, 155, 114, 174), // Light2
        (54, 140, 112, 152), // Light3
        (15, 212, 120, 255),
        (16, 212, 120, 255),
    ],
    (0x00, 0xFF),
    255,
);

pub const CITRINE_LIGHT: [ColorMap; 256] = make_light_theme(
    &[
        LIGHT_BG_COLOR,
        LIGHT_BG2_COLOR,
        LIGHT_FG_COLOR,
        (39, 75, 58, 14),
        (45, 113, 87, 21),
        (47, 170, 131, 32),
        (50, 255, 215, 116),
        (52, 255, 228, 161),
        (54, 255, 236, 192),
        (15, 255, 196, 48),
        (16, 255, 196, 48),
    ],
    (0x00, 0xFF),
    255,
);

pub const RUBY_LIGHT: [ColorMap; 256] = make_light_theme(
    &[
        LIGHT_BG_COLOR,
        LIGHT_BG2_COLOR,
        LIGHT_FG_COLOR,
        (39, 75, 14, 14),
        (45, 113, 21, 21),
        (47, 170, 32, 32),
        (50, 255, 116, 116),
        (52, 255, 161, 161),
        (54, 255, 192, 192),
        (15, 255, 48, 48),
        (16, 255, 48, 48),
    ],
    (0x00, 0xFF),
    255,
);

pub const PEARL_LIGHT: [ColorMap; 256] = make_light_theme(
    &[
        LIGHT_BG_COLOR,
        LIGHT_BG2_COLOR,
        LIGHT_FG_COLOR,
        (39, 67, 67, 67),
        (45, 100, 100, 100),
        (47, 150, 150, 150),
        (50, 234, 234, 234),
        (52, 240, 240, 240),
        (54, 244, 244, 244),
        (15, 224, 224, 224),
        (16, 224, 224, 224),
    ],
    (0x00, 0xFF),
    255,
);

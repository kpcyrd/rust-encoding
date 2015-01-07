// AUTOGENERATED FROM index-iso-8859-4.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index index-iso-8859-4.txt see the Encoding Standard
// https://encoding.spec.whatwg.org/
//
// Identifier: 72f29c92344d351fe9e74a946e7e0468d76d542c6894ff82982cb652ebe0feb7
// Date: 2014-12-19

static FORWARD_TABLE: &'static [u16] = &[
    128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
    143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157,
    158, 159, 160, 260, 312, 342, 164, 296, 315, 167, 168, 352, 274, 290, 358,
    173, 381, 175, 176, 261, 731, 343, 180, 297, 316, 711, 184, 353, 275, 291,
    359, 330, 382, 331, 256, 193, 194, 195, 196, 197, 198, 302, 268, 201, 280,
    203, 278, 205, 206, 298, 272, 325, 332, 310, 212, 213, 214, 215, 216, 370,
    218, 219, 220, 360, 362, 223, 257, 225, 226, 227, 228, 229, 230, 303, 269,
    233, 281, 235, 279, 237, 238, 299, 273, 326, 333, 311, 244, 245, 246, 247,
    248, 371, 250, 251, 252, 361, 363, 729,
];

/// Returns the index code point for pointer `code` in this index.
#[inline]
#[stable]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[(code - 0x80) as uint]
}

static BACKWARD_TABLE_LOWER: &'static [u8] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138,
    139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153,
    154, 155, 156, 157, 158, 159, 160, 0, 0, 0, 164, 0, 0, 167, 168, 0, 0, 0,
    0, 173, 0, 175, 176, 0, 0, 0, 180, 0, 0, 0, 184, 0, 0, 0, 0, 0, 0, 0, 0,
    193, 194, 195, 196, 197, 198, 0, 0, 201, 0, 203, 0, 205, 206, 0, 0, 0, 0,
    0, 212, 213, 214, 215, 216, 0, 218, 219, 220, 0, 0, 223, 0, 225, 226, 227,
    228, 229, 230, 0, 0, 233, 0, 235, 0, 237, 238, 0, 0, 0, 0, 0, 244, 245,
    246, 247, 248, 0, 250, 251, 252, 0, 0, 0, 192, 224, 0, 0, 161, 177, 0, 0,
    0, 0, 0, 0, 200, 232, 0, 0, 208, 240, 170, 186, 0, 0, 204, 236, 202, 234,
    0, 0, 0, 0, 0, 0, 0, 0, 171, 187, 0, 0, 0, 0, 165, 181, 207, 239, 0, 0,
    199, 231, 0, 0, 0, 0, 0, 0, 211, 243, 162, 0, 0, 166, 182, 0, 0, 0, 0, 0,
    0, 0, 0, 209, 241, 0, 0, 0, 189, 191, 210, 242, 0, 0, 0, 0, 0, 0, 0, 0,
    163, 179, 0, 0, 0, 0, 0, 0, 0, 0, 169, 185, 0, 0, 0, 0, 172, 188, 221, 253,
    222, 254, 0, 0, 0, 0, 0, 0, 217, 249, 0, 0, 0, 0, 0, 0, 0, 0, 0, 174, 190,
    0, 0, 0, 0, 0, 0, 0, 0, 183, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 255, 0, 178, 0, 0, 0, 0,
];

static BACKWARD_TABLE_UPPER: &'static [u16] = &[
    0, 0, 0, 0, 32, 64, 96, 128, 160, 192, 224, 256, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 288,
];

/// Returns the index pointer for code point `code` in this index.
#[inline]
#[stable]
pub fn backward(code: u32) -> u8 {
    let offset = (code >> 5) as uint;
    let offset = if offset < 23 {BACKWARD_TABLE_UPPER[offset] as uint} else {0};
    BACKWARD_TABLE_LOWER[offset + ((code & 31) as uint)]
}

#[cfg(test)]
single_byte_tests!(
    mod = iso_8859_4
);

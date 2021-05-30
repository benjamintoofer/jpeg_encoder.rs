/// A two-byte code in which the first byte is hexadecimal FF (X’FF’) and the second byte is a value
/// between 1 and hexadecimal FE (X’FE’)

pub const MARKER_START: u8 = 0xFF;
/// Start Of Frame markers
///
///     non-differential, Huffman coding
///
///         Baseline DCT
pub const SOF0: u8 = 0xC0; 
///         Extended sequential DCT
pub const SOF1: u8 = 0xC1; 
///         Progressive DCT
pub const SOF2: u8 = 0xC2;
///         Lossless (sequential)
pub const SOF3: u8 = 0xC3;

///     differential, Huffman coding
///
///         Differential sequential DCT
pub const SOF5: u8 = 0xC5; 
///         Differential progressive DCT
pub const SOF6: u8 = 0xC6; 
///         Differential lossless (sequential)
pub const SOF7: u8 = 0xC7; 


/// Huffman table specification
///
///     Define Huffman table(s)
pub const DHT: u8 = 0xC4;

/// Other markers
///
///     Start of image
pub const SOI: u8 = 0xD8;
///     End of image
pub const EOI: u8 = 0xD9;
///     Start of scan
pub const SOS: u8 = 0xDA;
///     Define quantization table(s)
pub const DQT: u8 = 0xDB;
///     Reserved for application segments
pub const APP0: u8 = 0xE0;
pub const APP1: u8 = 0xE1;
pub const APP2: u8 = 0xE2;
pub const APP3: u8 = 0xE3;
pub const APP4: u8 = 0xE4;
pub const APP5: u8 = 0xE5;
pub const APP6: u8 = 0xE6;
pub const APP7: u8 = 0xE7;
pub const APP8: u8 = 0xE8;
pub const APP9: u8 = 0xE9;





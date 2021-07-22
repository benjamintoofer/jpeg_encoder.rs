use std::fs;

use image::{self, EncodableLayout, GenericImageView, Pixel};
use ndarray::arr2;

pub mod jpeg;
pub mod decoder;
pub mod markers;

/**
TODO (benjamintoofer@gmail.com)
Consider using ndarray library with blas for optimal vector operations
Record performance difference
*/
const BLOCK_SIZE: usize = 8usize;

const Q_50_luma: [[ i32; 8]; 8] = [
    [16, 11, 10, 16, 24, 40, 51, 61],
    [12, 12, 14, 19, 26, 58, 60, 55],
    [14, 13, 16, 24, 40, 57, 69, 56],
    [14, 17, 22, 29, 51, 87, 80, 62],
    [18, 22, 37, 56, 68, 109, 103, 77],
    [24, 35, 55, 64, 81, 104, 113, 92],
    [49, 64, 78, 87, 103, 121, 120, 101],
    [72, 92, 95, 98, 112, 100, 103, 99]
];

const Q_50_chroma: [[ i32; 8]; 8] = [
    [17, 18, 24, 47, 99, 99, 99, 99],
    [18, 21, 26, 66, 99, 99, 99, 99],
    [24, 26, 56, 99, 99, 99, 99, 99],
    [47, 66, 99, 99, 99, 99, 99, 99],
    [99, 99, 99, 99, 99, 99, 99, 99],
    [99, 99, 99, 99, 99, 99, 99, 99],
    [99, 99, 99, 99, 99, 99, 99, 99],
    [99, 99, 99, 99, 99, 99, 99, 99]
];

const S: [f32; 8] = [
	0.353553390593273762200422,
	0.254897789552079584470970,
	0.270598050073098492199862,
	0.300672443467522640271861,
	0.353553390593273762200422,
	0.449988111568207852319255,
	0.653281482438188263928322,
	1.281457723870753089398043,
];

const A: [f32; 6] = [
	std::f32::NAN,
	0.707106781186547524400844,
	0.541196100146196984399723,
	0.707106781186547524400844,
	1.306562964876376527856643,
	0.382683432365089771728460,
];
enum SubsampleType {
    V4_4_4,
    V4_2_2,
    V4_2_0
}

impl SubsampleType {
    pub fn get_row_divisor(&self) -> u8 {
        match self {
            SubsampleType::V4_4_4 => {1}
            SubsampleType::V4_2_2 => {2}
            SubsampleType::V4_2_0 => {2}
        }
    }

    pub fn get_col_divisor(&self) -> u8 {
        match self {
            SubsampleType::V4_4_4 => {1}
            SubsampleType::V4_2_2 => {1}
            SubsampleType::V4_2_0 => {2}
        }
    }
}

fn main() {
    println!("Hello, world!");
    let file_path = "./assets/fanta_image.jpg";

    let file = fs::read(file_path);
     if let Ok(image_data) = file {
        let result = decoder::Decoder::decode(image_data);
        match result {
            Ok(res) => {println!("{}", res)}
            Err(err) => {println!("{}", err)}
        }
     }
    // let img = image::open(file).unwrap();
    // println!("{:?}", img.dimensions());
    // println!("{:?}", img.color());
    // println!("{:?}", img.as_bytes().len());

    // This will be used to grab 64 pixels at a time
    // let rgb_image = img.into_rgb8();
    // let temp = rgb_image.as_bytes();
    // let mut y_prime:[[ f32; 8]; 8] = [[0f32; 8]; 8];
    // let mut y_prime:[ f32; 8] = [0f32; 8];
    let mut y_prime:[[ f32; 8]; 8] = [
        [-64., -68., -71., -72., -80., -81., -81., -85.],
        [-67., -70., -75., -76., -80., -79., -76., -75.],
        [-61., -68., -75., -75., -79., -81., -80., -74.],
        [-60., -67., -65., -65., -66., -63., -63., -64.],
        [-57., -67., -58., -65., -59., -54., -40., -40.],
        [-45., -36., -26., -23., -21., -17., -18., -13.],
        [-33., -20., -20., -4., -6., 2., 0., 0.],
        [-21., -10., -3., 6., 9., 14., 13., 9.]
    ];
    // let mut buff = [0i32; 64];
    // get_zigzag(&Q_50_luma, &mut buff);
    // println!("BUFF {:?}", buff);
    // for i in 0usize..64 {
    //     let row = i / 8;
    //     let col = i % 8;
    //     let pixel = img.get_pixel(row as u32, col as u32);
    //     let channel = pixel.channels();
    //     let ycrcb_vector: [f32; 3] = rgb_2_ycrcb(channel[0], channel[1], channel[2]);
    //     y_prime[row][col] = ycrcb_vector[0] - 128f32;
    // }
    // println!("PIXEL: {:?}", pixel.0);
    // println!("PIXEL: {:?}", pixel.channels());
    // let channel = pixel.channels();
    // let ycrcb_vector = rgb_2_ycrcb(channel[0], channel[1], channel[2]);
    // println!("ycrcb: {:?}", ycrcb_vector);
    // println!("rgb: {:?}", ycrcb_2_rgb(ycrcb_vector[0], ycrcb_vector[1], ycrcb_vector[2]));

    let mut input = [0f32; 8];
    let mut intermediate = [5f32; 8];
    let mut output = [0f32; 8];
    
    // let dct = Type2And3Butterfly8::new();
    // println!("OUTPUT {:?}", y_prime);
    // dct.process_dct2(&mut y_prime);
    // dct.process_dct2_with_scratch(&mut y_prime, &mut intermediate);
    // dct.process_dct3_with_scratch(&mut intermediate, &mut output);

    // println!("BEFORE {:?}", y_prime);
    // let mut transposed: [[ f32; 8]; 8] = [[ 0f32; 8]; 8];
    // let mut transformed: [[ f32; 8]; 8] = [[ 0f32; 8]; 8];
    // for (i, elem) in y_prime.into_iter().enumerate() {
    //     transformed[i] = transform(&elem);
    // }
    // println!("AFTER {:?}", transformed);
    // transpose(&transformed, &mut transposed);
    // println!("TRANSPOSE {:?}", transposed);
    // for (i, elem) in transposed.into_iter().enumerate() {
    //     transformed[i] = transform(&elem);
    // }
    // transpose(&transformed, &mut transposed);
    // println!("\n\nDONE {:?}", transposed);

    // let mut luma_quantized_matrix:[[ i32; 8]; 8] = [[ 0i32; 8]; 8];
    // let mut chroma_quantized_matrix:[[ i32; 8]; 8] = [[ 0i32; 8]; 8];
    // generate_quantize_table_quality(Q_50_luma, 50, &mut luma_quantized_matrix);
    // // generate_quantize_table_quality(Q_50_chroma, 50, &mut chroma_quantized_matrix);

    // divide_matrix(&transposed, &luma_quantized_matrix, &mut chroma_quantized_matrix);
    // println!("\n\nDONE {:?}", chroma_quantized_matrix);
}

fn compress_jpeg_brute() {
    // Iterate through each pixel and convert from RGB -> YCrCb

}

fn transpose(matrix: &[[ f32; BLOCK_SIZE]; BLOCK_SIZE], new_matrix: &mut [[ f32; 8]; 8]) {
    for i in 0..BLOCK_SIZE {
        for j in 0..BLOCK_SIZE {
            new_matrix[j][i] = matrix[i][j];
        }
    }
}


fn rgb_2_ycrcb(r:u8, b: u8, g:u8) -> [f32; 3] {
    let ycrcb_offset: [f32; 3] = [16.0, 128.0, 128.0];
    let matrix_rgb_2_ycrcb: &'static [[f32; 3]; 3] = &[
        [0.257, 0.504, 0.098],
        [-0.148, -0.291, 0.439],
        [0.439, -0.368, -0.071]
    ];
    let rgb_vector: [f32; 3] = [
        r as f32,
        b as f32,
        g as f32
    ];
    
    let temp = multiply(matrix_rgb_2_ycrcb, rgb_vector);
    add(temp, ycrcb_offset)
}

fn ycrcb_2_rgb(y:f32, cr:f32, cb: f32)-> [f32; 3] {
    let ycrcb_offset: [f32; 3] = [16.0, 128.0, 128.0];
    let matrix_rgb_2_ycrcb: &'static [[f32; 3]; 3] = &[
        [1.164, 0.0, 1.596],
        [1.164, -0.392, -0.813],
        [1.164, 2.017, 0.0]
    ];

    let ycrcb_vector: [f32; 3] = [y,cr,cb];
    let temp = subtract(ycrcb_vector, ycrcb_offset);
    multiply(matrix_rgb_2_ycrcb, temp)
}

fn down_sample() -> u32 {
    0
}

fn multiply(matrix: &'static [[f32; 3]; 3], vector:[f32; 3]) -> [f32; 3] {
    return [
        ((matrix[0][0] * vector[0]) + (matrix[0][1] * vector[1]) + (matrix[0][2] * vector[2])),
        ((matrix[1][0] * vector[0]) + (matrix[1][1] * vector[1]) + (matrix[1][2] * vector[2])),
        ((matrix[2][0] * vector[0]) + (matrix[2][1] * vector[1]) + (matrix[2][2] * vector[2])),
    ]
}

fn add(vector_a: [f32; 3], vector_b: [f32; 3]) -> [f32; 3] {
    return [
        vector_a[0] + vector_b[0],
        vector_a[1] + vector_b[1],
        vector_a[2] + vector_b[2]
    ]
}

fn subtract(vector_a: [f32; 3], vector_b: [f32; 3]) -> [f32; 3] {
    return [
        vector_a[0] - vector_b[0],
        vector_a[1] - vector_b[1],
        vector_a[2] - vector_b[2]
    ]
}

fn apply_discrete_cosine_transform() -> u8 {

    // Input is 8 X 8 pixel block of eithe y, Cr, cb

    // Shift by -128 to change scale from 0 - 255 -> -128 - 128 (For cosine
    
    // Get DCT 2 coefficients. ITs a contribution of each frequency block with some weight (the coefficient)
    0
}

fn compress_jpeg_optimal() {

}
struct JPEGData {
    amplitude: u8,
    run_length: u8,
    size: u8,
}

impl JPEGData {
    pub fn to_byte_pair() -> u16 {
        0
    }
}
fn entropy_encoding(previous_dc: i32, block: [[ i32; 8]; 8]) {
    let mut buffer: [i32; 64] = [0i32; 64];
    let temp = get_zigzag(&block, &mut buffer);
}

fn get_zigzag(block: &[[ i32; 8]; 8], buffer: &mut [i32; 64]) {
    let sum_limit = (BLOCK_SIZE * 2) - 1;
    let limit = BLOCK_SIZE - 1;
    let mut i = 0;
    let mut col: usize;
    let mut row: usize;
    let mut buffer_index = 0;
    while i < sum_limit {
        if i % 2 == 0 {
            row = usize::min(i, limit);
            col = i - row;
        } else {
            col = usize::min(i, limit);
            row = i - col;
        }
        let mut j = 0;
        let index_limit = usize::min(i, (limit * 2) - i);
        while j <= index_limit {
            if i % 2 == 0 {
                buffer[buffer_index] = block[row - j][col + j];
            } else {
                buffer[buffer_index] = block[row + j][col - j];
            }
            buffer_index = buffer_index + 1;
            j = j + 1;
        }
        i = i + 1;
    }
}

fn generate_quantize_table_quality(table: [[ i32; 8]; 8], quality: i32, new_table: &mut [[ i32; 8]; 8]) {
    if quality < 50 {
        for i in 0..BLOCK_SIZE {
            for j in 0..BLOCK_SIZE {
                new_table[j][i] = table[i][j] * (50 / quality);
            }
        }
    } else {
        for i in 0..BLOCK_SIZE {
            for j in 0..BLOCK_SIZE {
                new_table[j][i] = table[i][j] * ((100 - quality) / 50);
            }
        }
    }
}

fn divide_matrix(matrix_a: &[[ f32; BLOCK_SIZE]; BLOCK_SIZE], matrix_b: &[[ i32; BLOCK_SIZE]; BLOCK_SIZE], matrix_buffer: &mut [[ i32; BLOCK_SIZE]; BLOCK_SIZE]) {
    for i in 0..BLOCK_SIZE {
        for j in 0..BLOCK_SIZE {
            matrix_buffer[i][j] = matrix_a[i][j] as i32 / matrix_b[i][j];
        }
    }
}

fn transform(vector: &[f32; BLOCK_SIZE]) -> [f32; BLOCK_SIZE] {
	// Algorithm by Arai, Agui, Nakajima, 1988. For details, see:
	// https://web.stanford.edu/class/ee398a/handouts/lectures/07-TransformCoding.pdf#page=30
    let mut output: [f32; 8] = [0f32; 8];
	let v0 = vector[0] + vector[7];
	let v1 = vector[1] + vector[6];
	let v2 = vector[2] + vector[5];
	let v3 = vector[3] + vector[4];
	let v4 = vector[3] - vector[4];
	let v5 = vector[2] - vector[5];
	let v6 = vector[1] - vector[6];
	let v7 = vector[0] - vector[7];
	
	let v8 = v0 + v3;
	let v9 = v1 + v2;
	let v10 = v1 - v2;
	let v11 = v0 - v3;
	let v12 = -v4 - v5;
	let v13 = (v5 + v6) * A[3];
	let v14 = v6 + v7;
	
	let v15 = v8 + v9;
	let v16 = v8 - v9;
	let v17 = (v10 + v11) * A[1];
	let v18 = (v12 + v14) * A[5];
	
	let v19 = -v12 * A[2] - v18;
	let v20 = v14 * A[4] - v18;
	
	let v21 = v17 + v11;
	let v22 = v11 - v17;
	let v23 = v13 + v7;
	let v24 = v7 - v13;
	
	let v25 = v19 + v24;
	let v26 = v23 + v20;
	let v27 = v23 - v20;
	let v28 = v24 - v19;
	
	output[0] = S[0] * v15;
	output[1] = S[1] * v26;
	output[2] = S[2] * v21;
	output[3] = S[3] * v28;
	output[4] = S[4] * v16;
	output[5] = S[5] * v25;
	output[6] = S[6] * v22;
	output[7] = S[7] * v27;

    output
}


use image::{self, GenericImageView, Pixel};
use ndarray::arr2;

/**
TODO (benjamintoofer@gmail.com)
Consider using ndarray library with blas for optimal vector operations
Record performance difference
*/
fn main() {
    println!("Hello, world!");
    let file = "./assets/image_1.jpg";

    let img = image::open(file).unwrap();
    println!("{:?}", img.dimensions());
    println!("{:?}", img.color());
    println!("{:?}", img.as_bytes().len());
    let pixel = img.get_pixel(0, 0);
    println!("PIXEL: {:?}", pixel.0);
    println!("PIXEL: {:?}", pixel.channels());
    let channel = pixel.channels();
    let ycrcb_vector = rgb_2_ycrcb(channel[0], channel[1], channel[2]);
    println!("ycrcb: {:?}", ycrcb_vector);
    println!("rgb: {:?}", ycrcb_2_rgb(ycrcb_vector[0], ycrcb_vector[1], ycrcb_vector[2]));
}

fn compress_jpeg_brute() {
    // Iterate through each pixel and convert from RGB -> YCrCb

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
    0
}

fn compress_jpeg_optimal() {

}

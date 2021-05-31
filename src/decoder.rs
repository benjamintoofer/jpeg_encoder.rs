// NOTE (benjamintoofer@gmail.com): Only support for baseline parsing
use std::convert::TryInto;

use crate::{jpeg::{FrameHeader, Header, HuffmanTable, ImageComponent, QuantizationTable}, markers::{MARKER_START, SOI, EOI, APP0, DQT, SOF0, DHT}};

#[derive(Debug)]
pub struct Decoder {

}

impl Decoder {
    /// Decode a jpeg image to generate a bmp
    pub fn decode(image_data: &[u8]) -> Result<u32, String> {
        let mut jpeg_header: Header = Header::new();
        let mut offset = 0usize;
        // Start by verifying that the SOI is the first byte pair
        let mut first_byte = image_data[offset];
        let mut second_byte = image_data[offset + 1];
        let mut length: u16 = 0;
        offset += 2;

        if first_byte != MARKER_START || second_byte != SOI {
            return Err("Invalid JPEG: missing SOI marker".to_string());
        }

        let (f, s, l) = parse_section_header(&image_data, offset);
        first_byte = f;
        second_byte = s;
        length = l;
        while second_byte != EOI {
            println!("SECOND MARKER = {:X}", second_byte);
            if first_byte != MARKER_START {
                return Err("Invalid JPEG: Incorrect marker start".to_string());
            }

            if second_byte == APP0 {
                read_app0();
            }

            if second_byte == DQT {
                let new_offset = offset + 2;
                let quantization_data = &image_data[new_offset..(new_offset + length as usize)];
                let qts = read_quantization_table(quantization_data)?;
                for qt in qts {
                     jpeg_header.qt[qt.table_id as usize] = Some(qt)
                }
            }

            if second_byte == SOF0 {
                let new_offset = offset + 2;
                let frame_header_data = &image_data[new_offset..(new_offset + length as usize)];
                let frame_header  = read_frame_header(&frame_header_data)?;
                jpeg_header.frame_header = Some(frame_header);
            }

            if second_byte == DHT {
                let new_offset = offset + 2;
                let huffman_table_data = &image_data[new_offset..(new_offset + length as usize)];
                read_huffman_table(&huffman_table_data)?;
            }

            offset = offset + length as usize + 2;
            let (f, s, l) = parse_section_header(&image_data, offset);
            first_byte = f;
            second_byte = s;
            length = l;
        }
        // println!("F: {}; S: {}; LEN: {}",first_byte, second_byte, length);
        println!("JPEG HEADER {}", jpeg_header);
        Ok(0u32)
    }
}

fn parse_section_header(image_data: &[u8], offset: usize) -> (u8, u8, u16) {
    let marker_start = image_data[offset];
    let marker = image_data[offset + 1];
    let length: u16 = ((image_data[offset + 2] as u16) << 8) + image_data[offset + 3] as u16;
    return (marker_start, marker, length);
}

fn read_app0() {
    println!("READ ================> APP0");
}

fn read_quantization_table<'a>(quantization_data: &'a[u8]) -> Result<Vec<QuantizationTable>, String>{
    println!("READ ================> DQT");
    let mut table_datas: Vec<QuantizationTable> = vec![];
    let mut offset = 2usize;
    while offset < quantization_data.len() {
        let precision_and_id = quantization_data[offset];
        if (precision_and_id & 0xF0) != 0 {
            return Err("read_quantization_table: Unsupported JPEG: Only support 8 bit precision, not 16".to_string());
        }
        let table_id = precision_and_id & 0xF;
        offset = offset + 1;
        let slice_qd = &quantization_data[offset..(offset + 64)];
        let table_data: &[u8; 64] = slice_qd.try_into().expect("slice with incorrect length");
        table_datas.push(QuantizationTable{table_id, table_data});
        offset += 64;
    }   
    
    Ok(table_datas)
}

fn read_frame_header<'a>(frame_header_data: &'a[u8]) -> Result<FrameHeader, String> { 
    println!("READ ================> SOF0");
    let mut offset = 2usize;
    if frame_header_data[offset] != 8 {
        return Err("read_frame_header: Unsupported JPEG: Only support 8 bit precision, not 16".to_string());
    }
    offset += 1;
    let num_of_lines: u16 = ((frame_header_data[offset] as u16) << 8) + frame_header_data[offset + 1] as u16;

    offset += 2;
    let num_of_samples_per_line: u16 = ((frame_header_data[offset] as u16) << 8) + frame_header_data[offset + 1] as u16;

    offset += 2;
    let num_of_image_components = frame_header_data[offset];
    offset += 1;

    let mut image_components = vec![];
    for _ in 0..num_of_image_components as usize {
        let image_comp= ImageComponent {
            component_id: frame_header_data[offset],
            horz_sample_factor: (frame_header_data[offset + 1] & 0xF0) >> 4,
            vert_sample_factor: frame_header_data[offset + 1] & 0xF,
            qt_id:frame_header_data[offset + 2],
        };
        image_components.push(image_comp);
        offset += 3;
    }
    Ok(FrameHeader {
        num_of_lines,
        num_of_samples_per_line,
        image_components,
    })
}

fn read_huffman_table(huffman_data: &[u8]) -> Result<u32, String> {
    println!("READ ================> DHT");
    let mut offset = 2usize;
    let table_class = (huffman_data[offset] & 0xF0) >> 4;
    let huffman_id = huffman_data[offset] & 0xF;
    println!("TABLE CLASS: {}; HUFFMAN ID: {}", table_class, huffman_id);

    offset += 1;
    let num_of_huffman = huffman_data[offset];
    println!("NUM OF HUFF: {}", num_of_huffman);
    Ok(0)
}

fn read_scan_header(scan_data: &[u8])-> Result<u32, String> {

    Ok(0)
}
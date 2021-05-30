use std::{fmt::{self, Display}, u16};

// const zig_zag_map: [usize; 64] = [
//     0, 1, 8, 16, 9, 2, 3, 10,
//     17, 32, 40, 33, 18, 11, 4, 5,
//     12, 19, 
// ]

#[derive(Clone, Copy, Debug)]
pub struct QuantizationTable<'a> {
    pub table_id: u8,
    pub table_data: &'a [u8]
}

impl fmt::Display for QuantizationTable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut matrix_str = String::from("[\n");
        let mut num_vec: Vec<String> = vec!["".to_string(); 8];
        for i in 0..64  {
            let col = i % 8;
            if col == 0 {
                matrix_str.push_str("  [");
            }
            
            num_vec[col as usize] = self.table_data[i].to_string();
            if col == 7 {
                matrix_str.push_str(format!("{}]\n", num_vec.join(", ")).as_str());
            }
            
        }
        matrix_str.push_str("\n]");
        write!(f, "Table ID: {}\n{}",self.table_id, matrix_str)
    }
}

#[derive(Debug)]
pub struct FrameHeader {
    pub num_of_lines: u16,
    pub num_of_samples_per_line: u16,
    pub image_components: Vec<ImageComponent>,
}

#[derive(Debug)]
pub struct ImageComponent {
    pub component_id: u8,
    pub horz_sample_factor: u8,
    pub vert_sample_factor: u8,
    pub qt_id: u8,
}

pub struct HuffmanTable<'a> {
    pub table_id: u8,
    pub table_data: &'a [u8]
}

#[derive(Debug)]
pub struct Header<'a> {
    pub frame_header: Option<FrameHeader>,
    pub qt: [Option<QuantizationTable<'a>>; 4]
}

impl<'a> Header<'a> {
    pub fn new() -> Self {
        Header{
            qt: [None; 4],
            frame_header:  None
            // qt: Default::default()
        }
    }
}

impl fmt::Display for Header<'_>  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut qt_str = String::from("");
        for qt in self.qt.iter() {
            match qt {
                Some(table) => {
                    qt_str.push_str(format!("{}\n", table).as_str())
                }
                None => {}
            }
        }

        write!(f, "HEADER\nFrame Header\n{:?}\nQuantization Tables\n{}",self.frame_header.as_ref().unwrap(), qt_str)
    }
}
pub struct JEPGWriter {

}
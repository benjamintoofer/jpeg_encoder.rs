use std::{fmt::{self, Display}, u16};

// const zig_zag_map: [usize; 64] = [
//     0, 1, 8, 16, 9, 2, 3, 10,
//     17, 32, 40, 33, 18, 11, 4, 5,
//     12, 19, 
// ]

#[derive(Debug, Clone, Copy)]
pub struct QuantizationTable<'a> {
    pub table_id: u8,
    pub table_data: &'a [u8; 64]
}

impl<'a> fmt::Display for QuantizationTable<'a> {
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

#[derive(Debug, Clone, Copy)]
pub struct HuffmanTable<'a> {
    pub table_class: u8, // DC (1) or AC (0) huffman table
    pub table_id: u8,
    pub offsets: &'a [u8; 16],
    pub symbols: &'a [u8],
}

impl<'a> HuffmanTable<'a> {
    pub fn create() -> HuffmanTable<'a> {
        HuffmanTable {
            table_class: 0,
            table_id: 0,
            offsets: &[0; 16],
            symbols: &[0; 162],
        }
    }
}

impl fmt::Display for HuffmanTable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut huffman_str = String::from("");
        let mut start: usize = 0;
        let mut offsetTotal = 0;
        for offset in self.offsets.iter() {
            offsetTotal += *offset as usize;
            let end = offsetTotal;
            
            for symbol in self.symbols[start..end].iter() {
                huffman_str.push_str(format!("{:02x} ", symbol).as_str());
            }
            huffman_str.push_str("\n");
            start = end;
        }
        write!(f, "Table ID: {}\nSymbols:{}", self.table_id, huffman_str)
    }
}
#[derive(Debug)]
pub struct Header<'a> {
    pub frame_header: Option<FrameHeader>,
    pub quant_tables: [Option<QuantizationTable<'a>>; 4],
    pub ac_huffman_tables: [Option<HuffmanTable<'a>>; 4],
    pub dc_huffman_tables: [Option<HuffmanTable<'a>>; 4],
}

impl<'a> Header<'a> {
    pub fn new() -> Self {
        Header{
            frame_header:  None,
            ac_huffman_tables: [None; 4],
            quant_tables: [None; 4],
            dc_huffman_tables: [None; 4],
        }
    }
}

impl fmt::Display for Header<'_>  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut qt_str = String::from("");
        // Quantization Tables
        for qt in self.quant_tables.iter() {
            match qt {
                Some(table) => {
                    qt_str.push_str(format!("{}\n", table).as_str())
                }
                None => {}
            }
        }

        // DC Huffman Tables
        let mut dht_str = String::from("");
        for dht in self.dc_huffman_tables.iter() {
            match dht {
                Some(table) => {
                    dht_str.push_str(format!("{}\n", table).as_str())
                }
                None => {}
            }
        }

        // AC Huffman Tables
        let mut aht_str = String::from("");
        for aht in self.ac_huffman_tables.iter() {
            match aht {
                Some(table) => {
                    aht_str.push_str(format!("{}\n", table).as_str())
                }
                None => {}
            }
        }

        write!(f, "HEADER\nFrame Header\n{:?}\nQuantization Tables\n{}\nDC Huffman Tables\n{}\nAC Huffman Tables\n{}\n",
            self.frame_header.as_ref().unwrap(),
            qt_str,
            dht_str,
            aht_str,
        )
    }
}
pub struct JEPGWriter {

}
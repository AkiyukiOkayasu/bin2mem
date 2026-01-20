use std::env;
use std::fs::File;
use std::io::{self, Read, Write, BufWriter};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input.bin> <output.hex>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let input_file = File::open(input_path)?;
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);
    let mut reader = io::BufReader::new(input_file);

    convert(&mut reader, &mut writer)?;

    println!("Successfully converted {} to {}", input_path, output_path);
    Ok(())
}

fn convert<R: Read, W: Write>(input: &mut R, output: &mut W) -> io::Result<()> {
    let mut buffer = [0u8; 4];

    loop {
        let mut chunk_len = 0;
        while chunk_len < 4 {
            let n = input.read(&mut buffer[chunk_len..])?;
            if n == 0 {
                break;
            }
            chunk_len += n;
        }

        if chunk_len == 0 {
            break;
        }

        // Pad with zeros if necessary
        if chunk_len < 4 {
            for i in chunk_len..4 {
                buffer[i] = 0;
            }
        }

        // Unpack as little-endian unsigned integer
        let val = u32::from_le_bytes(buffer);
        
        // Write as 8-digit hex string
        writeln!(output, "{:08x}", val)?;

        if chunk_len < 4 {
            // We reached EOF inside the chunk
            break;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_convert_exact_multiple() {
        // 8 bytes (2 words)
        let input_data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let mut input = Cursor::new(input_data);
        let mut output = Vec::new();

        convert(&mut input, &mut output).unwrap();

        let output_str = String::from_utf8(output).unwrap();
        // 0x04030201 -> 04030201
        // 0x08070605 -> 08070605
        let expected = "04030201\n08070605\n";
        assert_eq!(output_str, expected);
    }

    #[test]
    fn test_convert_padding() {
        // 5 bytes (1 word + 1 byte)
        let input_data = vec![0x01, 0x02, 0x03, 0x04, 0xAA];
        let mut input = Cursor::new(input_data);
        let mut output = Vec::new();

        convert(&mut input, &mut output).unwrap();

        let output_str = String::from_utf8(output).unwrap();
        // 0x04030201 -> 04030201
        // 0x000000AA -> 000000aa
        let expected = "04030201\n000000aa\n";
        assert_eq!(output_str, expected);
    }

    #[test]
    fn test_convert_empty() {
        let input_data = vec![];
        let mut input = Cursor::new(input_data);
        let mut output = Vec::new();

        convert(&mut input, &mut output).unwrap();

        let output_str = String::from_utf8(output).unwrap();
        let expected = "";
        assert_eq!(output_str, expected);
    }
}


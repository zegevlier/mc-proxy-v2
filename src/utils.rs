// This converts a long string into one that's shortened.
// alongstringlikethis would become alongs...kethis
pub fn make_string_fixed_length(string: String, length: usize) -> String {
    if string.len() <= length {
        return string;
    } else {
        let part_size = length - 3 / 2;
        format!(
            "{}...{}",
            string[0..part_size].to_string(),
            string[string.len() - part_size..].to_string()
        )
    }
}

// pub fn calc_hash(name: &str) -> String {
//     let mut hasher = Sha1::new();
//     hasher.input_str(name);
//     let mut hex: Vec<u8> = iter::repeat(0)
//         .take((hasher.output_bits() + 7) / 8)
//         .collect();
//     hasher.result(&mut hex);

//     let negative = (hex[0] & 0x80) == 0x80;

//     let regex = Regex::new(LEADING_ZERO_REGEX).unwrap();

//     if negative {
//         two_complement(&mut hex);
//         format!(
//             "-{}",
//             regex
//                 .replace(hex.as_slice().to_hex().as_str(), "")
//                 .to_string()
//         )
//     } else {
//         regex
//             .replace(hex.as_slice().to_hex().as_str(), "")
//             .to_string()
//     }
// }

// fn two_complement(bytes: &mut Vec<u8>) {
//     let mut carry = true;
//     for i in (0..bytes.len()).rev() {
//         bytes[i] = !bytes[i] & 0xff;
//         if carry {
//             carry = bytes[i] == 0xff;
//             bytes[i] = bytes[i] + 1;
//         }
//     }
// }

// mod tests {
//     use super::calc_hash;

//     #[test]
//     pub fn calc_hashes() {
//         assert_eq!(
//             "-7c9d5b0044c130109a5d7b5fb5c317c02b4e28c1",
//             calc_hash("jeb_")
//         );
//         assert_eq!(
//             "4ed1f46bbe04bc756bcb17c0c7ce3e4632f06a48",
//             calc_hash("Notch")
//         );
//         assert_eq!(
//             "88e16a1019277b15d58faf0541e11910eb756f6",
//             calc_hash("simon")
//         );
//     }
// }

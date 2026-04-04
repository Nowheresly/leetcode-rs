
pub struct Solution {}

impl Solution {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        if encoded_text.is_empty() {
            return "".to_string();
        }
        let n = encoded_text.len();
        let rows = rows as usize;
        let cols = n / rows;
        let bytes = encoded_text.as_bytes(); // Work with bytes for O(1) indexing
        let mut res = Vec::with_capacity(n);

        for i in 0..cols {
            let mut row = 0;
            let mut col = i;

            // Move diagonally: down 1 row, right 1 column
            // Stop if we hit the bottom OR the right edge of the matrix
            while row < rows && col < cols {
                res.push(bytes[row * cols + col]);
                row += 1;
                col += 1;
            }
        }
        let mut decoded = String::from_utf8(res).unwrap();

        // Use the built-in trim_end to handle trailing whitespace
        decoded.truncate(decoded.trim_end().len());

        decoded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {

        assert_eq!(String::from("i love leetcode"), Solution::decode_ciphertext(String::from("iveo    eed   l te   olc"), 4));

    }
}

pub fn wc_of_string(text: &str) -> usize {
    text.len()
}

pub fn wc(filename: &str) -> usize {
    let content = std::fs::read_to_string(filename).unwrap();
    wc_of_string(&content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wc_of_string_works() {
        let filename = "files/hello.txt";
        let content = std::fs::read_to_string(filename).unwrap();
        let result = wc_of_string(&content);
        assert_eq!(result, 11);
    }

    #[test]
    fn wc_works() {
        let cases = [
            ("files/hello.txt", 11),
            ("files/empty.txt", 0),
            ("files/hello_with_newline.txt", 12),
            ("files/spaces.txt", 47),
            ("files/unicode.txt", 12),
        ];

        for (filename, bytes) in cases {
            let result = wc(&filename);
            assert_eq!(result, bytes);
        }
    }
}

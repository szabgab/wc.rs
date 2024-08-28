pub fn wc_of_string(text: &str) -> (usize, usize) {
    let words = text.split_whitespace().collect::<Vec<_>>().len();
    (text.len(), words)
}

pub fn wc(filename: &str) -> (usize, usize) {
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
        assert_eq!(result, (11, 2));
    }

    #[test]
    fn wc_works() {
        let cases = [
            ("files/hello.txt", (11, 2)),
            ("files/empty.txt", (0, 0)),
            ("files/hello_with_newline.txt", (12, 2)),
            ("files/spaces.txt", (47, 7)),
            ("files/unicode.txt", (12, 2)),
        ];

        for (filename, bytes) in cases {
            let result = wc(&filename);
            assert_eq!(result, bytes);
        }
    }
}

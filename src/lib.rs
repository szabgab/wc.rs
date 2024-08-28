pub fn wc_of_string(text: &str) -> (usize, usize, usize) {
    let bytes = text.len();
    let words = text.split_whitespace().collect::<Vec<_>>().len();
    let lines = text.split('\n').collect::<Vec<_>>().len();

    (bytes, words, lines - 1)
}

pub fn wc(filename: &str) -> (usize, usize, usize) {
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
        assert_eq!(result, (11, 2, 0));
    }

    #[test]
    fn wc_works() {
        let cases = [
            ("files/hello.txt", (11, 2, 0)),
            ("files/empty.txt", (0, 0, 0)),
            ("files/hello_with_newline.txt", (12, 2, 1)),
            ("files/spaces.txt", (47, 7, 3)),
            ("files/unicode.txt", (12, 2, 1)),
        ];

        for (filename, bytes) in cases {
            let result = wc(&filename);
            assert_eq!(result, bytes);
        }
    }
}

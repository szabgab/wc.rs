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
        let filename = "files/hello.txt";
        let result = wc(&filename);
        assert_eq!(result, 11);
    }
}

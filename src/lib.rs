
pub fn wc(text: &str) -> usize {
    text.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let filename = "files/hello.txt";
        let content = std::fs::read_to_string(filename).unwrap();
        let result = wc(&content);
        assert_eq!(result, 11);
    }
}

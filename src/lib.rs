
pub fn wc(text: &str) -> usize {
    text.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = wc("Hello world");
        assert_eq!(result, 11);
    }
}

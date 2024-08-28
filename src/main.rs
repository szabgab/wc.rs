fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    //println!("{args:?}");
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    for filename in args[1..].iter() {
        //println!("{filename}");
        let (bytes, words, lines) = wc::wc(filename);
        println!("{lines} {words} {bytes} {filename}");
        total_lines += lines;
        total_words += words;
        total_bytes += bytes;
    }
    if args.len() > 2 {
        println!("{total_lines} {total_words} {total_bytes} total");
    }
}

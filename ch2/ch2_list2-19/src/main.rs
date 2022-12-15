fn main() {
    let search_term = "picture";
    let quote = "x\
Every face, every shop bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what
it is th same books;
What do we seek through millions of pages?";
    let mut line_num: usize = 1;

    for line in quote.lines() {
        if line.contains(search_term) {
            println!("{}, {}", search_term, line_num)
        }
        line_num += 1;
    }
}

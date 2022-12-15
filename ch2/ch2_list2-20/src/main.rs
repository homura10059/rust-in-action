fn main() {
    let search_term = "picture";
    let quote = "x\
Every face, every shop bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what
it is th same books;
What do we seek through millions of pages?";

    for (i, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            let line_num = i + 1;
            println!("{}, {}", search_term, line_num)
        }
    }
}

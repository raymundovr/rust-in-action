use regex::Regex;

fn main() {
    // PARAMETERS
    let re = Regex::new("oo").unwrap();
    let haystack = "Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";
    for line in haystack.lines() {
        match re.find(line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

use crate::file_io::*;

#[test]
fn single_quote_file() {
    let path = "testing-data/test1.md";
    let quotes = read_markdown(path);
    assert_eq!(vec!["File with a single quote"], quotes);
}

#[test]
fn no_quote_file() {
    let path = "testing-data/test2.md";
    let quotes = read_markdown(path);
    assert_eq!(vec![""], quotes);
}

#[test]
fn multi_quote_file() {
    let path = "testing-data/test3.md";
    let quotes = read_markdown(path);
    assert_eq!(vec!["Then there's a pair of us - don't tell!\nThey'd banish us, you know.", "To tell your name the livelong day\nTo an admiring bog!"], quotes);
}

#[test]
fn complex_quote_file() {
    let path = "testing-data/test4.md";
    let quotes = read_markdown(path);
    assert_eq!(vec!["This is a blockquote with two paragraphs. Lorem ipsum dolor sit amet,\nconsectetuer adipiscing elit. Aliquam hendrerit mi posuere lectus.\nVestibulum enim wisi, viverra nec, fringilla in, laoreet vitae, risus.\n\nDonec sit amet nisl. Aliquam semper ipsum sit amet velit. Suspendisse\nid sem consectetuer libero luctus adipiscing.", "This is a blockquote with two paragraphs. Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aliquam hendrerit mi posuere lectus. Vestibulum enim wisi, viverra nec, fringilla in, laoreet vitae, risus.", "Donec sit amet nisl. Aliquam semper ipsum sit amet velit. Suspendisse id sem consectetuer libero luctus adipiscing.", ""], quotes);
}

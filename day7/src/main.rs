use std::fs;

fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("./data").expect("failed to read file");

    // rough idea
    // accumulate every time ls is called
}

fn get_content_size(input: &str) -> usize {
    input.split_terminator("\n").fold(0, |accumulator, line| {
        accumulator + line.split_once(" ").unwrap().0.parse::<usize>().unwrap()
    })
}

#[test]
fn can_determine_size_of_single_file_directory() {
    // no recursiveness yet

    let input = "584 i\n";
    let got = get_content_size(input);
    assert_eq!(584, got)
}

#[test]
fn can_determine_size_of_multi_file_directory() {
    // no recursiveness yet
    let input = "4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    let got = get_content_size(input);
    assert_eq!(24933642, got)
}

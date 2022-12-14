use std::fs;

fn main() {
    let contents = fs::read_to_string("./data").expect("failed to read file");

    println!("start of packet {}", find_start_of_packet(&contents));
    println!("start of message {}", find_start_of_message(&contents));
}

fn find_start_of_packet(datastream: &str) -> usize {
    let stream: Vec<char> = datastream.chars().collect();

    let mut start = 0;

    for i in 4..stream.len() - 1 {
        let prev: Vec<char> = stream[i - 4..i].into();

        if prev.iter().enumerate().all(|(i, c)| {
            let mut others = prev.clone();
            others.swap_remove(i);

            !others.contains(c)
        }) {
            start = i;
            break;
        }
    }

    start
}

fn find_start_of_message(datastream: &str) -> usize {
    let stream: Vec<char> = datastream.chars().collect();

    let mut start = 0;

    for i in 14..stream.len() - 1 {
        let prev: Vec<char> = stream[i - 14..i].into();

        if prev.iter().enumerate().all(|(i, c)| {
            let mut others = prev.clone();
            others.swap_remove(i);

            !others.contains(c)
        }) {
            start = i;
            break;
        }
    }

    start
}

#[test]
fn start_of_packet_finder_test() {
    let test_cases = vec![
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    test_cases
        .iter()
        .for_each(|tc| assert_eq!(find_start_of_packet(tc.0), tc.1))
}

#[test]
fn start_of_message_finder_test() {
    let test_cases = vec![
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
    ];

    test_cases
        .iter()
        .for_each(|tc| assert_eq!(find_start_of_message(tc.0), tc.1))
}

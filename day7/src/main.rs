fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
enum Command{
    CD(String),
        LS,
}

#[derive(Debug)]
struct ErrUnknownCommand;

fn parse_command(input: &str) -> Result<Command,ErrUnknownCommand>{
    let cmd = &input[2..5];

    let arg = &input[5..];
    match cmd {
   "cd" => Ok(Command::CD(arg.to_string())),
 "ls"=> Ok(Command::LS),
  _ => Err(ErrUnknownCommand)
    }
}

#[test]
fn parse_commands() {
    assert_eq!(parse_command("$ cd /").unwrap(),Command::CD("/".to_owned()) );
    assert_eq!(parse_command("$ ls").unwrap(),Command::LS)

}

use std::fs;

fn main() {
    println!("Hello, world!");
}
#[derive(Debug)]
enum Instruction {
    AddX(isize),
    Noop,
}

struct CPU {
    cycle: usize,
    register: isize,
}

impl CPU {
    fn process(&mut self, instr: Instruction) {
        self.cycle += 1;

        match instr {
            Instruction::AddX(v) => self.register += v,
            Instruction::Noop => {}
        }
    }
}

#[test]
fn instructions_test() {
    let mut cpu = CPU {
        cycle: 0,
        register: 1,
    };

    cpu.process(Instruction::Noop);
    assert_eq!(cpu.cycle, 1);

    assert_eq!(cpu.register, 1);
    cpu.process(Instruction::AddX(15));
    assert_eq!(cpu.register, 16);
}

#[test]
fn test_case() {
    let contents = fs::read_to_string("./testdata").expect("failed to read file");

    let mut cpu = CPU {
        cycle: 1,
        register: 1,
    };

    let mut strengths = vec![];
    contents
        .lines()
        .map(|line| match line[..4].as_ref() {
            "noop" => Instruction::Noop,
            "addx" => Instruction::AddX(line[5..].parse::<isize>().unwrap()),
            _ => Instruction::Noop,
        })
        .for_each(|instr| {
            println!(
                "instruction: {:?}, cycle:{}, register: {}",
                instr, cpu.cycle, cpu.register
            );

            if cpu.register == 420 {
                println!("hi");
                strengths.push(cpu.register);
            }
            cpu.process(instr);
        });

    assert_eq!(strengths[0], 420);
}

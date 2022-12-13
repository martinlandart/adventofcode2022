use std::{collections::HashSet, fs};

fn main() {
    let contents = fs::read_to_string("./data").expect("failed to read file");

    println!("total {}", get_cycle_totals(&contents));
}
#[derive(Debug)]
enum Instruction {
    AddX(isize),
    Noop,
}

enum Subroutine {
    Sleep,
    AddX(isize),
}

struct CPU {
    cycle: usize,
    register: isize,
    stack: Vec<Subroutine>,
}

impl Iterator for CPU {
    type Item = (usize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        self.cycle += 1;

        if let Some(sr) = self.stack.pop() {
            match sr {
                Subroutine::AddX(v) => self.register += v,
                Subroutine::Sleep => {}
            }
            Some((self.cycle, self.register))
        } else {
            None
        }
    }
}

impl CPU {
    fn push_instr(&mut self, instr: &Instruction) {
        match instr {
            Instruction::AddX(v) => {
                self.stack.push(Subroutine::AddX(*v));
                self.stack.push(Subroutine::Sleep);
            }
            Instruction::Noop => self.stack.push(Subroutine::Sleep),
        }
    }

    // fn process(&mut self, sr: &Subroutine) {
    //     self.cycle += 1;
    //
    //     // if let Some(sr) = self.stack.pop() {
    //     match sr {
    //         Subroutine::AddX(v) => self.register += v,
    //         Subroutine::Sleep => {}
    //     }
    //     // };
    // }
}

#[test]
fn instructions_test() {
    let mut cpu = CPU {
        cycle: 0,
        register: 1,
        stack: vec![],
    };

    assert_eq!(cpu.register, 1);
    cpu.push_instr(&Instruction::Noop);
    cpu.next();
    assert_eq!(cpu.cycle, 1);
    assert_eq!(cpu.register, 1);

    cpu.push_instr(&Instruction::AddX(15));

    cpu.next();
    assert_eq!(cpu.cycle, 2);
    assert_eq!(cpu.register, 1);

    cpu.next();
    assert_eq!(cpu.cycle, 3);
    assert_eq!(cpu.register, 16);
}

fn get_cycle_totals(input: &str) -> isize {
    let mut cpu = CPU {
        cycle: 1,
        register: 1,
        stack: vec![],
    };
    let instructions = input
        .lines()
        .map(|line| match line[..4].as_ref() {
            "noop" => Instruction::Noop,
            "addx" => Instruction::AddX(line[5..].parse::<isize>().unwrap()),
            _ => Instruction::Noop,
        })
        .rev()
        .collect::<Vec<Instruction>>();

    instructions.iter().for_each(|i| cpu.push_instr(i));

    let mut cycles: HashSet<usize> = HashSet::new();
    cycles.insert(20);
    cycles.insert(60);
    cycles.insert(100);
    cycles.insert(140);
    cycles.insert(180);
    cycles.insert(220);

    let strengths = cpu.into_iter().fold(vec![], |mut accum: Vec<isize>, item| {
        if cycles.contains(&item.0) {
            accum.push(item.1 * item.0 as isize);
        }
        accum
    });

    let mut total = 0;
    for s in strengths {
        total += s;
    }

    total
}

#[test]
fn test_case() {
    let contents = fs::read_to_string("./testdata").expect("failed to read file");
    let total = get_cycle_totals(&contents);
    assert_eq!(total, 13140);
}

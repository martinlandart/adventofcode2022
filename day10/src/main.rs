use std::{collections::HashSet, fs};

fn main() {
    let contents = fs::read_to_string("./data").expect("failed to read file");

    println!("total {}", get_cycle_totals(&contents));

    let mut cpu = CPU::new();
    cpu.load_instructions(&contents);

    loop {
        if let None = cpu.next() {
            break;
        }
    }

    println!("{}", cpu.render());
}

#[derive(Debug)]
enum Instruction {
    AddX(isize),
    Noop,
}

#[derive(Debug)]
enum Subroutine {
    Sleep,
    AddX(isize),
}

#[derive(Debug)]
struct CPU {
    cycle: usize,
    register: isize,
    stack: Vec<Subroutine>,
    lit: HashSet<usize>,
}

impl Iterator for CPU {
    type Item = (usize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        let position = self.cycle - 1;

        if is_lit(position.try_into().unwrap(), self.register) {
            self.lit.insert(self.cycle);
        }

        println!(
            "cycle {}, position {}, register {}, lit {}",
            self.cycle,
            position,
            self.register,
            is_lit(position.try_into().unwrap(), self.register)
        );

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
    fn new() -> CPU {
        CPU {
            cycle: 1,
            register: 1,
            stack: vec![],
            lit: HashSet::new(),
        }
    }

    fn push_instr(&mut self, instr: &Instruction) {
        match instr {
            Instruction::AddX(v) => {
                self.stack.push(Subroutine::AddX(*v));
                self.stack.push(Subroutine::Sleep);
            }
            Instruction::Noop => self.stack.push(Subroutine::Sleep),
        }
    }

    fn load_instructions(&mut self, input: &str) {
        let instructions = input
            .lines()
            .map(|line| match line[..4].as_ref() {
                "noop" => Instruction::Noop,
                "addx" => Instruction::AddX(line[5..].parse::<isize>().unwrap()),
                _ => Instruction::Noop,
            })
            .rev()
            .collect::<Vec<Instruction>>();

        instructions.iter().for_each(|i| self.push_instr(i));
    }

    fn render(&self) -> String {
        let mut pixels: Vec<char> = vec![];
        println!(
            "rendering to cycle {}. lit contains {}",
            self.cycle,
            self.lit.len()
        );
        for i in 1..self.cycle - 1 {
            if self.lit.contains(&i) {
                pixels.push('#');
            } else {
                pixels.push('.');
            }

            if i % 40 == 0 && i != self.cycle - 2 {
                pixels.push('\n');
            }
        }

        pixels.iter().collect()
    }
}

fn get_cycle_totals(input: &str) -> isize {
    let mut cpu = CPU::new();

    cpu.load_instructions(&input);

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

fn is_lit(position: isize, register: isize) -> bool {
    if register < 0 {
        return false;
    }

    let mut p = position;
    let mut r = register;

    if (r - p < 2) && (r == p || r == p + 1 || r == p - 1) {
        return true;
    }

    if register == 39 {
        r = -1;
    }

    if position == 39 {
        p = -1;
    }

    if position >= 40 {
        p = position % 40;
    }

    if register >= 40 {
        r = r % 40;
    }

    if r == p || r == p + 1 || r == p - 1 {
        return true;
    } else {
        return false;
    }
}

#[test]
fn is_lit_test() {
    assert_eq!(is_lit(0, 1), true);
    assert_eq!(is_lit(0, 0), true);
    assert_eq!(is_lit(0, 2), false);

    assert_eq!(is_lit(39, 38), true);
    assert_eq!(is_lit(39, 39), true);
    assert_eq!(is_lit(39, 40), true);

    assert_eq!(is_lit(0, 39), true);
    assert_eq!(is_lit(0, 40), true);
}

#[test]
fn render_crt_test() {
    let input = fs::read_to_string("./testdata").expect("failed to read file");

    let mut cpu = CPU::new();
    cpu.load_instructions(&input);

    loop {
        if let None = cpu.next() {
            break;
        }
    }

    let got: HashSet<usize> = cpu.lit.clone();

    assert_eq!(got.contains(&1), true);
    assert_eq!(got.contains(&2), true);
    assert_eq!(got.contains(&3), false);
    assert_eq!(got.contains(&4), false);
    assert_eq!(got.contains(&5), true);
    assert_eq!(got.contains(&6), true);
    assert_eq!(got.contains(&7), false);
    assert_eq!(got.contains(&8), false);
    assert_eq!(got.contains(&9), true);
    assert_eq!(got.contains(&10), true);
    assert_eq!(got.contains(&41), true);
    assert_eq!(
        &cpu.render(),
        "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
    );
}

#[test]
fn test_case() {
    let contents = fs::read_to_string("./testdata").expect("failed to read file");
    let total = get_cycle_totals(&contents);
    assert_eq!(total, 13140);
}

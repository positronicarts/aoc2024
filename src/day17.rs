use std::collections::HashMap;

pub struct Day17;

#[derive(Debug)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

// 2,4,        1,2,            7,5,            1,3,              4,3,            5,5,      0,3,               3,0
// Bst(B=A%8), Bxl(B=B XOR 2), Cdv(C = A/2^B), Bxl(B = B XOR 3), Bxc(B=B XOR C), Out(5=B), Adv (A = A / 2^3), Jnz(if A!=0, repeat)

impl OpCode {
    fn from_str(d: i64) -> OpCode {
        match d {
            0 => OpCode::Adv,
            1 => OpCode::Bxl,
            2 => OpCode::Bst,
            3 => OpCode::Jnz,
            4 => OpCode::Bxc,
            5 => OpCode::Out,
            6 => OpCode::Bdv,
            7 => OpCode::Cdv,
            _ => panic!("Unknown OpCode: {}", d),
        }
    }
}

#[derive(Debug, Clone)]
struct Computer {
    registers: HashMap<char, i64>,
    pointer: usize,
    programme: Vec<i64>,
}

impl Computer {
    fn run(&mut self, term_on_mismatch: bool) -> Vec<i64> {
        let mut output = Vec::new();
        loop {
            if self.pointer >= self.programme.len() {
                break;
            }

            if term_on_mismatch && self.programme[..output.len()] != output {
                break;
            }

            let op = OpCode::from_str(self.programme[self.pointer]);
            self.pointer += 1;
            let operand = self.programme[self.pointer];
            self.pointer += 1;

            let actual_operand = match operand {
                0 | 1 | 2 | 3 => operand,
                4 => *self.registers.get(&'A').unwrap(),
                5 => *self.registers.get(&'B').unwrap(),
                6 => *self.registers.get(&'C').unwrap(),
                _ => panic!("Unknown operand: {}", operand),
            };

            match op {
                OpCode::Adv => {
                    self.registers.insert(
                        'A',
                        self.registers.get(&'A').unwrap() / 2_i64.pow(actual_operand as u32),
                    );
                }
                OpCode::Bxl => {
                    self.registers
                        .insert('B', self.registers.get(&'B').unwrap() ^ actual_operand);
                }
                OpCode::Bst => {
                    self.registers.insert('B', actual_operand % 8);
                }
                OpCode::Jnz => {
                    if *self.registers.get(&'A').unwrap() != 0 {
                        self.pointer = actual_operand as usize;
                    }
                }
                OpCode::Bxc => {
                    self.registers.insert(
                        'B',
                        self.registers.get(&'B').unwrap() ^ self.registers.get(&'C').unwrap(),
                    );
                }
                OpCode::Out => {
                    output.push(actual_operand % 8);
                }
                OpCode::Bdv => {
                    self.registers.insert(
                        'B',
                        self.registers.get(&'A').unwrap() / 2_i64.pow(actual_operand as u32),
                    );
                }
                OpCode::Cdv => {
                    self.registers.insert(
                        'C',
                        self.registers.get(&'A').unwrap() / 2_i64.pow(actual_operand as u32),
                    );
                }
            }
        }
        output
    }
}

impl aoc24::DayInner<Day17, String> for Day17 {
    fn day(&self) -> i32 {
        17
    }

    fn inner(&self, input: String) -> (String, String) {
        // Read data - make sure we have a blank line at the end to check the final entries.
        let lines: Vec<&str> = input.lines().collect();
        let programme = lines[4]
            .split("Program: ")
            .nth(1)
            .expect("ugh")
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let mut p1_computer = Computer {
            registers: HashMap::new(),
            pointer: 0,
            programme,
        };

        p1_computer.registers.insert(
            'A',
            lines[0]
                .split("Register A: ")
                .nth(1)
                .expect("ugh")
                .parse::<i64>()
                .unwrap(),
        );
        p1_computer.registers.insert(
            'B',
            lines[1]
                .split("Register B: ")
                .nth(1)
                .expect("ugh")
                .parse::<i64>()
                .unwrap(),
        );
        p1_computer.registers.insert(
            'C',
            lines[2]
                .split("Register C: ")
                .nth(1)
                .expect("ugh")
                .parse::<i64>()
                .unwrap(),
        );

        let p2_computer = p1_computer.clone();

        let p1_output = p1_computer.run(false);
        let p1_output_string_tmp: String = p1_output.iter().map(|d| d.to_string() + ",").collect();
        let p1_output_string = p1_output_string_tmp[..p1_output_string_tmp.len() - 1].to_string();

        let mut test_a_reg = 1;
        let mut matches = 0;

        loop {
            let (matched, test_output) = try_with_val(&p2_computer, test_a_reg);
            if matched {
                break;
            }

            let mut these_matches = 0;
            for ii in 0..test_output.len() {
                if ii >= p2_computer.programme.len() {
                    break;
                }
                if test_output[test_output.len() - 1 - ii]
                    == p2_computer.programme[p2_computer.programme.len() - 1 - ii]
                {
                    these_matches += 1;
                } else {
                    break;
                }
            }

            if these_matches > matches {
                matches = these_matches;
                println!(
                    "{}: {:?} ({}) -> {:?} ({}) {test_a_reg:b}",
                    test_a_reg,
                    test_output,
                    test_output.len(),
                    p2_computer.programme,
                    p2_computer.programme.len()
                );
                test_a_reg *= 8;
            } else {
                test_a_reg += 1;
            }
        }

        // And we're done!
        (p1_output_string, test_a_reg.to_string())
    }
}

fn try_with_val(p2_computer: &Computer, test_a_reg: i64) -> (bool, Vec<i64>) {
    let mut test_computer = p2_computer.clone();
    test_computer.registers.insert('A', test_a_reg);
    let test_output = test_computer.run(false);

    let mut stars = "".to_string();
    let mut halfstars = "".to_string();
    let mut spaces = "".to_string();
    for ii in 0..test_output.len() {
        if ii >= p2_computer.programme.len() {
            break;
        }
        if test_output[ii] == p2_computer.programme[ii] {
            if spaces.len() == 0 {
                stars.push('*');
            } else {
                halfstars.push('?');
            }
        } else {
            spaces.push(' ');
        }
    }
    (test_output == test_computer.programme, test_output)
}

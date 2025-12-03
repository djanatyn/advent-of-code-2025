#[derive(Debug, Clone)]
struct Dial {
    dial: i32,
    rotations_at_zero: i32,
}

#[derive(Debug, Clone)]
enum Instruction {
    Right(i32),
    Left(i32),
}

impl Dial {
    pub const MODULO: i32 = 100;

    fn process(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Left(magnitude) => {
                let difference = self.dial - magnitude;
                let result_dial = difference.rem_euclid(Self::MODULO);
                let mut rotations = difference.div_euclid(Self::MODULO).abs();

                // if we start at 0, we already counted it
                if self.dial == 0 {
                    rotations -= 1
                }

                // if we start at 0, we already counted it
                if result_dial == 0 {
                    rotations += 1
                }

                self.dial = dbg!(result_dial);
                self.rotations_at_zero += dbg!(rotations);
            }
            Instruction::Right(magnitude) => {
                let sum = self.dial + magnitude;
                let result_dial = sum.rem_euclid(Self::MODULO);
                let rotations = sum.div_euclid(Self::MODULO).abs();

                self.dial = dbg!(result_dial);
                self.rotations_at_zero += dbg!(rotations);
            }
        }
    }
}

impl Instruction {
    fn parse(input: &str) -> Instruction {
        let (direction, number) = input.split_at(1);
        dbg!(number);
        let magnitude = number.parse::<i32>().unwrap();
        match direction {
            "L" => Instruction::Left(magnitude),
            "R" => Instruction::Right(magnitude),
            _ => panic!("invalid input"),
        }
    }
}

fn main() {
    let instructions: Vec<Instruction> = std::fs::read_to_string("input1.txt")
        .unwrap()
        .lines()
        .map(|line| Instruction::parse(line))
        .collect();
    let mut dial = Dial {
        dial: 50,
        rotations_at_zero: 0,
    };
    for instruction in instructions {
        dbg!((&dial, &instruction));
        dial.process(instruction);
    }
    dbg!(&dial);
}

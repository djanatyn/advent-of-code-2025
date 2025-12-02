#[derive(Debug, Clone)]
struct Dial {
    dial: i32,
    rotations_at_zero: i32,
}

impl Dial {
    fn process(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Left(magnitude) => {
                let mut rotation = self.dial - magnitude;
                while rotation < 0 {
                    rotation += 100
                }
                self.dial = rotation;
            }
            Instruction::Right(magnitude) => {
                let mut rotation = self.dial + magnitude;
                while rotation >= 100 {
                    rotation -= 100
                }
                self.dial = rotation;
            }
        }

        if self.dial == 0 {
            self.rotations_at_zero += 1;
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Right(i32),
    Left(i32),
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

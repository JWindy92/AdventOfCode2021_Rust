
struct Submarine {
    position: i32,
    depth: i32,
    aim: i32,
}

impl Submarine {

    fn init() -> Submarine {
        Submarine {
            position: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn navigate(&mut self, instruction: (String, i32)) {
        if instruction.0 == "up" || instruction.0 == "down" {
            self.adjust_aim(instruction);
        }
        else if instruction.0 == "forward" {
            self.move_forward(instruction);
        }
    }

    fn move_forward(&mut self, instruction: (String, i32)) {
        self.position += instruction.1;
        self.depth += self.aim * instruction.1;
    }

    fn adjust_aim(&mut self, instruction: (String, i32)) {
        if instruction.0 == "down" {
            self.aim += instruction.1;
        } else {
            self.aim -= instruction.1;
        }
    }

    fn show_position(&self) {
        println!("Position: {}", self.position);
        println!("Depth: {}", self.depth);
        println!("Answer: {}", self.position * self.depth);
    }
}

fn main() {
    let instructions = read_instructions();
    let mut sub = Submarine::init();
    
    for instruction in instructions {
        sub.navigate(instruction);
    }

    sub.show_position();
}

fn read_instructions() -> Vec<(String, i32)> {
    let file = std::fs::read_to_string(String::from("input.txt")).expect("Failed to read input file");

    let mut instructions: Vec<(String, i32)> = vec![];
    for line in file.lines() {
        let mut split = line.split(" ");
        let instruction: (String, i32) = (split.next().unwrap().to_string(), split.next().unwrap().parse().unwrap());
        instructions.push(instruction)
    }

    println!("# Instructions: {}", instructions.len());
    return instructions
}

// *** Code used to solve part one of the challenge *** //

// fn navigate(&mut self, instruction: (String, i32)) {
//     if instruction.0 == "up" || instruction.0 == "down" {
//         self.move_vertically(instruction);
//     }
//     else if instruction.0 == "forward" {
//         self.move_horizontal(instruction);
//     }
// }

// fn move_horizontal(&mut self, instruction: (String, i32)) {
//     self.position += instruction.1;
// }

// fn move_vertically(&mut self, instruction: (String, i32)) {
//     if instruction.0 == "down" {
//         self.depth += instruction.1;
//     } else {
//         self.depth -= instruction.1;
//     }
// }
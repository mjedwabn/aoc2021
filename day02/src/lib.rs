pub mod day02 {
    use std::io::BufRead;

    pub fn get_multiplied_position(input: &mut dyn BufRead) -> i32 {
        let instructions = parse_input(input);

        let mut nav = Navigation::new();

        for i in instructions {
            nav.execute_instruction(i);
        }

        return nav.depth * nav.horizontal_position;
    }

    pub fn get_multiplied_position2(input: &mut dyn BufRead) -> i32 {
        let instructions = parse_input(input);

        let mut nav = Navigation2::new();

        for i in instructions {
            nav.execute_instruction(i);
        }

        return nav.depth * nav.horizontal_position;
    }

    pub struct Navigation {
        depth: i32,
        horizontal_position: i32
    }

    pub trait NavigationSystem {
        fn execute_instruction(&mut self, instruction: (String, i32));
    }

    impl NavigationSystem for Navigation {
        fn execute_instruction(&mut self, instruction: (String, i32)) { 
            if instruction.0 == "forward" {
                self.horizontal_position += instruction.1;
            }
            else if instruction.0 == "down" {
                self.depth += instruction.1;
            }
            else if instruction.0 == "up" {
                self.depth -= instruction.1;
            }
        }
    }

    impl Navigation {
        fn new() -> Navigation {
            Navigation {
                depth: 0,
                horizontal_position: 0
            }
        }
    }

    pub struct Navigation2 {
        depth: i32,
        horizontal_position: i32,
        aim: i32
    }

    impl NavigationSystem for Navigation2 {
        fn execute_instruction(&mut self, instruction: (String, i32)) { 
            if instruction.0 == "forward" {
                self.horizontal_position += instruction.1;
                self.depth += self.aim * instruction.1;
            }
            else if instruction.0 == "down" {
                self.aim += instruction.1;
            }
            else if instruction.0 == "up" {
                self.aim -= instruction.1;
            }
        }
    }

    impl Navigation2 {
        fn new() -> Navigation2 {
            Navigation2 {
                depth: 0,
                horizontal_position: 0,
                aim: 0
            }
        }
    }

    fn parse_input(input: &mut dyn BufRead) -> Vec<(String, i32)> {
        return input.lines()
            .map(|line| line.unwrap())
            .map(|line| parse_instruction(&line))
            .collect();
    }

    fn parse_instruction(instruction: &String) -> (String, i32) {
        let parts: Vec<&str> = instruction.split(" ").collect();
        return (String::from(parts[0]), parts[1].parse::<i32>().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use crate::day02;
    use std::{fs::File, io::BufReader};

    #[test]
    fn day01_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day02::get_multiplied_position(&mut f), 150);
    }

    #[test]
    fn part1_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day02::get_multiplied_position(&mut f), 1635930);
    }

    #[test]
    fn day01_part2_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day02::get_multiplied_position2(&mut f), 900);
    }

    #[test]
    fn part2_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day02::get_multiplied_position2(&mut f), 1781819478);
    }
}

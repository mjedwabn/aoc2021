pub mod day01 {
    use std::io::BufRead;

    pub fn count_depth_increases(input: &mut dyn BufRead) -> usize {
        let depths = parse_input(input);
        return depths.windows(2)
            .filter(|w| &w[1] > &w[0])
            .count();
    }

    pub fn count_the_number_of_times_the_sum_increases(input: &mut dyn BufRead) -> usize {
        let depths = parse_input(input);
        return depths.windows(3)
            .map(|w| &w[0] + &w[1] + &w[2])
            .collect::<Vec<i32>>()
            .windows(2)
            .filter(|w| &w[1] > &w[0])
            .count();
    }

    fn parse_input(input: &mut dyn BufRead) -> Vec<i32> {
        return input.lines()
            .map(|line| line.unwrap())
            .map(|line| line.parse::<i32>().unwrap())
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use crate::day01;
    use std::{fs::File, io::BufReader};

    #[test]
    fn day01_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day01::count_depth_increases(&mut f), 7);
    }

    #[test]
    fn part1_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day01::count_depth_increases(&mut f), 1233);
    }

    #[test]
    fn day01_part2_sample_input() {
        let mut f = BufReader::new(File::open("./sample.input").unwrap());
        assert_eq!(day01::count_the_number_of_times_the_sum_increases(&mut f), 5);
    }

    #[test]
    fn day01_part2_input() {
        let mut f = BufReader::new(File::open("./day.input").unwrap());
        assert_eq!(day01::count_the_number_of_times_the_sum_increases(&mut f), 1275);
    }
}

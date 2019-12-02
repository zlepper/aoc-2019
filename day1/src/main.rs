use aoc_lib::AocImplementation;

fn main() {
    let day1 = Day1 {};

    day1.start(1)
}

struct Day1 {}

impl AocImplementation<i32> for Day1 {
    fn process_input(&self, input: &str) -> Vec<i32> {
        input.split('\n').map(|line| line.parse().unwrap()).collect()
    }

    fn execute(&self, input: Vec<i32>) -> Option<i32> {
        let answer = input.into_iter()
            .map(calculate_fuel)
            .map(|fuel| fuel + calculate_additional_fuel(fuel))
            .sum();
        Some(answer)
    }
}

fn calculate_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn calculate_additional_fuel(fuel: i32) -> i32 {
    let additional_fuel = calculate_fuel(fuel);

//    println!("Additional fuel {} fuel {}", additional_fuel, fuel);
    if additional_fuel <= 0 {
        0
    } else {
        additional_fuel + calculate_additional_fuel(additional_fuel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod calculate_fuel_for_mass {
        use super::*;

        #[test]
        fn example1() {
            let result = calculate_fuel(12);
            assert_eq!(result, 2);
        }

        #[test]
        fn example2() {
            let result = calculate_fuel(14);
            assert_eq!(result, 2);
        }

        #[test]
        fn example3() {
            let result = calculate_fuel(1969);
            assert_eq!(result, 654);
        }

        #[test]
        fn example4() {
            let result = calculate_fuel(100_756);
            assert_eq!(result, 33583);
        }
    }

    mod calculate_rocket_fuel_mass_science {
        use super::*;

        #[test]
        fn example1() {
            let result = calculate_additional_fuel(2) + 2;
            assert_eq!(result, 2);
        }

        #[test]
        fn example2() {
            let result = calculate_additional_fuel(654) + 654;
            assert_eq!(result, 966);
        }

        #[test]
        fn example3() {
            let result = calculate_additional_fuel(33583) + 33583;
            assert_eq!(result, 50346);
        }
    }
}
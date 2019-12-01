use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read input file");

    let total_fuel: i32 = contents.split('\n').map(|line| line.parse().unwrap()).map(calculate_fuel).map(|fuel| fuel + calculate_additional_fuel(fuel)).sum();

    println!("total {}", total_fuel)

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
            let result= calculate_additional_fuel(2) + 2;
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
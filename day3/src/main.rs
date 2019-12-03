use std::collections::HashSet;
use aoc_lib::AocImplementation;


fn main() {
    let day = Day3{};
    day.start(3);
}

struct Day3 {}

impl AocImplementation<Vec<String>> for Day3 {
    fn process_input(&self, input: &str) -> Vec<Vec<String>> {
        input.split('\n').map(|line| line.split(',').map(|c| c.to_owned()).collect()).collect()
    }

    fn execute(&self, input: Vec<Vec<String>>) -> Option<i32> {
        let w1t = input[0].clone();
        let w1 = w1t.iter().map(|s| &s[..]).collect();
        let w2t = input[1].clone();
        let w2 = w2t.iter().map(|s| &s[..]).collect();

//        let distance = find_crossing_wires(w1, w2);
//        Some(distance)

        let distance = find_first_crossing_point(w1, w2);
        Some(distance)
    }
}

fn find_first_crossing_point(wire1: Vec<&str>, wire2: Vec<&str>) -> i32 {
    let wire1_coords = find_touched_coordinates(wire1);
    let wire2_coords = find_touched_coordinates(wire2);

    let intersections = find_intersections(wire1_coords.clone().into_iter().collect(), wire2_coords.clone().into_iter().collect());

    let mut first_intersection: Vec<usize> = intersections.iter()
        .map(|inter| wire1_coords.iter().position(|c| c == inter).unwrap() + wire2_coords.iter().position(|c| c == inter).unwrap() + 2)
        .collect();

    first_intersection.sort_by(|a, b| a.cmp(b));

    first_intersection[0] as i32
}

// Calculates the distance to the closest intersection
fn find_crossing_wires(wire1: Vec<&str>, wire2: Vec<&str>) -> i32 {
    let wire1_coords = find_touched_coordinates(wire1);
    let wire2_coords = find_touched_coordinates(wire2);

    let intersections = find_intersections(wire1_coords.into_iter().collect(), wire2_coords.into_iter().collect());

    let mut ordered: Vec<(i32, i32)> = intersections.into_iter().collect();
    ordered.sort_by(|a, b| calculate_manhattan_distance(a).cmp(&calculate_manhattan_distance(b)));

    let closests = calculate_manhattan_distance(&ordered[0]);

    closests
}

fn find_intersections(wire1: HashSet<(i32, i32)>, wire2: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    wire1.intersection(&wire2).map(|pos| pos.to_owned()).collect()
}

fn calculate_manhattan_distance((x, y): &(i32, i32)) -> i32 {
    x.abs() + y.abs()
}

// Returns all the coordinates that this wire touches
fn find_touched_coordinates(operations: Vec<&str>) -> Vec<(i32, i32)> {
    let mut coordinates = Vec::new();
    let mut current_position = (0, 0);
    for op in operations {
        let direction = &op[0..1];
        let distance: i32 = op[1..].parse().unwrap();
        let change = match direction {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => panic!("Unexpected input: {}", op)
        };

        for _ in 0..distance {
            current_position = (current_position.0 + change.0, current_position.1 + change.1);
            coordinates.push(current_position);
        }
    }

    coordinates
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn wire1_paths() {
        let wire1 = vec!["R8", "U5", "L5", "D3"];

        let coordinates = find_touched_coordinates(wire1);

        let expected = vec![(1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (8, 1), (8, 2), (8, 3), (8, 4), (8, 5), (7, 5), (6, 5), (5, 5), (4, 5), (3, 5), (3, 4), (3, 3), (3, 2)];


        assert_eq!(coordinates, expected)
    }

    #[test]
    fn calculates_manhattan_distance() {
        let coord = (3, 3);
        let distance = calculate_manhattan_distance(&coord);

        assert_eq!(distance, 6)
    }

    #[test]
    fn finds_interaction() {
        let intersections = find_intersections(HashSet::from_iter(vec![(1, 1), (4, 3)]), HashSet::from_iter(vec![(3, 4), (1, 1)]));

        assert_eq!(intersections, HashSet::from_iter(vec![(1, 1)]))
    }

    #[test]
    fn example1() {
        let distance = find_crossing_wires(vec!["R8", "U5", "L5", "D3"], vec!["U7", "R6", "D4", "L4"]);
        assert_eq!(distance, 6);
    }

    #[test]
    fn part1_example2() {
        let distance = find_first_crossing_point("R75,D30,R83,U83,L12,D49,R71,U7,L72".split(',').collect(), "U62,R66,U55,R34,D71,R55,D58,R83".split(',').collect());
        assert_eq!(distance, 610)
    }
}

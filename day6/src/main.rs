use std::collections::{HashMap, VecDeque};
use aoc_lib::AocImplementation;

fn main() {
    let day = Day6{};
    day.start(6);
}

struct Day6 {

}

impl AocImplementation<String> for Day6 {
    fn process_input(&self, input: &str) -> Vec<String> {
        input.split('\n').map(|s| s.to_string()).collect()
    }

    fn execute(&self, input: Vec<String>) -> Option<i32> {
       Some(get_orbit_count(input.iter().map(|s| &s[..]).collect()))
    }
}

fn get_orbit_map(rows: Vec<&str>) -> HashMap<&str, &str> {
     rows.into_iter()
        .map(|r| r.split(')').collect::<Vec<&str>>())
        .map(|pair| (pair[1], pair[0]))
        .collect()
}

#[derive(Clone, Debug)]
struct Node {
    visited: bool,
    orbits: String,
    name: String
}

#[derive(Clone, Debug)]
struct QueuedPosition {
    depth: i32,
    node: String
}

fn get_orbit_transfers(rows: Vec<&str>) -> i32 {
    let pairs = get_orbit_map(rows);

    let mut nodes: HashMap<String, Node> = pairs.into_iter().map(|p| (p.1.to_string(), Node{visited: false, orbits: p.0.to_string(), name: p.1.to_string()})).collect();

    println!("nodes: {:#?}", nodes);

    let currently_orbiting = nodes.remove("YOU").unwrap().orbits;
    let santa_orbiting = nodes.remove("SAN").unwrap().orbits;

    let mut queue = VecDeque::new();

    queue.push_back(QueuedPosition {
        depth: 0,
        node: currently_orbiting.clone()
    });

    nodes.get_mut(&currently_orbiting).unwrap().visited = true;

    while let Some(n) = queue.pop_front() {

        if santa_orbiting == n.node {
            return n.depth
        }

        let nodes_c = nodes.clone();
        let node = nodes_c.get(&n.node).unwrap();

        {
            let mut i = nodes.clone();
            let before = i.get_mut(&node.orbits).unwrap();

            if !before.visited {
                before.visited = true;

                queue.push_back(QueuedPosition { depth: n.depth + 1, node: before.name.clone() })
            }
        }

        for (_, after) in nodes.iter_mut().filter(|a| a.1.orbits == node.name) {

            if after.visited {
                continue
            }

            after.visited = true;
            queue.push_back(QueuedPosition {
                node: after.name.clone(),
                depth: n.depth + 1
            })
        }


    }


    -1


}



fn get_orbit_count(rows: Vec<&str>) -> i32 {
    let pairs = get_orbit_map(rows);

    let mut counts = 0;

    for (key, _value) in &pairs {
        let mut next = key;
        while let Some(after) = pairs.get(next) {
            counts += 1;
            next = after;
        }
    }

    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let data = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"#;

        let input = data.split('\n').collect();
        let count = get_orbit_count(input);
        assert_eq!(count, 42)
    }

    #[test]
    fn example2() {
        let data = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"#;

        let input = data.split('\n').collect();
        let count = get_orbit_transfers(input);
        assert_eq!(count, 4)
    }
}
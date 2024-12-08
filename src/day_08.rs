use std::collections::HashMap;

/**
   Returns a hashmap that's built from the input at path. The hashmap contains the antenna symbol
   as key, and the positions of all antennas with the same symbol as a vector of (x, y) tuples.
 */
fn preprocess(path: &str) -> (usize, usize, HashMap<char, Vec<(usize, usize)>>) {
    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let contents = std::fs::read_to_string(path).expect(path);
    let lines: Vec<&str> = contents.lines().collect();
    let max_y = lines.len() - 1;
    let max_x = lines[0].chars().count() - 1;
    
    // Iterate over every position and sort the found antennas into the hashmap
    for y in 0..=max_y {
        let line = lines[y];
        let chars: Vec<char> = line.chars().collect();

        for x in 0..=max_x {
            let c = chars[x];
            if c == '.' {
                continue;
            }

            if let Some(positions) = map.get_mut(&c) {
                positions.push((x, y));
            } else {
                map.insert(c, vec![(x, y)]);
            }
        }
    }

    (max_x, max_y, map)
}

/**
   Computes the antinodes of two positions. Doesn't check for bounds, instead it just returns the
   two possible isize positions, which of course might be negative.
 */
fn get_antinodes_of(first: (usize, usize), second: (usize, usize)) -> ((isize, isize), (isize, isize)) {
    let f = (first.0 as isize, first.1 as isize);
    let s = (second.0 as isize, second.1 as isize);
    let diff = (s.0 - f.0, s.1 - f.1);

    ((f.0 - diff.0, f.1 - diff.1), (s.0 + diff.0, s.1 + diff.1))
}

pub fn part_1(path: &str) -> u32 {
    let (max_x_u, max_y_u, map) = preprocess(path);
    let max_x = max_x_u as isize;
    let max_y = max_y_u as isize;
    let mut all_antinodes: Vec<(isize, isize)> = vec![];

    for values in map.values() {
        // The range already guarantees that we stay in bounds. If the values only contain one
        // antenna positions, the 'j' loop will never run, and the 'i' loop will terminate after
        // the first iteration.
        for i in 0..values.len() {
            for j in i+1..values.len() {
                let i_j_antinodes = get_antinodes_of(values[i], values[j]);
                all_antinodes.push(i_j_antinodes.0);
                all_antinodes.push(i_j_antinodes.1);
            }
        }
    }

    all_antinodes.sort();
    let mut in_bound_antinodes: Vec<&(isize, isize)> = all_antinodes.iter()
        .filter(|antinode| antinode.0 >= 0 && antinode.0 <= max_x
            && antinode.1 >= 0 && antinode.1 <= max_y)
        .collect();
    in_bound_antinodes.dedup();

    println!("Found {} unique antinode positions", in_bound_antinodes.len());
    in_bound_antinodes.len() as u32
}

/**
   Computes the resonant antinodes of two positions. The two positions themselves
   are also included in the resulting positions, since the two positions resonate
   with each other. All returned positions are guaranteed to stay in bounds.
 */
fn get_resonant_antinodes_of(first: (usize, usize), second: (usize, usize), max_x: isize, max_y: isize) -> Vec<(usize, usize)> {
    let f = (first.0 as isize, first.1 as isize);
    let s = (second.0 as isize, second.1 as isize);
    let diff = (s.0 - f.0, s.1 - f.1);

    let mut results: Vec<(usize, usize)> = vec![];

    // ((f.0 - diff.0, f.1 - diff.1), (s.0 + diff.0, s.1 + diff.1))
    let mut current_modified_first = f.clone();

    // As long as the modifications stay in bounds, add the modified position to the results
    while current_modified_first.0 >= 0 && current_modified_first.0 <= max_x
            && current_modified_first.1 >= 0 && current_modified_first.1 <= max_y {
        results.push((current_modified_first.0 as usize, current_modified_first.1 as usize));
        current_modified_first = (current_modified_first.0 - diff.0, current_modified_first.1 - diff.1);
    }

    let mut current_modified_second = s.clone();

    // As long as the modifications stay in bounds, add the modified position to the results
    while current_modified_second.0 >= 0 && current_modified_second.0 <= max_x
            && current_modified_second.1 >= 0 && current_modified_second.1 <= max_y {
        results.push((current_modified_second.0 as usize, current_modified_second.1 as usize));
        current_modified_second = (current_modified_second.0 + diff.0, current_modified_second.1 + diff.1);
    }

    results
}

pub fn part_2(path: &str) -> u32 {
    let (max_x_u, max_y_u, map) = preprocess(path);
    let max_x = max_x_u as isize;
    let max_y = max_y_u as isize;

    let mut all_antinodes: Vec<(usize, usize)> = vec![];

    for values in map.values() {
        // The range already guarantees that we stay in bounds. If the values only contain one
        // antenna positions, the 'j' loop will never run, and the 'i' loop will terminate after
        // the first iteration.
        for i in 0..values.len() {
            for j in i+1..values.len() {
                let mut i_j_antinodes = get_resonant_antinodes_of(values[i], values[j], max_x, max_y);
                all_antinodes.append(&mut i_j_antinodes);
            }
        }
    }

    all_antinodes.sort();
    all_antinodes.dedup();

    println!("Found {} unique resonant antinode positions", all_antinodes.len());
    all_antinodes.len() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_preprocess() {
        let (max_x, max_y, map) = preprocess("./inputs/day_08_test.txt");
        assert_eq!(max_x, 11);
        assert_eq!(max_y, 11);
        assert!(map.contains_key(&'0'));
        assert_eq!(*map.get(&'0').unwrap(), vec![(8, 1), (5, 2), (7, 3), (4, 4)]);
    }

    #[test]
    fn test_get_antinodes_of_normal_positions() {
        let antinodes = get_antinodes_of((8, 8), (9, 9));
        assert!(antinodes.0 == (7, 7));
        assert!(antinodes.1 == (10, 10));
    }

    #[test]
    fn test_get_antinodes_out_of_bounds() {
        let antinodes = get_antinodes_of((1, 1), (3, 3));
        assert!(antinodes.0 == (-1, -1));
        assert!(antinodes.1 == (5, 5));
    }

    #[test]
    fn test_get_antinode_reverse() {
        let antinodes = get_antinodes_of((9, 9), (8, 8));
        assert!(antinodes.0 == (10, 10));
        assert!(antinodes.1 == (7, 7));
    }

    #[test]
    fn test_part_1() {
        let result = part_1("./inputs/day_08_test.txt");
        assert_eq!(result, 14);
    }

    #[test]
    fn get_resonant_antinodes_of_normal_positions() {
        let antinodes = get_resonant_antinodes_of((0, 0), (1, 1), 3, 3);
        assert_eq!(antinodes, vec![(0, 0), (1, 1), (2, 2), (3, 3)]);
    }

    #[test]
    fn get_resonant_antinodes_of_more_complex_positions() {
        let antinodes = get_resonant_antinodes_of( (4, 6), (3, 4), 8, 10);
        assert_eq!(antinodes, vec![(4, 6), (5, 8), (6, 10), (3, 4), (2, 2), (1, 0)]);
    }

    #[test]
    fn test_part_2() {
        let result = part_2("./inputs/day_08_test.txt");
        assert_eq!(result, 34);
    }
}
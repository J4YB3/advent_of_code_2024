use std::collections::HashMap;

/**
   Returns a map that contains the page rules. The keys represent the page that needs to come first,
   the value is a vector of page numbers that need to come after the page in the key.
   Also returns a vector of lines, where each line is itself a vector of numbers that represent the 
   order of the pages for the single updates. Each line is one update.
 */
fn preprocess(path: &str) -> (HashMap<u8, Vec<u8>>, Vec<Vec<u8>>) {
    let contents = std::fs::read_to_string(path).expect(path);
    let lines_iterator = contents.lines().map(|x| x.to_string());

    let mut rules: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut updates: Vec<Vec<u8>> = vec![];
    let mut rules_finished = false;

    lines_iterator
        .for_each(|line| {
            if line == "" {
                rules_finished = true;
                return;
            }

            if rules_finished {
                updates.push(line.split(',').map(|v| v.parse::<u8>().expect("Should be a parsable u8")).collect());
                return;
            }

            let rule: Vec<u8> = line.split('|').map(|v| v.parse::<u8>().expect("Should be a parsable u8")).collect();
            match rules.get(&rule[0]) {
                Some(previous_ruleset) => {
                    let mut updated_ruleset = previous_ruleset.clone();
                    updated_ruleset.push(rule[1]);
                    rules.insert(rule[0], updated_ruleset);
                },
                None => { rules.insert(rule[0], vec![rule[1]]); },
            }
        });

    (rules, updates)
}

/**
   Returns None if the update is valid (no invalid index was found). And Some(idx) with
   the index of the first element that was invalid, if the update is invalid
 */
fn is_update_valid(rules: &HashMap<u8, Vec<u8>>, update: &Vec<u8>) -> Option<usize> {
    // Go through all numbers in the update and validate
    // that the rule conditions are fulfilled
    for current_number_idx in 0..update.len() {
        let current_number = update[current_number_idx];

        if let Some(rule_values) = rules.get(&current_number) {
            // Check if any number before the current number is required to come
            // after the current number. If so, the update is invalid
            for other_number_idx in 0..current_number_idx {
                let other_number = update[other_number_idx];
                if rule_values.contains(&other_number) {
                    return Some(current_number_idx);
                }
            }
        }
    }

    None
}

/**
   Finds and returns the middle page number (median) of the given update
 */
fn get_middle_page_number(update: &Vec<u8>) -> u8 {
    let middle_index = (update.len() - 1) / 2;
    update[middle_index]
}

pub fn part_1(path: &str) -> u16 {
    let (rules, updates) = preprocess(path);
    let mut sum: u16 = 0;

    for update in updates {
        if let None = is_update_valid(&rules, &update) {
            sum += get_middle_page_number(&update) as u16;
        }
    }

    println!("The sum of the middle page numbers of the valid updates was {}", sum);
    sum
}

/**
   Corrects the order of the update if it isn't already in the correct order.
   It does this by checking for each element if it is already in the correct order.
   If not, it bubbles up the element to the maximum position where it fulfills all of its rules.
   The implemented algorithm has a worst case complexity of O(n^2).
   It's possible that the break condition for the reset_counter is not required, but I didn't spend
   the extra time to verify the invariant for loop, so I left it in to be sure.
 */
fn correct_ordering(rules: &HashMap<u8, Vec<u8>>, update: &Vec<u8>) -> Option<Vec<u8>> {
    let mut current_number_idx: usize = 0;
    let mut local_update = update.clone();
    let mut reset_counter: u16 = 0;

    loop {
        if current_number_idx == local_update.len() {
            break;
        }

        let current_update_valid = is_update_valid(rules, &local_update);
        // If the update is valid, we have found the first valid configuration after the minimum
        // amount of steps, so we return the update configuration.
        if current_update_valid.is_none() {
            return Some(local_update);
        }

        // Otherwise, get the first position where the update didn't fulfill the rules
        let wrong_index = current_update_valid.unwrap();
        let mut min_valid_idx_option: Option<usize> = None;

        // If there is a rule for this number, find the minimum index where this number has to
        // be placed, so the number is at the right spot and fulfills all of its rules.
        if let Some(wrong_number_rule_values) = rules.get(&local_update[wrong_index]) {
            for i in 0..wrong_index {
                let checked_number = local_update[i];
                if wrong_number_rule_values.contains(&checked_number) {
                    min_valid_idx_option = Some(i);
                    break;
                }
            }
        }

        // If a minimum valid index was found, shift the number to that spot
        if let Some(min_valid_idx) = min_valid_idx_option {
            let removed_number = local_update.remove(wrong_index);
            local_update.insert(min_valid_idx, removed_number);

            reset_counter += 1;
            // If there were 25 resets already, break the loop. It's likely that it's a rule
            // configuration that can never be fulfilled by the current update.
            if reset_counter >= 25 {
                return None;
            }

            // Reset the current_number_idx because the shift might
            // have caused already checked values to be invalid now.
            current_number_idx = 0;
            continue;
        }

        current_number_idx += 1;
    }

    Some(local_update)
}

pub fn part_2(path: &str) -> u16 {
    let (rules, updates) = preprocess(path);
    
    let mut sum: u16 = 0;

    for update in updates {
        // If the update is invalid
        if let Some(_) = is_update_valid(&rules, &update) {
            if let Some(corrected_update) = correct_ordering(&rules, &update) {
                sum += get_middle_page_number(&corrected_update) as u16;
            }
        }
    }

    println!("The sum of the middle page numbers of the corrected updates was {}", sum);
    sum
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn test_preprocess() {
        let (rules, updates) = preprocess("./inputs/day_05_test.txt");

        assert_eq!(rules.len(), 6);

        assert!(rules.get(&47).is_some_and(|x| x.len() == 4));
        assert!(rules.get(&97).is_some_and(|x| x.len() == 6));
        assert!(rules.get(&75).is_some_and(|x| x.len() == 5));
        assert!(rules.get(&61).is_some_and(|x| x.len() == 3));
        assert!(rules.get(&29).is_some_and(|x| x.len() == 1));
        assert!(rules.get(&53).is_some_and(|x| x.len() == 2));

        assert_eq!(updates[0], vec![75, 47, 61, 53, 29]);
        assert_eq!(updates.len(), 6);
    }

    #[test]
    fn test_is_update_valid() {
        let (rules, updates) = preprocess("./inputs/day_05_test.txt");
        assert!(is_update_valid(&rules, &updates[0]).is_none());
        assert!(is_update_valid(&rules, &updates[1]).is_none());
        assert!(is_update_valid(&rules, &updates[2]).is_none());
        assert_eq!(is_update_valid(&rules, &updates[3]).unwrap(), 1);
        assert_eq!(is_update_valid(&rules, &updates[4]).unwrap(), 2);
        assert_eq!(is_update_valid(&rules, &updates[5]).unwrap(), 2);
    }

    #[test]
    fn test_part_1() {
        let result = part_1("./inputs/day_05_test.txt");
        assert_eq!(result, 143);
    }

    #[test]
    fn test_correct_ordering() {
        let (rules, _) = preprocess("./inputs/day_05_test.txt");
        assert_eq!(correct_ordering(&rules, &vec![75, 97, 47, 61, 53]), Some(vec![97, 75, 47, 61, 53]));
        assert_eq!(correct_ordering(&rules, &vec![61, 13, 29]), Some(vec![61, 29, 13]));
        assert_eq!(correct_ordering(&rules, &vec![97, 13, 75, 29, 47]), Some(vec![97, 75, 47, 29, 13]));

        // A vector that's already in the correct order
        let vector: Vec<u8> = vec![75, 47, 61, 53, 29];
        assert_eq!(correct_ordering(&rules, &vector), Some(vector));
    }

    #[test]
    fn test_part_2() {
        let result = part_2("./inputs/day_05_test.txt");
        assert_eq!(result, 123);
    }
}
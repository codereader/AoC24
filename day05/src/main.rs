use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Read;
use std::fs::File;

fn main() {
    // Create an empty mutable string
    let _test_content = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    
    // Read a file in the local file system
    let mut data_file = File::open("src/input.txt").unwrap();
    let mut file_content = String::new();
    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();
    
    let normalized_file = file_content.replace('\r', "");
    let lines = normalized_file.split('\n');

    let mut rules_before: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut rules_after: HashMap<i32, HashSet<i32>> = HashMap::new();

    let mut i = lines;
    loop {
        let line = i.next();

        if line.is_none() || line.unwrap().trim().is_empty() {
            break;
        }

        //println!("{}", line.unwrap());
        let pieces: Vec<_> = line.unwrap().split('|').collect();
        let key = pieces[0].parse::<i32>().expect("Parser error");
        let value = pieces[1].parse::<i32>().expect("Parser error");

        let before_set = rules_before.entry(key).or_insert_with(|| HashSet::new());
        before_set.insert(value);

        let after_set = rules_after.entry(value).or_insert_with(|| HashSet::new());
        after_set.insert(key);
    }

    let mut printouts = Vec::new();
    loop {
        let line = i.next();

        if line == None || line.unwrap().is_empty() {
            break;
        }
        
        let pages:Vec<_> = line.unwrap().split(',').map(|x| x.parse::<i32>().expect("Parse error")).collect();
        printouts.push(pages);
    }

    println!("Evaluating Pages...");
    let mut failed_pages: Vec<Vec<i32>> = Vec::new();

    let mut sum_part1 = 0;

    for pages in printouts.iter() {

        let is_ok = evaluate_page(pages.clone(), &rules_before, &rules_after, |_, _| {});

        if is_ok {
            //println!("Page is OK");
            let middle_index = pages.len() / 2;
            //println!("Middle Page: {0}", pages[middle_index]);
            sum_part1 += pages[middle_index];
        }
        else {
            //println!("Page FAILED");
            failed_pages.push(pages.clone());
        }
    }

    println!("[Part1]: Sum of middle page numbers = {0}", sum_part1); // 5948

    let mut sum_part2 = 0;

    for mut pages in failed_pages {
        //println!("{:?}", pages);

        let mut _runs = 0;
        while !evaluate_page(pages.clone(), &rules_before, &rules_after, |a: usize, b: usize|
        {
            let temp = pages[a];
            pages[a] = pages[b];
            pages[b] = temp;
        }) {
            _runs += 1;
        }

        //println!("Page after {0} runs: {1:?}", runs, pages);
        let middle_index = pages.len() / 2;
        //println!("Middle Page: {0}", pages[middle_index]);
        sum_part2 += pages[middle_index];
    }

    println!("[Part2]: Sum of middle page numbers of fixed pages = {0}", sum_part2); // 3062
}

fn evaluate_page<F: FnMut(usize, usize)>(pages: Vec<i32>,
    rules_before: &HashMap<i32, HashSet<i32>>, rules_after: &HashMap<i32, HashSet<i32>>,
    mut fix_action: F) -> bool {
    for i in 0..pages.len() {
        for left in 0..i {
            //print!("{0} must be after {1}: ", pages[i], pages[left]);

            let rules_for_page = rules_after.get(&pages[i]);

            if rules_for_page.is_none() {
                continue;
            }

            if rules_for_page.unwrap().contains(&pages[left]) {
                //println!("OK");
            }
            else {
                //println!("Rule Violation");
                fix_action(i, left);
                return false;
            }
        }

        let rules_for_page = rules_before.get(&pages[i]);

        if rules_for_page.is_none() {
            continue;
        }

        for right in i+1..pages.len() {
            //print!("{0} must be before {1}: ", pages[i], pages[right]);

            if rules_for_page.unwrap().contains(&pages[right]) {
                //println!("OK");
            }
            else {
                fix_action(i, right);
                return false;
            }
        }
    }

    return true;
}
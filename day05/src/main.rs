use itertools::Itertools;

// the input was two clear segments, so i split it up manually
// instead of writing a disgusting regular expression
const INPUT1: &str = include_str!("../input_a");
const INPUT2: &str = include_str!("../input_b");

fn get_rules(input: &str) -> Vec<(usize,usize)> {
    input.lines()
        .map(|line| {
            line.split('|')
                .map(|i| i.parse::<usize>().unwrap())
                .collect_tuple().unwrap()
        }).collect::<Vec<(usize,usize)>>()

}
fn get_updates(input: &str) -> Vec<Vec<usize>> {
    input.lines()
        .map(|line| {
            line.split(',')
                .map(|i| i.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        }).collect::<Vec<Vec<usize>>>()
}

fn solve_2(rules: &Vec<(usize,usize)>, updates: &Vec<Vec<usize>>) -> u32 {
    // rather than checking the rules, we can just sort them by which has
    // the most relevant rules in the map
    let mut rule_map: Vec<Vec<bool>> = vec![vec![false; 100]; 100];
    for rule in rules {
        rule_map[rule.1][rule.0] = true;
    }

    let mut tally:u32 = 0;
    for update in updates {
        let mut valid = true;
        for seq in 0..update.len()-1 {
            // first determine validity
            if rule_map[update[seq]][update[seq+1]] {
                valid = false;
                break;
            }
        }
        if !valid {
            // repair
            // actually, we don't need to repair
            // we just have to get the middle one
            let midpoint = update.len() / 2;
            //println!("invalid: {:?}", update);
            // count how many times num|[other num in sequence] occurs in map
            for seq in update {
                // this calculates the index it would be in the correct array
                // by counting how many rules reference it
                let index:u32 = update
                    .iter()
                    .filter(|i| i != &seq)
                    .map(|ii| {
                        match rule_map[*seq][*ii] {
                            true => 1,
                            false => 0
                        }
                    }).sum();
                if index as usize == midpoint {
                    tally += *seq as u32;
                    break;
                }
            }

        }
    }

    tally
}

fn solve_1(rules: &Vec<(usize,usize)>, updates: &Vec<Vec<usize>>) -> u32 {

    // lets make a 2D map of all the rules that exist
    // when a comparison is made, say 75 > 42 we look up map[75][42]
    // if this is false, the combination is not allowed because a rule 42|75 exists
    // we can also print the map and it might look interesting
    let mut rule_map: Vec<Vec<bool>> = vec![vec![true; 100]; 100];
    for rule in rules {
        rule_map[rule.1][rule.0] = false;
    }

    let mut tally:u32 = 0;
    for update in updates {
        // validate update sequence
        // no need to check last one
        let mut valid = true;
        for seq in 0..update.len()-1 {
            if !rule_map[update[seq]][update[seq+1]] {
                // rule break found
                //println!("invalid: {:?}", update);
                //println!("idx {seq}: broke rule [{}|{}]",update[seq+1],update[seq]);
                valid = false;
                break;
            }
        }
        if valid {
            //println!("valid: {:?}", update);
            // if valid, find midpoint
            let midpoint = update.len() / 2;
            // add to tally
            //println!("midpoint: {}", update[midpoint]);
            tally += update[midpoint] as u32;
        }
    }
    tally
}

fn main()
{

    let rules = get_rules(INPUT1);
    let updates = get_updates(INPUT2);


    /*
    if false {
        println!("rules:");
        for l in rules {
            println!("{:?}", l);
        }
        println!("updates:");
        for l in updates {
            println!("{:?}", l);
        }
    }
    */

    println!("{}", solve_1(&rules,&updates));
    println!("{}", solve_2(&rules,&updates));
}
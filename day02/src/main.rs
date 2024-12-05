const INPUT: &str = include_str!("../input");

fn main() {

    println!("{}", solve_1(INPUT));
    println!("{}", solve_2(INPUT));

}

fn solve_1(file: &str) -> i32 {
    let lines: Vec<&str> = file.lines().collect();
    let mut good_lines = 0;
    for line in &lines {
        let parsed_line = line
            .split_whitespace()
            .map(|int_item| int_item.parse::<i32>().unwrap());
        if let None = find_element(parsed_line) {
            good_lines += 1
        }
    }
    good_lines
}

fn solve_2(file: &str) -> i32 {

    let lines: Vec<&str> = file.lines().collect();
    //println!("Total lines: {}",lines.len());

    let mut good_lines = 0;
    let mut good_lines_fixed = 0;

    for line in &lines {
        let parsed_line = line
            .split_whitespace()
            .map(|int_item | int_item.parse::<i32>().unwrap());
        if let Some(n) = find_element(parsed_line.clone()) {
            // println!("Bad found at {n}")
            let q = match n {
                n if n < 1 => 0,
                _ => n - 1
            };
            for i in q..=n+1 {
                let new_list = parsed_line.clone()
                .enumerate().filter(|(index,_)| index != &i)
                .map(|(_, value)| value);
                if find_element(new_list).is_none() {
                    good_lines_fixed += 1;
                    break;
                }
            }
        }
        else {
            good_lines += 1
        }

    }
    //println!("raw:   {good_lines}");
    //println!("fixed: {good_lines_fixed}");
    good_lines + good_lines_fixed

}

fn find_element(iter: (impl Iterator<Item = i32> + Clone)) -> Option<usize> {

    let mut clone = iter.clone();
    let first = clone.next().unwrap();
    let second = clone.next().unwrap();

    let comp = match first-second {
        n if n < 0 => |a: i32, b: i32| a < b && i32::abs_diff(a,b) <= 3,    // increasing
        n if n > 0 => |a: i32, b: i32| a > b && i32::abs_diff(a,b) <= 3,    // decreasing
        _ => |_a: i32, _b: i32| false                 // none of the above
    };
    if !(comp(0,1) || comp(1,0)) {
        // we are in the bad timeline
        //println!("equality");
        return Some(0);
    };
    iter.clone()
            .zip(iter.clone().skip(1))
            .map(|(a, b)| comp(a,b))
            .position(|a| a == false)
}
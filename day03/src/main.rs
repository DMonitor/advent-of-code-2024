use regex::Regex;

const INPUT: &str = include_str!("../input");

fn solve_1(input: &str) {
    //let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let restir = "mul\\((\\d+),(\\d+)\\)";
    println!("regex: {}", restir);
    let find_mults = Regex::new(restir).unwrap();
    let mults = find_mults.captures_iter(&input);

    let mut sum = 0;

    for m in mults {
        sum += m[1].parse::<i32>().unwrap() * m[2].parse::<i32>().unwrap();
    }

    println!("total: {sum}");
}
fn solve_2(input: &str) {
    //let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let restir = "(do\\(\\))|(don't\\(\\))|(?:mul\\((\\d+),(\\d+)\\))";
    println!("regex: {}", restir);
    let find_mults = Regex::new(restir).unwrap();
    let mults = find_mults.captures_iter(&input);

    let mut sum = 0;
    let mut modifier = 1;

    for m in mults {

        //println!("0: {}",&m[0]);
        if m.get(1).is_some() {
            modifier = 1;
            //println!("day01: {}",&m[1]);
        }
        if m.get(2).is_some() {
            modifier = 0;
            //println!("2: {}",&m[2]);
        }
        if modifier == 0 {
            continue;
        }
        if m.get(3).is_some() && m.get(4).is_some() {
            let mult = m[3].parse::<i32>().unwrap() * m[4].parse::<i32>().unwrap();
            //println!("mult({},{}) = {mult}", &m[3],&m[4]);
            sum += mult*modifier;
        }
    }
    
    println!("total: {sum}");
}

fn main()
{
    solve_1(INPUT);
    solve_2(INPUT);
}
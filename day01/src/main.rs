use itertools::Itertools;
const INPUT: &str = include_str!("../input");
fn parse_file(file: &str) -> (Vec<u32>, Vec<u32>) {

    file.lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map( |num| num.parse::<u32>().unwrap())
                .collect_tuple().unwrap()
        })
        .unzip()
}

// first, make list of uniques from day01
// then, for each unique, count appearances in 2
// sum appearances

// for each in day01, check if in day01
// else count appearances in 2 and add to total

// sort both lists
// in list1:
// if [n] = [n+day01] continue
// else:
//  in list2:
//  while list2[i] < [n] continue
//  if list2[i] == [n] ++
//  otherwise break


fn solve_2() -> u32 {

    let (mut list_one, mut list_two) = parse_file(INPUT);

    list_one.sort();
    list_two.sort();

    let mut total = 0;
    let mut list_one_p= list_one.into_iter().peekable();

    let mut iters = 1;
    while let Some(current) = list_one_p.next() {

        let next = list_one_p.peek();

        match next {
            Some(value) => {if *value == current {iters += 1; continue;}}
            None => ()
        }
        // should be unique now
        let mut matches = 0;
        for item in list_two.iter() {
            //println!("Checking {item} and {current}");
            if *item < current {
                continue;
            }
            else if *item > current {
                break;
            }
            matches += 1;
        }
        total += (matches * current)*iters;
        iters = 1
    }

    total
}

fn solve_1() -> u32 {
    let (mut list_one, mut list_two) = parse_file(INPUT);

    list_one.sort();
    list_two.sort();


    let mut list_two_iter = list_two.into_iter();
    let mut dist: u32 = 0;
    for val1 in list_one {
        dist += u32::abs_diff(val1, list_two_iter.next().unwrap());
    }
    dist
}

fn main()
{
    println!("{}", solve_1());
    println!("{}", solve_2());
}
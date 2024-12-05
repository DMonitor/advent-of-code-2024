const INPUT: &str = include_str!("../input");

fn main() {
    println!("{}",solve_1(INPUT));
    println!("{}",solve_2(INPUT));
}

fn solve_2(input: &str) -> u32 {
    /* 
    let input = concat!(
        "..X...\n",
        ".SAMX.\n",
        ".A..A.\n",
        "XMAS.S\n",
        ".X....\n");
    */
    
    /*
    let input = concat!("MMMSXXMASM\n",
                        "MSAMXMSMSA\n",
                        "AMXSXMAAMM\n",
                        "MSAMASMSMX\n",
                        "XMASAMXAMM\n",
                        "XXAMMXXAMA\n",
                        "SMSMSASXSS\n",
                        "SAXAMASAAA\n",
                        "MAMMMXMMMM\n",
                        "MXMXAXMASX");
    */
    //println!("{}",input);
    let mut tally = 0;
    let puzzle: Vec<Vec<char>> = input
                                .lines()
                                .map(|line| line.chars().collect())
                                .collect();
    for row in 1..puzzle.len()-1 {
        for col in 1..puzzle[row].len()-1 {
            if puzzle[row][col] == 'A' {
                if check2(row,col,&puzzle) {
                    tally += 1;
                }
            }
        }
    }
    tally
}

fn solve_1(input: &str) -> u32 {
    /*
    let input = concat!(
        "..X...\n",
        ".SAMX.\n",
        ".A..A.\n",
        "XMAS.S\n",
        ".X....\n");
    */

    /*
    let input = concat!("MMMSXXMASM\n",
                        "MSAMXMSMSA\n",
                        "AMXSXMAAMM\n",
                        "MSAMASMSMX\n",
                        "XMASAMXAMM\n",
                        "XXAMMXXAMA\n",
                        "SMSMSASXSS\n",
                        "SAXAMASAAA\n",
                        "MAMMMXMMMM\n",
                        "MXMXAXMASX");
    */
    //println!("{}",input);
    let mut tally:u32 = 0;
    let puzzle: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    for row in 0..puzzle.len() {
        for col in 0..puzzle[row].len() {
            tally += check1(row,col,&puzzle);
        }
    }
    tally
}

fn check1(row:usize, col:usize, puzzle:&Vec<Vec<char>>) -> u32
{
    let mut tally = 0;
    if check_v(row, col, &puzzle) {tally += 1;}
    if check_h(row, col, &puzzle) {tally += 1;}
    if check_45(row, col, &puzzle) {tally += 1;}
    if check_315(row, col, &puzzle) {tally += 1;}
    tally
}
fn check_v(row:usize, col:usize, puzzle:&Vec<Vec<char>>) -> bool
{
    if row < 3 {return false};
    (puzzle[row][col] == 'X' && puzzle[row-1][col] == 'M' && puzzle[row-2][col] == 'A' && puzzle[row-3][col] == 'S') ||
        (puzzle[row][col] == 'S' && puzzle[row-1][col] == 'A' && puzzle[row-2][col] == 'M' && puzzle[row-3][col] == 'X')
}


fn check_h(row:usize, col:usize, puzzle:&Vec<Vec<char>>) -> bool
{
    if col < 3 {return false};
    (puzzle[row][col] == 'X' && puzzle[row][col-1] == 'M' && puzzle[row][col-2] == 'A' && puzzle[row][col-3] == 'S') ||
        (puzzle[row][col] == 'S' && puzzle[row][col-1] == 'A' && puzzle[row][col-2] == 'M' && puzzle[row][col-3] == 'X')
}

fn check_315(row:usize, col:usize, puzzle:&Vec<Vec<char>>) -> bool
{
    if row < 3 {return false};
    if col < 3 {return false};
    (puzzle[row][col] == 'X' && puzzle[row-1][col-1] == 'M' && puzzle[row-2][col-2] == 'A' && puzzle[row-3][col-3] == 'S') ||
        (puzzle[row][col] == 'S' && puzzle[row-1][col-1] == 'A' && puzzle[row-2][col-2] == 'M' && puzzle[row-3][col-3] == 'X')
}

fn check_45(row:usize, col:usize, puzzle:&Vec<Vec<char>>) -> bool
{
    if row < 3 {return false};
    if col > puzzle[0].len() - 4 {return false};
    (puzzle[row][col] == 'X' && puzzle[row-1][col+1] == 'M' && puzzle[row-2][col+2] == 'A' && puzzle[row-3][col+3] == 'S') ||
        (puzzle[row][col] == 'S' && puzzle[row-1][col+1] == 'A' && puzzle[row-2][col+2] == 'M' && puzzle[row-3][col+3] == 'X')
}

fn check2(r:usize, c:usize, p:&Vec<Vec<char>>) -> bool
{
    let nw = p[r-1][c-1];
    let ne = p[r-1][c+1];
    let sw = p[r+1][c-1];
    let se = p[r+1][c+1]; 
    if nw == 'M' {
        if sw == 'M' {
            /*
            M S
             A
            M S 
            */
            return ne == 'S' &&  se == 'S'
        }
        else if sw == 'S' {
            /*
            M M
             A
            S S 
            */
            return ne == 'M' && se == 'S';
        }
    }
    else if nw == 'S' {
        if sw == 'S' {
            /*
            S M
             A
            S M 
            */
            return ne == 'M' &&  se == 'M'
        }
        else if sw == 'M' {
            /*
            S S
             A
            M M 
            */
            return ne == 'S' && se == 'M';
        }
    }
    false
}


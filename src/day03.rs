pub fn part1(input: String){
    let res = input
        .lines()
        .fold((0,0), |(mut count, pos), line|{
            let x = line.chars().nth(pos).unwrap();
            count += if x == '#' {1} else {0};
            return (count, (pos + 3) % 31)
        });

    println!("{:?}", res);

}

pub fn part2(input: String){
    let mut product: usize = 1;
    for (right, down) in [(1,1), (3,1), (5,1), (7,1), (1,2)].iter(){
        product *= check_slope(&input, (*right as usize,*down as usize)).0;
    }

    println!("{}", product);
}

pub fn check_slope(input: &String, step: (usize,usize)) -> (usize,usize){
    let res: (usize,usize) = input
        .lines()
        .step_by(step.1)
        .fold((0,0), |(mut count, pos), line|{
            let x = line.chars().nth(pos).unwrap();
            count += if x == '#' {1} else {0};
            return (count, (pos + step.0) % 31)
        });
    println!("{:?}", res);
    return res;
}
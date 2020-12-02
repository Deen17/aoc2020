pub fn part1(input: String){
    let iter = input
        .lines()
        .map(|line| line.split_ascii_whitespace());

    let mut count = 0;
    for mut line in iter{
        let policy = line.next();
        let c = line.next().unwrap().chars().nth(0).unwrap();
        let pass = line.next();
        // println!("{:?} {:?} {:?}", policy,c,pass);
        let mut  p= policy.unwrap().split("-").take(2);
        let min:usize = p.next().unwrap().parse().unwrap();
        let max:usize = p.next().unwrap().parse().unwrap();
        if let Some(s) = pass{
            let num = s.matches(c).collect::<Vec<&str>>().len();
            count = if num >= min && num  <= max { count + 1} else {count};
        }
    }
    println!("count: {}", count);
}

pub fn part2(input: String){
    let iter = input
        .lines()
        .map(|line| line.split_ascii_whitespace());

    let mut count = 0;
    for mut line in iter{
        let policy = line.next();
        let c = line.next().unwrap().chars().nth(0).unwrap();
        let pass = line.next();
        let mut  p= policy.unwrap().split("-").take(2);
        let left:usize = p.next().unwrap().parse().unwrap();
        let right:usize = p.next().unwrap().parse().unwrap();
        if let Some(s) = pass{
            let first = s.chars().nth(left-1).unwrap();
            let second = s.chars().nth(right-1).unwrap();
            count = if (first == c) ^ (second == c) { count + 1} else {count};
        }
    }
    println!("count {}", count);
}
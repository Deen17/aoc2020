use std::collections::HashSet;
use std::iter::FromIterator;

pub fn part1(input: String){
    let res= input
        .lines()
        .map(|x| 
            x.
                chars()
                .fold(0, 
                    |y, item| (y<<1) + ((item == 'B' || item == 'R') as isize))
        )
        .max().unwrap();
    println!("max: {}", res);
}

pub fn part2(input: String){
    let mut seats : HashSet<isize>= HashSet::from_iter(64..897);
    let res= input
        .lines()
        .map(|x| 
            x.
                chars()
                .fold(0, 
                    |y, item| (y<<1) + ((item == 'B' || item == 'R') as isize))
        )
        .map(|x| seats.remove(&x));
    for _ in res{}
    println!("seats: {:?}", seats);
}
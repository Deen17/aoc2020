use std::fmt::Binary;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn part1(input: String){
    let res= input
        .lines()
        .map(|x| 
                isize::from_str_radix(x.chars().map(|y| if y == 'F' || y == 'L' {'0'} else {'1'}).collect::<String>().as_str(),2).unwrap(),
        )
        .max().unwrap();
    println!("max: {}", res);
}

pub fn part2(input: String){
    let mut seats : HashSet<isize>= HashSet::from_iter(64..897);
    let res= input
        .lines()
        .map(|x| 
                isize::from_str_radix(x.chars().map(|y| if y == 'F' || y == 'L' {'0'} else {'1'}).collect::<String>().as_str(),2).unwrap())
        .map(|x| seats.remove(&x));
    for _ in res{}
    println!("seats: {:?}", seats);
}
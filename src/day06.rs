use std::collections::HashSet;

type CharSet<'a> = HashSet< char>;

pub fn part1(input: String){
    let res: usize= input
        .split("\n\n")
        .map(|x| 
            x
                .chars()
                .fold(HashSet::new(), 
                    |mut y, item| {if item.is_alphabetic(){ y.insert(item);} y})
                .len()
        )
        .sum();
    println!("sum: {}", res);
}

pub fn part2(input: String){
    let res: usize= input
        .split("\n\n")
        .map(|x: &str| 
            x
                .lines()
                .fold((CharSet::new(), 0), 
                    |(y, count), item|
                        {
                            // let mut this = HashSet::from_iter(item.chars());
                            let mut set: HashSet<char> = HashSet::new();
                            for i in item.chars(){
                                set.insert(i);
                            }
                            set.retain(|x| x.is_alphabetic());
                            if count == 0{
                                return (set, count+1)
                            }
                            else {
                                (y.intersection(&set).map(|x| x.to_owned()).collect(),count+1)
                            }
                            
                        }
                ).0
                .len()
        )
        .sum();
    println!("sum: {}", res);
}
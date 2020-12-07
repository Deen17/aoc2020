
use std::collections::HashMap;

fn parse_string(input: &String)-> HashMap<&str, Vec<(usize, &str)>>{
    let map: HashMap<&str, Vec<(usize, &str)>>= input
        .lines()
        .fold(HashMap::new(), |mut map, x: &str| {
            let mut it =  x.split(" bags contain");
            let container =it.next().unwrap();
            let contained = it.next().unwrap().split(",");
            let mut v: Vec<(usize, &str)> = vec![];
            for i in contained{
                
                let trimmed = i
                    .trim_end_matches(" bags")
                    .trim_end_matches(" bag.")
                    .trim_end_matches(" bags.")
                    .trim_end_matches(" bag")
                    .trim_start();
                if trimmed.chars().nth(0).unwrap().is_numeric(){
                    let num = trimmed.chars().nth(0).unwrap().to_digit(10).unwrap() as usize;
                    let (_, bag) = trimmed.split_at(2);
                    v.push((num, bag));
                }
            }
            map.insert(container, v);
            map
        });
    map
}

pub fn part1(input: String){
    let map = parse_string(&input);
    let mut count = map.keys().filter(|b| can_contain_gold(&map, b)).count();
    if map.contains_key(&"shiny gold") {count -= 1;}
    println!("count: {}", count);
}

fn can_contain_gold(map: &HashMap<&str, Vec<(usize, &str)>>, bag: &str) -> bool{
    (bag == "shiny gold") || map.get(&bag).unwrap().iter().any(|(_,x)|  can_contain_gold(map, x))
}

fn number_of_bags(map: &HashMap<&str, Vec<(usize, &str)>>, bag: &str) -> usize{
    if map.get(&bag).unwrap().is_empty(){return 1}
    map.get(&bag).unwrap().iter()
        .map(|(num, name)| (num * number_of_bags(&map,name)) )
        .sum::<usize>() + 1
}

pub fn part2(input: String){
    let map = parse_string(&input);
    println!("total bags: {}", number_of_bags(&map, "shiny gold")-1);
}

#[test]
fn test() {
    let input: String = String::from(
    "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.");
    let map = parse_string(&input);
    // println!("map: {:?}", map);
    assert_eq!(number_of_bags(&map, "dark violet") - 1, 0);
    println!("");
    assert_eq!(number_of_bags(&map, "dark blue") - 1, 2);
    println!("");
    assert_eq!(number_of_bags(&map, "shiny gold") -1 , 126);
    println!("");

    let input = String::from(
"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
    );
    let map = parse_string(&input);
    println!("map: {:?}", map);
    assert_eq!(number_of_bags(&map, "dotted black")-1, 0);
    assert_eq!(number_of_bags(&map, "vibrant plum")-1, 11);
    assert_eq!(number_of_bags(&map, "shiny gold")-1, 32);
}
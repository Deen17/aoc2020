use std::collections::HashSet;


pub fn part1(input: String){
    let mut set: HashSet<usize> = HashSet::new();
    let list: Vec<(&str, &str)> = input
        .lines()
        .map(|x| x.split_ascii_whitespace())
        .map(|mut it| (it.next().unwrap(), it.next().unwrap()))
        .collect();
    // println!("count: {:#?}", list);
    let mut line = 0isize;
    let mut acc = 0isize;
    while !set.contains(&(line as usize)){
        set.insert(line as usize);
        match list[line as usize]{
            ("acc", num) => {
                let operand: isize = num.parse().unwrap();
                acc += operand;
                line += 1;
            },
            ("nop", _num) => {
                line+=1;
            },
            ("jmp", num) => {
                let operand: isize = num.parse().unwrap();
                line += operand;
            },
            _=> panic!("shouldnt be here")
        };
        // println!("line {}, acc: {}", line, acc);
    }
    println!("acc {}", acc);
}

pub fn part2(input: String){
    let mut set: HashSet<usize> = HashSet::new();
    let list: Vec<(&str, &str)> = input
        .lines()
        .map(|x| x.split_ascii_whitespace())
        .map(|mut it| (it.next().unwrap(), it.next().unwrap()))
        .collect();
    set.insert(624);
    let mut line = 0isize;
    let mut acc = 0isize;
    let mut fix = 0;
    while line != 624{
        let mut copied_set = set.clone();
        line = 0;
        acc = 0;
        if let ("acc", _) = list[fix]{
            fix += 1;
            continue;
        } 
        while !copied_set.contains(&(line as usize)){
            copied_set.insert(line as usize);
            let mut inst =list[line as usize].clone();
            if line as usize == fix{
                inst = match inst {
                    ("nop", num) => ("jmp", num),
                    ("jmp", num)=> ("nop", num),
                    _ => panic!("should not be here 2")
                };
            }
            
            match inst{
                ("acc", num) => {
                    let operand: isize = num.parse().unwrap();
                    acc += operand;
                    line += 1;
                },
                ("nop", _num) => {
                    line+=1;
                },
                ("jmp", num) => {
                    let operand: isize = num.parse().unwrap();
                    line += operand;
                },
                _=> panic!("shouldnt be here")
            };
            // println!("fix: {} , line: {}, acc: {}",fix, line, acc);
        }
        fix += 1;
    }
    
    println!("acc {}", acc);
}

#[test]
fn test() {

}
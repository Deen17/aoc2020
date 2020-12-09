use std::collections::VecDeque;


pub fn part1(input: String){
    let mut dq: VecDeque<usize> = VecDeque::new();
    let _res = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .any(|x| {
            if dq.len() < 25 {
                dq.push_back(x);
                return false
            }

            let mut res = false;
            dq.make_contiguous();
            for (i,item) in dq.iter().enumerate(){
                for other in dq.as_slices().0[i..].iter(){
                    res = item + other == x;
                    if res {break}
                }
                if res{break}
            }
            // println!("num: {},\tdq {:?}", x,dq);
            dq.pop_front();
            dq.push_back(x);
            
            if !res{println!("{}", x);}
            !res
        });

}

fn find_range(list: &Vec<usize>, target: usize) -> (usize,usize){
    for start in 0..1000{
        let mut sum = 0;
        for i in start..1000{
            sum += list[i];
            if sum == target{
                return (start, i)
            }
            else if sum > target {
                break;
            }
        }
    }
    panic!("could not find range");
}

pub fn part2(input: String){
    let target:usize = 133015568;
    let list : Vec<usize> = input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    let (start,end) = find_range(&list,target);
    // println!("{} {} ", start,end);
    let minimum = list[start..end].iter().min().unwrap();
    let maximum = list[start..end].iter().max().unwrap();

    println!("{} {}", minimum, maximum);
    println!("{}", minimum+maximum);

}

#[test]
fn test() {

}
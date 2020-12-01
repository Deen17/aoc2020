
pub fn part1(inp: String) {
    let mut sorted: Vec<u32> = 
        inp
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
    sorted.sort();
    
    let total: u32 = 2020;
    
    for num in sorted.iter(){
        let desired_num = total - num;
        if let Ok(_pos) = sorted.binary_search(&desired_num){
            println!("{}", num * desired_num);
            break;
        }
    }
}

pub fn part2(inp: String){
    let mut sorted: Vec<u32> = 
        inp
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
    sorted.sort();

    let total: u32 = 2020;
    
    for (i,first) in (&sorted).iter().enumerate(){
        let desired_num = total - first;
        let (_left,right) = sorted.split_at(i+1);

        for second in (right).iter(){
            
            if first + second + second > total{
                continue;
            }
            let third = desired_num - second;

            if let Ok(_pos) = right.binary_search(&third){
                println!("{}, {}, {}", first, second, third);
                println!("{}", first * second * third);
                return;
            }
        }

    }
}
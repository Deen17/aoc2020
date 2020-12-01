
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

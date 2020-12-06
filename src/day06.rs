
pub fn part1(input: String){
    let res: usize= input
        .split("\n\n")
        .map(|x| 
            x
                .chars()
                .fold(0u32, 
                    |y, item| if item.is_alphabetic() {y | (1u32 << (item as usize - 96))} else {y})
                .count_ones() as usize
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
                .fold(0x3FF_FFFFu32, 
                    |set, item|
                        {
                            let mut cur = 0u32;
                            for i in item.chars(){
                                if i.is_alphabetic(){
                                    cur |= 1u32 << (i as usize - 96);
                                }
                            }
                            set & cur
                        }
                )
                .count_ones() as usize
        )
        .sum();
    println!("sum: {}", res);
}
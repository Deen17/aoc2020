use regex::Regex;


pub fn part1(input: String){
    // let patterns = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "cid", "hgt"];
    let patterns = Regex::new(r"\b(ecl|pid|eyr|hcl|byr|iyr|hgt)").unwrap();

    let res= input
        .split("\n\n")
        .filter(|entry| patterns.find_iter(entry).collect::<Vec<_>>().len() == 7)
        // .map(|x| println!("\n\npass: {}", x))
        .count();
    println!("valids: {}", res);
}

pub fn part2(input: String){
    // let patterns = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "cid", "hgt"];
    let patterns = Regex::new(r"\b(ecl|pid|eyr|hcl|byr|iyr|hgt)").unwrap();
    let byr = Regex::new(r"byr:(?P<birthyear>\d{4}\b)").unwrap();
    let iyr = Regex::new(r"iyr:(?P<issueyear>\d{4}\b)").unwrap();
    let eyr = Regex::new(r"eyr:(?P<expirationyear>\d{4}\b)").unwrap();
    let hgt = Regex::new(r"hgt:(?P<height>\d+(in|cm))").unwrap();
    let ecl = Regex::new(r"ecl:(?P<eyecolor>(amb|blu|brn|gry|grn|hzl|oth))").unwrap();
    let hcl = Regex::new(r"hcl:(?P<haircolor>#[0-9a-f]{6})").unwrap();
    let pid = Regex::new(r"pid:(?P<pid>\d{9}\b)").unwrap();

    let res= input
        .split("\n\n")
        .filter(|entry: &&str| {
            if patterns.find_iter(entry).count() != 7{
                return false
            }

            let birthyear = byr.captures(entry).unwrap().name("birthyear").map_or("0", |x| x.as_str()).parse::<usize>().unwrap();
            if !(birthyear <= 2002 && birthyear >= 1920){
                return false
            }

            let issueyear = iyr.captures(entry).unwrap().name("issueyear").map_or("0", |x| x.as_str()).parse::<usize>().unwrap();
            if !(issueyear <= 2020 && issueyear >= 2010){
                return false
            }

            let expiration = eyr.captures(entry).unwrap().name("expirationyear").map_or("0", |x| x.as_str()).parse::<usize>().unwrap();
            if !(expiration <= 2030 && expiration >= 2020){
                return false
            }

            if hgt.captures(entry).is_none(){
                return false
            }

            let height = hgt
                .captures(entry)
                .unwrap()
                .name("height")
                .map_or("0", |x| x.as_str());
            // println!("{}",&height.chars().rev().collect::<String>()[0..2]);
            match &height.chars().rev().collect::<String>()[0..2]{
                "ni" =>{
                    let x = height[0..height.len()-2].parse::<usize>().unwrap();
                    if !(x <= 76 && x >= 59){
                        return false
                    } 
                },
                "mc" => {
                    let x = height[0..height.len()-2].parse::<usize>().unwrap();
                    if !(x <= 193 && x >= 150){
                        return false
                    } 
                },
                _=>{return false}
            }

            if ecl.captures(entry).is_none(){
                return false
            }

            if ecl.captures(entry).unwrap().name("eyecolor").is_none(){
                return false
            }

            if hcl.captures(entry).is_none(){
                return false
            }

            if hcl.captures(entry).unwrap().name("haircolor").is_none(){
                return false
            }

            if pid.captures(entry).is_none(){
                return false
            }

            if pid.captures(entry).unwrap().name("pid").is_none(){
                return false
            }

            true
        })
        .count();
    println!("res: {}", res);
}
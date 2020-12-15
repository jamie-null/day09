
#![warn (
    clippy::all,
)]

use std::error::Error;
use std::fs::File;
use std::io::{BufReader,BufRead};
use std::cmp;

fn main() -> Result<(), Box<dyn Error>> {
    let raw = File::open("./input.txt")?;
    let buf = BufReader::new(raw);

    //let mut curr: usize = 0;
    let mut nums = Vec::new();
    for line  in buf.lines() {
        let line = line.unwrap();
        let num = line.parse::<usize>()?;
        nums.push(num);
        /*
        if curr + 1 > 25 {
            let mut first = 0;
            let mut found = false;
            while first < 25 && !found {
                first += 1;
                let mut second = 0;
                while second < 25 {
                    second += 1;
                    if nums[curr-first] + nums[curr-second] == nums[curr] {
                        found = true;
                        break;
                    }
                    println!("{},{} at {}",first,second,curr);
                }
            }
            if !found {
                println!("{} at {}",nums[curr],curr);
                break;
            }
        }
        curr += 1;
        */
    }
    /*
    //393911906
    let mut first = 0;
    let mut found = false;
    while first < nums.len() {
        let mut second = first + 1;
        while second <= nums.len() {
            let mut sum = 0;
            for i in first..second {
                sum += nums[i];
            }
            if sum == 393911906 {
                found = true;
                break;
            }
            second += 1;
        }
        if found {
            println!("{},{}",first,second);
            break;
        }
        first += 1;
    }
     */
    let mut min = std::usize::MAX;
    let mut max = 0;
    for i in 517..534 {
        min = cmp::min(min,nums[i]);
        max = cmp::max(max,nums[i]);
    }

    println!("{}",min + max);

    Ok(())
}

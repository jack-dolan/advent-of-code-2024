use std::fs;
use regex::Regex;

pub fn main() {
    // let data = fs::read_to_string("03/input-test.txt").expect("Unable to read file");
    // let data = fs::read_to_string("03/input-test-2.txt").expect("Unable to read file");
    let data = fs::read_to_string("03/input.txt").expect("Unable to read file");

    println!("{}", data);

    let re = Regex::new(r"mul\(\d+,\d+\)|don't\(\)|do\(\)").unwrap();
    let mul_results: Vec<&str> = re.find_iter(&data).map(|x| x.as_str()).collect();
    println!("{:?}", mul_results);
    
    let mut product_sum = 0;

    let mut enabled = true;
    for mul in mul_results {
      // if the string = "do()" then set enabled to true
      if mul == "do()" {
        enabled = true;
      }

      // if the string = "don't()" then set enabled to false
      else if mul == "don't()" {
        enabled = false;
      }

      // if the string isn't "do()" or "don't()" AND enabled = true, then do the same thing as before
      else if enabled {
        let re = Regex::new(r"mul\((.*?),").unwrap();
        let first_num = re.captures(mul).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();

        let re = Regex::new(r",(.*?)\)").unwrap();
        let second_num = re.captures(mul).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();

        product_sum += first_num * second_num;
      }
      
    }
    
    println!("Product sum: {}", product_sum)
}
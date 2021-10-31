use std::io;

fn main() {
    //输入部分
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input:String =  input.trim().parse()
        .expect("0");
    //计算部分

    //加法
    if input.contains("+"){
        let num1: Vec<&str> = input.rsplit("+").collect();
        let num2: Vec<&str> = input.split("+").collect();
        let num1 = num1[num1.len()-1].to_owned();
        let num1:f64 = num1.parse().unwrap();
        let num2 = num2[num2.len()-1].to_owned();
        let num2:f64= num2.parse().unwrap();
        println!("{}",num1+num2); }
    //减法
    else  if input.contains("-"){
        let num1: Vec<&str> = input.rsplit("-").collect();
        let num2: Vec<&str> = input.split("-").collect();
        let num1 = num1[num1.len()-1].to_owned();
        let num1:f64 = num1.parse().unwrap();
        let num2 = num2[num2.len()-1].to_owned();
        let num2:f64= num2.parse().unwrap();
        println!("{}",num1-num2); }
    //乘法
    if input.contains("*"){
        let num1: Vec<&str> = input.rsplit("*").collect();
        let num2: Vec<&str> = input.split("*").collect();
        let num1 = num1[num1.len()-1].to_owned();
        let num1:f64 = num1.parse().unwrap();
        let num2 = num2[num2.len()-1].to_owned();
        let num2:f64= num2.parse().unwrap();
        println!("{}",num1*num2); }
    //除法
    if input.contains("/"){
        let num1: Vec<&str> = input.rsplit("/").collect();
        let num2: Vec<&str> = input.split("/").collect();
        let num1 = num1[num1.len()-1].to_owned();
        let num1:f64 = num1.parse().unwrap();
        let num2 = num2[num2.len()-1].to_owned();
        let num2:f64= num2.parse().unwrap();
        println!("{}",num1/num2); }

    }



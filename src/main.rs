#![allow(non_snake_case)]
use std::process::Command;


fn makeString(stdout: &Vec<u8>) -> String {
    // executing a command returns an array of u8 instead of chars, this simply turns them into
    // chars and then returns all of them as a string
    let string =  String::from_utf8_lossy(stdout).to_string();
    return string
}

fn main() {
    let commandOut = Command::new("sensors").output().expect("Failed to execute command");
    let output = makeString(&commandOut.stdout).replace("\n", "||");

    let lines: Vec<&str> = output.split("||").collect();

    let mut temp = lines[8].to_string();

    temp = temp.replace("Tdie:         ", "");
    println!("{}", temp);
}

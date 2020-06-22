#![allow(non_snake_case)]
use std::process::Command;


// this returns an index if the specified interface is found
fn findIndex(list: &Vec<&str>, substring: &String) -> Result<usize, String> {
    for i in 0..list.len() {
        if list[i].contains(substring) {
            return Ok(i);
        }
    }
    return Err(String::from("Could not find interface"));
}


fn makeString(stdout: &Vec<u8>) -> String {
    // executing a command returns an array of u8 instead of chars, this simply turns them into
    // chars and then returns all of them as a string
    let string =  String::from_utf8_lossy(stdout).to_string();
    return string
}



fn main() {
    let command = Command::new("sensors").arg("-u").output().expect("Failed to execute command");
    let command = makeString(&command.stdout);

    let lines: Vec<&str> = command.split("\n").collect();

    let reading = lines[findIndex(&lines, &String::from("Tdie:")).unwrap_or(00) + 1].to_string();
    let reading: Vec<&str> = reading.split(":").collect();

    let mut temp: String = reading[1].to_string();
    temp = temp.replace(" ", "");
    temp.pop();
    temp.pop();
    println!("{} °C", temp);
}

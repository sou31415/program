use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::{env, fs::File, fs::OpenOptions};

#[allow(non_snake_case)]
fn main() {
    let TodoFile = "/Users/sotarofurukawa/.config/todo.txt";
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("[Usage] todo [Option] [Tag] | [TaskName]");
        println!("-add      :       Add Tasks");
        println!("-search   :       Search Data or Tasks");
        println!("-see      :       See All Tasks");
        return;
    }
    if args[1] == "-search" {
        let data = &args[2];
        let fp = File::open(TodoFile).unwrap();
        let reader = BufReader::new(fp);
        for line in reader.lines() {
            let line = line.unwrap();
            if line.find(data) == None {
                continue;
            } else {
                println!("{}", line);
            }
        }
    } else if args[1] == "-add" {
        let TaskName = &args[2].as_bytes();
        let fp = OpenOptions::new().append(true).open(TodoFile).unwrap();
        let mut writer = BufWriter::new(fp);
        writer.write(TaskName).unwrap();
        writer.write(b"\n").unwrap();
    } else if args[1] == "-see" {
        println!("{}", fs::read_to_string(TodoFile).unwrap());
    }
}

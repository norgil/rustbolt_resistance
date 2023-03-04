use std::{
    collections::HashSet,
    io::{BufRead},
};
// use linecount::count_lines;

fn main() {

    let args = std::env::args().collect::<Vec<String>>();

    let file1 = &args[1];
    let file2 = &args[2];

    let file1_contents = std::fs::File::open(file1).unwrap();
    let file2_contents = std::fs::File::open(file2).unwrap();

/*
    let file1_lines = count_lines(&file1_contents).unwrap();
    let file2_lines = count_lines(&file2_contents).unwrap();

    if &file1_lines == &file2_lines {
        println!("Files have equal number of lines!")
    }
    else {
        println!("Files have different number of lines!")
    }

    println!("Total number of lines in {} is: {}",&file1,&file1_lines);
    println!("Total number of lines in {} is: {}",&file2,&file2_lines);

    let maxlines: usize;
    if file1_lines > file2_lines {
        maxlines = file1_lines;
    }
    else {
        maxlines = file2_lines;
    }

    println!("Longest file is {} lines", maxlines);
*/
    let file1_reader = std::io::BufReader::new(file1_contents);

    let mut file1_lines = HashSet::new();

    for line in file1_reader.lines() {
       file1_lines.insert(line.unwrap());
    }

    let file2_reader = std::io::BufReader::new(file2_contents);

    let mut file2_lines = HashSet::new();

    for line in file2_reader.lines() {
       file2_lines.insert(line.unwrap());
    }

    for x in file1_lines.difference(&file2_lines) {
        println!("{x}");
    }  
}

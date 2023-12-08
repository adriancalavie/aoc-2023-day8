use std::fs;

struct Node {
    id: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(id: String, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Node {
        Node { id, left, right }
    }
}

fn read_lines(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Couldn't read input");
    let lines = content
        .lines()
        .map(|s| -> String { s.trim().to_string() })
        .collect();

    lines
}

fn main() {
    // let lines = read_lines("res/data.txt");
    // let lines = read_lines("res/data_mild.txt");
    let lines = read_lines("res/data_light.txt");

    for line in lines {
        println!("{}", line);
    }
}

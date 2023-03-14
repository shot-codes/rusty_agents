use agent::Agent;
use color::Color;
use matrix::Matrix;
use state::State;
use std::cmp;
use std::io::{BufRead, BufReader, Stdin};
#[allow(unused_imports)]
use std::str::FromStr;

mod action;
mod agent;
mod color;
mod matrix;
mod state;

pub fn parse_level<'a>(mut server_messages: BufReader<Stdin>) -> State<'a> {
    // Read frontmatter - #domain, hopsital, #levelname, <name>, #colors
    for _ in 0..5 {
        server_messages.read_line(&mut String::new()).unwrap();
    }

    let mut buf = String::new();

    // Read colors
    let mut agent_colors: [Option<Color>; 10] = [None; 10];
    let mut box_colors: [Option<Color>; 26] = [None; 26];
    loop {
        buf.clear();
        server_messages.read_line(&mut buf).unwrap();
        if buf.starts_with("#") {
            break;
        };
        let parts: Vec<&str> = buf.split(":").collect();
        let color = Color::from_str(parts[0].trim()).unwrap();
        let entities: Vec<&str> = parts[1].split(",").map(|s| s.trim()).collect();
        for entity in entities {
            let c = entity.chars().next().unwrap();
            if c.is_digit(10) {
                let index = (c as u32 - '0' as u32) as usize;
                agent_colors[index] = Some(color);
            } else if c.is_uppercase() {
                let index = (c as u32 - 'A' as u32) as usize;
                box_colors[index] = Some(color);
            }
        }
    }

    // Read level
    let mut agents: Vec<Agent> = Vec::new();
    let mut goals: Vec<char> = Vec::new();

    let mut raw_level: Vec<String> = Vec::new();
    let mut num_rows = 0;
    let mut num_cols = 0;
    loop {
        buf.clear();
        server_messages.read_line(&mut buf).unwrap();
        if buf.starts_with("#") {
            break;
        };
        let stripped_buf = buf
            .strip_suffix("\r\n")
            .or(buf.strip_suffix("\n"))
            .unwrap_or(buf.as_str());
        raw_level.push(stripped_buf.to_owned());
        num_cols = cmp::max(num_cols, stripped_buf.len());
        num_rows += 1;
    }

    let mut level = Matrix::new(num_rows, num_cols, ' ');
    for (i, string) in raw_level.iter().enumerate() {
        for (j, character) in string.chars().enumerate() {
            level.set(i, j, character);
        }
    }

    // Read agents, walls, and boxes
    let mut walls: Matrix<bool> = Matrix::new(1, 1, false);
    let mut boxes: Matrix<char> = Matrix::new(1, 1, ' ');

    for row in 0..level.rows {
        for col in 0..level.cols {
            let c = level.get(row, col);
            if '0' <= c && c <= '9' {
                agents.push(Agent {
                    row,
                    col,
                    color: agent_colors[c.to_string().parse::<usize>().unwrap()].unwrap(),
                })
            } else if 'A' <= c && c <= 'Z' {
            }
        }
    }

    agents = dbg!(agents);
    level.print_grid();

    State {
        agents,
        goals,
        walls,
        boxes,
        parent: None,
        joint_action: Vec::new(),
    }
}

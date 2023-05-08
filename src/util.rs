use crate::core::host_file::HOST_FILE_PATH;
use comfy_table::modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS};
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use faccess::PathExt;
use miette::Result;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn table(header: Vec<Cell>, rows: Vec<Vec<String>>, colored: bool) -> Table {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .apply_modifier(UTF8_SOLID_INNER_BORDERS);
    if !header.is_empty() {
        table.set_header(header);
    }
    for row in rows {
        let colored_row: Vec<_> = row
            .iter()
            .map(|c| {
                if colored {
                    green_cell(c)
                } else {
                    plain_cell(c)
                }
            })
            .collect();
        table.add_row(colored_row);
    }
    table
}

pub fn green_cell(text: &str) -> Cell {
    Cell::new(text).fg(Color::Green)
}

pub fn blue_cell(on: &bool) -> Cell {
    if on == &true {
        Cell::new("true").fg(Color::Blue)
    } else {
        plain_cell("false")
    }
}

pub fn plain_cell(text: &str) -> Cell {
    Cell::new(text)
}

pub fn read_lines<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn escalate() -> Result<()> {
    let hosts_file = Path::new(HOST_FILE_PATH);
    if !hosts_file.writable() {
        sudo::escalate_if_needed().unwrap();
    }
    Ok(())
}

use super::Command;
use crate::core::{HostsMeta, SwhResponse, SWH_HTTP_API_URL};
use anyhow::{Ok, Result};
use async_trait::async_trait;
use clap::Args;
use comfy_table::modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS};
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

pub struct ListCommand;

#[derive(Args, Debug)]
pub struct List {
    pub state: Option<String>,
}

#[async_trait]
impl Command<List> for ListCommand {
    async fn execute(&self, _args: &List) -> Result<()> {
        let swh_api_list: String = format!("{}/api/list", SWH_HTTP_API_URL);
        let res = reqwest::get(swh_api_list)
            .await?
            .json::<SwhResponse>()
            .await
            .expect("Failed to call list api");
        let SwhResponse { success, data } = &res;
        if success == &true {
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .apply_modifier(UTF8_SOLID_INNER_BORDERS)
                .set_header(vec![
                    green_cell("#"),
                    green_cell("title"),
                    green_cell("id"),
                    green_cell("on"),
                ]);

            for (index, meta) in data.iter().enumerate() {
                let HostsMeta { id, title, on } = meta;
                match (title, on) {
                    (Some(title), Some(on)) => {
                        table.add_row(vec![
                            green_cell(&index.to_string()),
                            plain_cell(title),
                            plain_cell(id),
                            blue_cell(on),
                        ]);
                    }
                    _ => (),
                }
            }
            println!("{table}");
        }

        Ok(())
    }
}

fn green_cell(text: &str) -> Cell {
    Cell::new(text).fg(Color::Green)
}

fn blue_cell(on: &bool) -> Cell {
    if on == &true {
        Cell::new("true").fg(Color::Blue)
    } else {
        plain_cell("false")
    }
}

fn plain_cell(text: &str) -> Cell {
    Cell::new(text)
}

use num_format::{Locale, ToFormattedString};
use prettytable::{format, Cell, Row, Table};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut table = Table::new();

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table.add_row(Row::new(vec![Cell::new("| Power | Number | Bitshift |")]));
    table.add_row(Row::new(vec![Cell::new("| --- | --- | --- |")]));

    let powers: Vec<(u32, u128)> = (4..=127).map(|x| (x, 2u128.pow(x))).collect();

    if args.len() <= 1 {
        let mut is_blue = true;
        for (n, val) in powers.iter() {
            let font_color = if is_blue { "\x1b[34m" } else { "\x1b[33m" };
            let row = Row::new(vec![Cell::new(&format!(
                "{}| 2^{} | {} | 1u << {}u |\x1b[0m",
                font_color,
                n,
                val.to_formatted_string(&Locale::en),
                n
            ))]);
            table.add_row(row);
            is_blue = !is_blue;
        }
    } else {
        let numeric_input: String = args[1].chars().filter(|c| c.is_digit(10)).collect();

        if let Ok(target) = numeric_input.parse::<usize>() {
            let mut is_blue = true;

            for (n, val) in powers.iter() {
                if *val < target as u128 {
                    continue;
                }
                if let Some(previous) = powers.iter().find(|&&(prev_n, _)| prev_n == n - 1) {
                    let font_color = if is_blue { "\x1b[34m" } else { "\x1b[33m" };
                    let row = Row::new(vec![Cell::new(&format!(
                        "{}| 2^{} | {} | 1u << {}u |\x1b[0m",
                        font_color,
                        previous.0,
                        previous.1.to_formatted_string(&Locale::en),
                        previous.0
                    ))]);
                    table.add_row(row);
                    is_blue = !is_blue;
                }

                if *val == target as u128 {
                    let row = Row::new(vec![Cell::new(&format!(
                        "\x1b[32m| **2^{}** | **{}** | **1u << {}u** |\x1b[0m",
                        n,
                        val.to_formatted_string(&Locale::en),
                        n
                    ))]);
                    table.add_row(row);
                } else {
                    let font_color = if is_blue { "\x1b[34m" } else { "\x1b[33m" };
                    let row = Row::new(vec![Cell::new(&format!(
                        "{}| 2^{} | {} | 1u << {}u |\x1b[0m",
                        font_color,
                        n,
                        val.to_formatted_string(&Locale::en),
                        n
                    ))]);
                    table.add_row(row);
                    is_blue = !is_blue;
                }

                if let Some(next) = powers.iter().find(|&&(next_n, _)| next_n == n + 1) {
                    let font_color = if is_blue { "\x1b[34m" } else { "\x1b[33m" };
                    let row = Row::new(vec![Cell::new(&format!(
                        "{}| 2^{} | {} | 1u << {}u |\x1b[0m",
                        font_color,
                        next.0,
                        next.1.to_formatted_string(&Locale::en),
                        next.0
                    ))]);
                    table.add_row(row);
                }

                break;
            }
        } else {
            println!("Invalid number provided.");
        }
    }

    table.printstd();
}

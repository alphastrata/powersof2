use num_format::{Locale, ToFormattedString};
use prettytable::{format, Cell, Row, Table};

fn main() {
    let mut table = Table::new();

    // Set table format to match Markdown table format with proper spacing
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    // Header row for the Markdown table
    table.add_row(Row::new(vec![Cell::new("| Power | Number | Bitshift |")]));
    table.add_row(Row::new(vec![Cell::new("| --- | --- | --- |")]));

    let powers: Vec<(u32, u64)> = (4..=63).map(|x| (x, 2u64.pow(x))).collect();

    for (n, val) in powers.iter() {
        let row = Row::new(vec![Cell::new(&format!(
            "| 2^{} | {} | 1u << {}u |",
            n,
            val.to_formatted_string(&Locale::en),
            n
        ))]);

        table.add_row(row);
    }

    // Print the table in Markdown format
    table.printstd();
}

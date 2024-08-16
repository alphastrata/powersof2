use num_format::{Locale, ToFormattedString};
use prettytable::{color, format, Attr, Cell, Row, Table};

fn main() {
    let mut table = Table::new();

    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    let lower_powers: Vec<(u32, u64)> = (4..=31).map(|x| (x, 2u64.pow(x))).collect();
    let upper_powers: Vec<(u32, u64)> = (32..=63).map(|x| (x, 2u64.pow(x))).collect();

    for (i, _) in lower_powers.iter().enumerate() {
        let (n1, val1) = lower_powers[i];
        let (n2, val2) = upper_powers[i];

        let color_style = if i % 2 == 0 {
            color::BRIGHT_CYAN
        } else {
            color::BRIGHT_YELLOW
        };

        let row = Row::new(vec![
            Cell::new(&format!(
                "2^{} | {:<20}",
                n1,
                val1.to_formatted_string(&Locale::en)
            ))
            .with_style(Attr::ForegroundColor(color_style)),
            Cell::new(&format!(
                "2^{} | {:<20}",
                n2,
                val2.to_formatted_string(&Locale::en)
            ))
            .with_style(Attr::ForegroundColor(color_style)),
        ]);

        table.add_row(row);
    }

    table.printstd();
}

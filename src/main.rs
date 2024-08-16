use num_format::{Locale, ToFormattedString};
use prettytable::{Cell, Row, Table};

fn main() {
    let mut table = Table::new();

    // Collect powers of two with their exponents
    let lower_powers: Vec<(u32, u64)> = (4..=31).map(|x| (x, 2u64.pow(x))).collect();
    let upper_powers: Vec<(u32, u64)> = (32..=63).map(|x| (x, 2u64.pow(x))).collect();

    for i in 0..lower_powers.len() {
        let (n1, val1) = lower_powers[i];
        let (n2, val2) = upper_powers[i];

        table.add_row(Row::new(vec![
            Cell::new(&format!(
                "2^{} | {}",
                n1,
                val1.to_formatted_string(&Locale::en)
            )),
            Cell::new(&format!(
                "2^{} | {}",
                n2,
                val2.to_formatted_string(&Locale::en)
            )),
        ]));
    }

    table.printstd();
}

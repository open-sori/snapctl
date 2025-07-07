use std::cmp;

pub fn print_table(headers: Vec<&str>, data: Vec<Vec<String>>) {
    if data.is_empty() {
        println!("No data to display.");
        return;
    }

    let mut column_widths = headers.iter().map(|h| h.len()).collect::<Vec<usize>>();

    for row in &data {
        for (i, cell) in row.iter().enumerate() {
            if i < column_widths.len() {
                column_widths[i] = cmp::max(column_widths[i], cell.len());
            }
        }
    }

    let header_line = headers
        .iter()
        .enumerate()
        .map(|(i, h)| format!("{:<width$}", h, width = column_widths[i]))
        .collect::<Vec<String>>()
        .join("  ");

    println!("{}", header_line);

    for row in &data {
        let row_line = row
            .iter()
            .enumerate()
            .map(|(i, c)| format!("{:<width$}", c, width = column_widths[i]))
            .collect::<Vec<String>>()
            .join("  ");
        println!("{}", row_line);
    }
}

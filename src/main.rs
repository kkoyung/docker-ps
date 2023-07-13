fn main() {
    const FORMAT: &str = "table {{.ID}};{{.Image}};{{.Names}};{{.Status}};{{.Ports}}";
    const COLUMN: usize = 5;
    const PORTS_AT: usize = 4;

    use std::process::Command;
    let output = Command::new("docker")
        .arg("ps")
        .arg("-a")
        .arg("--format")
        .arg(FORMAT)
        .output()
        .expect("Failed to execute process");
    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut max_length: [usize; COLUMN] = [0; COLUMN];
    let mut table: Vec<Vec<&str>> = Vec::new();

    for line in stdout.split("\n") {
        if line == "" {
            break;
        }

        let mut cols: Vec<&str> = line.split(";").collect();
        assert_eq!(cols.len(), COLUMN, "Column number does not match: {}", line);

        let ports: Vec<&str> = cols[PORTS_AT].split(", ").collect();
        if ports.len() > 0 {
            cols[PORTS_AT] = ports[0];
            for i in 0..ports.len() {
                let length = ports[i].chars().count();
                max_length[PORTS_AT] = if length > max_length[PORTS_AT] {
                    length
                } else {
                    max_length[PORTS_AT]
                };
            }
        }

        for (i, col) in cols.iter().enumerate() {
            let length = col.chars().count();
            max_length[i] = if length > max_length[i] {
                length
            } else {
                max_length[i]
            };
        }

        table.push(cols);
        if ports.len() > 0 {
            for i in 1..ports.len() {
                let mut row: Vec<&str> = vec![""; COLUMN];
                row[PORTS_AT] = ports[i];
                table.push(row);
            }
        }
    }

    for row in table {
        let formatted: Vec<String> = row
            .into_iter()
            .zip(max_length.into_iter())
            .map(|(s, width)| format!("{:<width$}", s, width = width))
            .collect();
        println!("{}", formatted.join("  "))
    }
}

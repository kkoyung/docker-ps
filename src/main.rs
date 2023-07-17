const FORMAT: &str = "table {{.ID}};{{.Image}};{{.Names}};{{.Status}};{{.Ports}}";
const COLUMN: usize = 5;
const PORTS_AT: usize = COLUMN - 1; // last column

fn main() {
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
    let mut table_ports: Vec<String> = Vec::new();

    for line in stdout.split("\n") {
        if line == "" {
            break;
        }

        let cols: Vec<&str> = line.split(";").collect();
        assert_eq!(cols.len(), COLUMN, "Column number does not match: {}", line);

        let ports: Vec<&str> = cols[PORTS_AT].split(", ").collect();

        // Create port column with prefix
        if ports.len() == 1 {
            if ports[0] == "PORTS" {
                table_ports.push(format!("{}", ports[0]));
            } else if ports[0] == "" {
                table_ports.push(format!("╌"));
            } else {
                table_ports.push(format!("─{}", ports[0]));
            }
        } else {
            for i in 0..ports.len() {
                if i == 0 {
                    table_ports.push(format!("┬{}", ports[i]));
                } else if i == ports.len() - 1 {
                    table_ports.push(format!("└{}", ports[i]));
                } else {
                    table_ports.push(format!("├{}", ports[i]));
                }

                // And measure the width of PORTS column
                let length = ports[i].chars().count();
                max_length[PORTS_AT] = if length > max_length[PORTS_AT] {
                    length
                } else {
                    max_length[PORTS_AT]
                };
            }
        }

        // Measure the widths of other columns
        for (i, col) in cols.iter().enumerate() {
            let length = col.chars().count();
            max_length[i] = if length > max_length[i] {
                length
            } else {
                max_length[i]
            };
        }

        // Generate the new table
        table.push(cols[0..COLUMN - 1].to_vec());
        if ports.len() > 0 {
            for _i in 1..ports.len() {
                let row: Vec<&str> = vec![""; COLUMN - 1];
                table.push(row);
            }
        }
    }

    for (i, row) in table.iter().enumerate() {
        let formatted: Vec<String> = row
            .into_iter()
            .zip(max_length.into_iter())
            .map(|(s, width)| format!("{:<width$}", s, width = width))
            .collect();
        print!("{}", formatted.join("  "));
        println!("  {}", table_ports[i]);
    }
}

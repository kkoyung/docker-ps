use std::process::Command;

const FORMAT: &str = "table {{.ID}};{{.Image}};{{.Names}};{{.Status}};{{.Ports}}";
const COLUMN: usize = 5;

struct Container<'a> {
    id: &'a str,
    image: &'a str,
    name: &'a str,
    status: &'a str,
    ports: Vec<&'a str>,
    is_header: bool, // meta data
}

impl<'a> Container<'a> {
    fn new(line: &'a str, is_header: bool) -> Container {
        let columns: Vec<&str> = line.split(';').collect();
        Container {
            id: columns[0],
            image: columns[1],
            name: columns[2],
            status: columns[3],
            ports: columns[4].split(", ").collect(),
            is_header,
        }
    }

    fn get_column_width(&self) -> [usize; COLUMN] {
        [
            self.id.len(),
            self.image.len(),
            self.name.len(),
            self.status.len(),
            self.ports.iter().fold(
                0,
                |max, next| if next.len() > max { next.len() } else { max },
            ),
        ]
    }

    fn extend_max_length(&self, max_length: &mut [usize; COLUMN]) {
        let length = self.get_column_width();
        for i in 0..COLUMN {
            if length[i] > max_length[i] {
                max_length[i] = length[i];
            }
        }
    }

    fn print_with_padding(&self, max_length: &[usize; COLUMN]) {
        // Remark: no padding for the last columns, even the mex length is
        // calculated in extend_max_length()
        if self.ports.len() == 1 {
            let sign = if self.is_header { "" } else { "─" };
            println!(
                "{:<length0$}  {:<length1$}  {:<length2$}  {:<length3$}  {}{}",
                self.id,
                self.image,
                self.name,
                self.status,
                sign,
                self.ports[0],
                length0 = max_length[0],
                length1 = max_length[1],
                length2 = max_length[2],
                length3 = max_length[3]
            );
        } else {
            let number_of_ports = self.ports.len();
            for (index, port) in self.ports.iter().enumerate() {
                let sign = if index == 0 {
                    '┬'
                } else if index == number_of_ports - 1 {
                    '└'
                } else {
                    '├'
                };
                println!(
                    "{:<length0$}  {:<length1$}  {:<length2$}  {:<length3$}  {}{}",
                    if index == 0 { self.id } else { "" },
                    if index == 0 { self.image } else { "" },
                    if index == 0 { self.name } else { "" },
                    if index == 0 { self.status } else { "" },
                    sign,
                    port,
                    length0 = max_length[0],
                    length1 = max_length[1],
                    length2 = max_length[2],
                    length3 = max_length[3]
                );
            }
        }
    }
}

fn main() {
    // Call "docker ps"
    let output = Command::new("docker")
        .arg("ps")
        .arg("-a")
        .arg("--format")
        .arg(FORMAT)
        .output()
        .expect("Failed to execute \"docker ps\" process");
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Convert to structured data
    let containers: Vec<Container> = stdout
        .split('\n')
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(index, line)| Container::new(line, index == 0))
        .collect();

    // Calculate max length of each column
    let mut max_length: [usize; COLUMN] = [0; COLUMN];
    containers
        .iter()
        .for_each(|container| container.extend_max_length(&mut max_length));

    // Print out
    containers
        .iter()
        .for_each(|container| container.print_with_padding(&max_length));
}

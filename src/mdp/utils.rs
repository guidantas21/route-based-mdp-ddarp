use std::{fs::File, io, io::BufRead};

pub fn read_matrix_from_file(file_name: String) -> io::Result<Vec<Vec<f32>>> {
    let file = File::open(file_name)?;
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();

    let first_line = lines.next().ok_or(io::Error::new(
        io::ErrorKind::InvalidData,
        "File is empty or invalid format",
    ))??;

    let size = first_line.trim().parse::<usize>().map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to parse size: {}", e),
        )
    })?;

    let mut distance_matrix = Vec::with_capacity(size);

    for line in lines.take(size) {
        let row: Vec<f32> = line?
            .split_whitespace()
            .map(|s| {
                s.parse::<f32>().map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Failed to parse distance: {}", e),
                    )
                })
            })
            .collect::<Result<Vec<f32>, _>>()?;

        distance_matrix.push(row);
    }

    if distance_matrix.len() != size {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Matrix size does not match specified size",
        ));
    }

    Ok(distance_matrix)
}

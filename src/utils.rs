pub mod utils {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::str::FromStr;
    use zip::ZipArchive;

    pub fn read_file_to_vec_3(file_path: &str) -> Vec<(i32, i32, i32)> {
        let file = File::open(file_path).expect("Failed to open file");
        let reader = BufReader::new(file);

        let mut vec = Vec::new();

        for line in reader.lines() {
            if let Ok(line) = line {
                let values: Vec<&str> = line.split(',').collect();
                if values.len() == 3 {
                    if let (Ok(val1), Ok(val2), Ok(val3)) = (
                        i32::from_str(values[0]),
                        i32::from_str(values[1]),
                        i32::from_str(values[2]),
                    ) {
                        vec.push((val1, val2, val3));
                    }
                }
            }
        }

        vec
    }

    pub fn read_file_to_vec_2(file_path: &str) -> Vec<(i32, i32)> {
        let file = File::open(file_path).expect("Failed to open file");
        let reader = BufReader::new(file);

        let mut vec = Vec::new();

        for line in reader.lines() {
            if let Ok(line) = line {
                let values: Vec<&str> = line.split(',').collect();
                if values.len() == 2 {
                    if let (Ok(val1), Ok(val2)) = (
                        i32::from_str(values[0]),
                        i32::from_str(values[1]),
                    ) {
                        vec.push((val1, val2));
                    }
                }
            }
        }

        vec
    }

    pub fn read_file_from_zip_to_vec_3(zip_path: &str, file_path: &str) -> Vec<(i32, i32, i32)> {
        let file = File::open(zip_path).expect("Failed to open ZIP file");
        let mut archive = ZipArchive::new(file).expect("Failed to create ZIP archive");

        let mut vec = Vec::new();

        if let Ok(zip_file) = archive.by_name(file_path) {
            let reader = BufReader::new(zip_file);

            for line in reader.lines() {
                if let Ok(line) = line {
                    let values: Vec<&str> = line.split(',').collect();
                    if values.len() == 3 {
                        if let (Ok(val1), Ok(val2), Ok(val3)) = (
                            i32::from_str(values[0]),
                            i32::from_str(values[1]),
                            i32::from_str(values[2]),
                        ) {
                            vec.push((val1, val2, val3));
                        }
                    }
                }
            }
        }

        vec
    }

    pub fn read_file_from_zip_to_vec_2(zip_path: &str, file_path: &str) -> Vec<(i32, i32)> {
        let file = File::open(zip_path).expect("Failed to open ZIP file");
        let mut archive = ZipArchive::new(file).expect("Failed to create ZIP archive");

        let mut vec = Vec::new();

        if let Ok(zip_file) = archive.by_name(file_path) {
            let reader = BufReader::new(zip_file);

            for line in reader.lines() {
                if let Ok(line) = line {
                    let values: Vec<&str> = line.split(',').collect();
                    if values.len() == 2 {
                        if let (Ok(val1), Ok(val2)) = (
                            i32::from_str(values[0]),
                            i32::from_str(values[1]),
                        ) {
                            vec.push((val1, val2));
                        }
                    }
                }
            }
        }

        vec
    }
}
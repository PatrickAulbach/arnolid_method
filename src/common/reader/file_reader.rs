use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;
use num::Num;
use std::str::FromStr;

pub struct FileReader<T> {
    data_type: T
}

impl<T> FileReader<T> where T: Num + FromStr {
    pub fn read_matrix_from_file(path: &Path) -> Vec<T> {
        let file = BufReader::new(File::open(path).unwrap());

        let mut arr: Vec<T> = Vec::new();
        for line in file.lines() {
            let mut l: Vec<T> = line.unwrap().trim().split(char::is_whitespace).flat_map(str::parse::<T>).collect();
            arr.append(&mut l)
        }
        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_struct_should_have_f64_as_data_type() {
        let data: Vec<f64> = FileReader::read_matrix_from_file(Path::new("src/common/reader/test.txt"));

        assert_eq!(vec![1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 0.0, -10.0, -15.514], data);
    }

    #[test]
    fn data_struct_should_have_f32_as_data_type() {
        let data: Vec<f32> = FileReader::read_matrix_from_file(Path::new("src/common/reader/test.txt"));

        assert_eq!(vec![1.0, 2.0, 3.0, 3.0, 2.0, 1.0, 0.0, -10.0, -15.514], data);
    }
}




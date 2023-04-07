use std::{fs, io::BufReader, error::Error};

pub fn get_file_size_in_mb(file_path: &str) -> f64 {
    let metadata: fs::Metadata = fs::metadata(file_path).expect("Error reading file metadata");
    let file_size: f64 = metadata.len() as f64;
    let mb_size: f64 = file_size / (1024.0 * 1024.0);
    mb_size
}

// check whitespace at beginning or end of string
// 
// # Arguments
//
// * `s` - A string slice that we want to check for whitespace at beginning or end
//
// # Returns
//
// * `Result<bool, &'static str>` - A result that is either a boolean or an error message
fn has_whitespace_at_beginning_or_end(s: &str) -> Result<bool,&'static str> {

    if s.len() == 0 {
        return Ok(false);
    }

    let c = s.chars().take(1).last().expect("Error getting first character");
    if c.is_whitespace()  {
        return Ok(true);
    }
    let c = s.chars().rev().take(1).last().expect("Error getting last character");
    if c.is_whitespace() {
        return Ok(true);
    }
    
    Ok(false)
}

pub fn check_all_column_for_nulls_and_whitespace(file_path: &str, delimiter: &str, &quote: &u32, &check_whitespace: &u32) {
    let file: fs::File = std::fs::File::open(file_path).unwrap();
    let bf: BufReader<fs::File> = BufReader::new(file);
    let mut rdr: csv::Reader<BufReader<fs::File>> = csv::ReaderBuilder::new()
        .delimiter(if delimiter == "," { b',' } else { b'\t' })
        .double_quote(match quote == 1 {
            true => true,
            false => false,
        })
        .from_reader(bf);
    let mut columns_with_nulls: Vec<String> = Vec::new();
    let mut has_whitespace: Vec<bool> = Vec::new();
    for result in rdr.records() {
        let record: csv::StringRecord = result.unwrap();
        for field in record.iter() {
            if field.is_empty() {
                columns_with_nulls.push(String::from(field));
            }
            if check_whitespace == 1 {
                if has_whitespace_at_beginning_or_end(field).unwrap() {
                    has_whitespace.push(true);
                }
            }
        }
    }
    if !columns_with_nulls.is_empty() {
        println!("Found columns with NULL values: {:?}", columns_with_nulls);
    } else {
        println!("No columns with nulls");
    }
    if check_whitespace == 1 {
   
        if has_whitespace.len() > 0 {
            println!("Found columns with whitespace at beginning or end");
        } else {
            println!("No columns with whitespace at beginning or end");
        }
    }
}

pub fn print_headers_few_lines_and_line_count(file_path: &str, delimiter: &str, &quote: &u32) {
    let file = get_file_handler(file_path).unwrap();
    let bf: BufReader<fs::File> = BufReader::new(file);
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(if delimiter == "," { b',' } else { b'\t' })
        .double_quote(match quote == 1 {
            true => true,
            false => false,
        })
        .from_reader(bf);
    let headers: &csv::StringRecord = rdr.headers().expect("Error reading headers");
    println!("Headers: {:?}", headers);
    println!(" ");
    let mut count: u32 = 0;
    for result in rdr.records() {
        let record: csv::StringRecord = result.unwrap();
        if count < 3 {
            println!("'Row: {:?}", record);
            println!(" ");
        }
        count += 1;
    }
    println!("number of lines: {}", count);
}

// get file handler
//
// # Arguments
//
// * `file_path` - A string slice that is the path to the file
//
// # Returns
//
// * `Result<fs::File,Box<dyn Error>>` - A result that is either a file handler or an error message
fn get_file_handler(file_path: &str) -> Result<fs::File,Box<dyn Error>> {
    let file: fs::File = fs::File::open(file_path)?;
    Ok(file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_whitespace_at_beginning_or_end_with_leading_space() {
        let s: &str = "  hello";
        let result: bool = has_whitespace_at_beginning_or_end(s).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_has_whitespace_at_beginning_or_end_with_trailing_space() {
        let s: &str = "hello  ";
        let result: bool = has_whitespace_at_beginning_or_end(s).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_has_whitespace_at_beginning_or_end_without_spaces() {
        let s: &str = "hello";
        let result: bool = has_whitespace_at_beginning_or_end(s).unwrap();
        assert_eq!(result, false);
    }

    #[test]
    fn test_has_whitespace_at_beginning_or_end_with_both_leading_and_trailing_spaces() {
        let s: &str = "  hello  ";
        let result: bool = has_whitespace_at_beginning_or_end(s).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_has_whitespace_at_beginning_or_end_with_empty_string() {
        let s: &str = "";
        let result: bool = has_whitespace_at_beginning_or_end(s).unwrap();
        assert_eq!(result, false);
    }

    #[test]
    fn test_get_file_handler() {
        let file_path: &str = "sample.csv";
        let result: fs::File = get_file_handler(file_path).unwrap();
        assert_eq!(result.metadata().unwrap().len(), 1077);
    }

    #[test]
    fn test_get_file_handler_with_non_existent_file() {
        let file_path: &str = "non_existent_file.csv";
        let result: Result<fs::File,Box<dyn Error>> = get_file_handler(file_path);
        assert!(result.is_err());
    }

    #[test]
    #[should_panic]
    fn test_get_file_handler_with_empty_string() {
        let file_path: &str = "";
        let _result: fs::File = get_file_handler(file_path).unwrap();
    }

}
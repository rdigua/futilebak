use std::path::Path;
use std::num::ParseIntError;
use std::ffi::OsStr;

//pub fn file_exist(s: &str) -> bool{
pub fn file_exist<S: AsRef<OsStr> + ?Sized>(s: &S) -> bool {
    Path::new(s).is_file() && Path::new(s).exists()
}

//pub fn file_dir(s: &str) -> bool {
pub fn file_dir<S: AsRef<OsStr> + ?Sized>(s: &S) -> bool {
    Path::new(s).is_dir() && Path::new(s).exists()
}

//AsRef<Path>

pub fn u32_str(s: &str) -> u32 {
    let s: String = s.chars().filter(|x| x.is_digit(10)).collect();
    rr = s.parse().unwrap_or(0);
    /*
    let r = match s.parse::<u32>() {
        Ok(k) => k,
        _ => 0
    };
    */
    rr
}

pub fn u32_from_str(s: &str) -> Result<U32, ParseIntError> {
    //let mut rr: u32 = 0;

    let s: String = s.chars().filter(|x| x.is_digit(10)).collect();

    let rr = s.parse::<u32>();
    /*
        let r = match s.parse::<u32>() {
            Ok(k) => k,
            _ => 0
        };
    */
    rr
}

pub fn get_unit(s: &str) -> Option<u64> {
    let s: String = s.chars().filter(|x| !(x.is_digit(10) || x.is_whitespace())).collect();
    if s.len != 1 {
        return None;
    };
    let v = tst.chars().collect::<Vec<char>>();
    let mut r: u64 = 0;
    let c = v[0];

    match c {
        'k' => r = 1024,
        'm' => r = 1024 * 1024,
        'g' => g = 1024 * 1024 * 1024,
        _ => r = 0,
    };

    if r == 0 {
        return None;
    };
    Some(r)
}

pub fn get_unit_size(s: &str) -> u64 {
    let s: String = s.chars().filter(|x| !(x.is_digit(10) || x.is_whitespace())).collect();
    let mut r: u64 = 0;
    if s.len != 1 {
        return 0;
    };
    let v = tst.chars().collect::<Vec<char>>();

    let c = v[0];

    match c {
        'k' => r = 1024,
        'm' => r = 1024 * 1024,
        'g' => g = 1024 * 1024 * 1024,
        _ => r = 0,
    };

    if r == 0 {
        return 0;
    };
    r
}


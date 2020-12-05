use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;

#[derive(Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    /*    fn to_string(&self) -> String {
            let mut build_string: String;
            build_string = format!("byr: {}", self.byr.as_ref().
                unwrap_or(&String::from("Leer")).to_string());
            build_string += &format!(" iyr: {}", self.iyr.as_ref().
                unwrap_or(&String::from("Leer")).to_string());
            build_string += &format!(" eyr: {}", self.eyr.as_ref().
                unwrap_or(&String::from("Leer")).to_string());
            build_string += &format!(" hgt: {}", self.hgt.as_ref().
                unwrap_or(&String::from("Leer")).to_string());
            build_string += &format!(" hcl: {}", self.hcl.as_ref().
                unwrap_or(&String::from("Leer")).to_string());
            build_string += &format!(" ecl: {}", self.ecl.as_ref().
                unwrap_or(&String::from("Leer")).to_string());
            build_string += &format!(" pid: {}", self.pid.as_ref().
                unwrap_or(&String::from("Leer")).to_string());
            build_string += &format!(" cid: {}", self.cid.as_ref().
                unwrap_or(&String::from("Leer")).to_string());
            build_string += &format!(" valid: {}", self.valid());
            build_string
        }*/

    fn build_valid_fields_vec(&self) -> Vec<bool> {
        let mut valid_fields: Vec<bool> = Vec::new();
        valid_fields.push(self.byr.is_some());
        valid_fields.push(self.iyr.is_some());
        valid_fields.push(self.eyr.is_some());
        valid_fields.push(self.hgt.is_some());
        valid_fields.push(self.hcl.is_some());
        valid_fields.push(self.ecl.is_some());
        valid_fields.push(self.pid.is_some());
        return valid_fields;
    }

    fn valid(&self) -> bool {
        let valid_fields: Vec<bool> = self.build_valid_fields_vec();
        return !(valid_fields.contains(&false));
    }

    fn valid_byr(&self) -> bool {
        if self.byr.as_ref().unwrap().len() != 4 {
            return false;
        }
        let year_result = self.byr.as_ref().unwrap().parse::<i32>();
        if !year_result.is_ok() {
            return false;
        }
        let year = year_result.unwrap();
        if !((1920 <= year) && (year <= 2002)) {
            return false;
        }
        return true;
    }

    fn valid_iyr(&self) -> bool {
        if self.iyr.as_ref().unwrap().len() != 4 {
            return false;
        }
        let year_result = self.iyr.as_ref().unwrap().parse::<i32>();
        if !year_result.is_ok() {
            return false;
        }
        let year = year_result.unwrap();
        if !((2010 <= year) && (year <= 2020)) {
            return false;
        }
        return true;
    }

    fn valid_eyr(&self) -> bool {
        if self.eyr.as_ref().unwrap().len() != 4 {
            return false;
        }
        let year_result = self.eyr.as_ref().unwrap().parse::<i32>();
        if !year_result.is_ok() {
            return false;
        }
        let year = year_result.unwrap();
        if !((2020 <= year) && (year <= 2030)) {
            return false;
        }
        return true;
    }

    fn valid_hgt(&self) -> bool {
        let re_unit = Regex::new(r"[a-z]+").unwrap();
        // re_unit.captures(&self.hgt.as_ref().unwrap());
        let unit_match_iter = re_unit.find_iter(&self.hgt.as_ref().unwrap());
        if unit_match_iter.count() != 1 {
            return false;
        }
        let unit = re_unit.find(&self.hgt.as_ref().unwrap()).unwrap().as_str();
        let re_height_value = Regex::new(r"[0-9]+").unwrap();
        let height_value_match_iter = re_height_value.find_iter(&self.hgt.as_ref().unwrap());
        if height_value_match_iter.count() != 1 {
            return false;
        }
        let height_value: i32 = re_height_value.
            find(&self.hgt.as_ref().unwrap()).unwrap().as_str().parse().unwrap();
        if unit == "cm" {
            if !((150 <= height_value) && (height_value <= 193)) {
                return false;
            }
        }
        if unit == "in" {
            if !((59 <= height_value) && (height_value <= 76)) {
                return false;
            }
        }
        return true;
    }

    fn valid_hcl(&self) -> bool {
        let re_hash = Regex::new(r"#").unwrap();
        let hash_matches = re_hash.find_iter(self.hcl.as_ref().unwrap());
        if hash_matches.count() != 1 {
            return false;
        }
        let re_code = Regex::new(r"[0-9a-f]+").unwrap();
        let code_matches = re_code.find_iter(self.hcl.as_ref().unwrap());
        if code_matches.count() != 1 {
            return false;
        }
        let code_match = re_code.find(self.hcl.as_ref().unwrap()).unwrap().as_str();
        if code_match.len() != 6 {
            return false;
        }
        return true;
    }

    fn valid_ecl(&self) -> bool {
        let valid_eye_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        return valid_eye_colors.contains(&self.ecl.as_ref().unwrap().as_str());
    }

    fn valid_pid(&self) -> bool {
        return self.pid.as_ref().unwrap().len() == 9;
    }

    fn valid_part_2(&self) -> bool {
        if self.valid() {
            let fields_valid_vec = vec![self.valid_byr(),
                                        self.valid_iyr(),
                                        self.valid_eyr(),
                                        self.valid_hgt(),
                                        self.valid_hcl(),
                                        self.valid_ecl(),
                                        self.valid_pid()];
            return fields_valid_vec.iter().all(|&x| x);
        }
        return false;
    }
}


fn read_input_data(file_name: &str) -> io::Result<Vec<Passport>> {
    let f = File::open(file_name)?;
    let f = BufReader::new(f);

    let mut passports: Vec<Passport> = Vec::new();
    let mut current_passport: Passport = Passport::default();

    for line in f.lines() {
        let line_string = line.unwrap();
        if line_string == "" {
            passports.push(current_passport);
            current_passport = Passport::default();
            continue;
        }
        let data_strings = line_string.split(" ").collect::<Vec<&str>>();
        for data_string in data_strings {
            let mut data_string_split = data_string.split(":");
            let (property, value) =
                (data_string_split.next().unwrap(), data_string_split.next().unwrap());
            match property {
                "byr" => current_passport.byr = Some(String::from(value)),
                "iyr" => current_passport.iyr = Some(String::from(value)),
                "eyr" => current_passport.eyr = Some(String::from(value)),
                "hgt" => current_passport.hgt = Some(String::from(value)),
                "hcl" => current_passport.hcl = Some(String::from(value)),
                "ecl" => current_passport.ecl = Some(String::from(value)),
                "pid" => current_passport.pid = Some(String::from(value)),
                "cid" => current_passport.cid = Some(String::from(value)),
                _ => {}
            }
        }
    }
    passports.push(current_passport);
    Ok(passports)
}

fn main() {
    let passports: Vec<Passport> = read_input_data("inputData.txt").unwrap();
    let mut valid_passport_count = 0;
    for passport in &passports {
        if passport.valid() {
            valid_passport_count += 1;
        }
    }
    println!("{}", valid_passport_count);
    valid_passport_count = 0;
    for passport in &passports {
        if passport.valid_part_2() {
            valid_passport_count += 1;
        }
    }
    println!("{}", valid_passport_count);
}

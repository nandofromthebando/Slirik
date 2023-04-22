use std::fs::read;
use std::ops::{Add, Div, Mul, Sub};
use std::process::Output;
use std::string::ToString;

use crate::Statement;

static mut CLASS: &str = "MAIN";
static mut DIRECTORY: String = String::new();
static mut OPERATION: String = String::new();
static mut VARIABLE_TYPE: String = String::new();
static mut LOADED_VARIABLE: String = String::new();
static mut VARIABLES: Vec<(&str, String, String)> = Vec::new();
// class, dir, var
static mut VARIABLE_VALUE: Vec<(String, Storage)> = Vec::new();

struct Storage {
    value_type: String,
    value: String,
}

pub unsafe fn execute(statement: Statement) {
    match statement.identifier.as_str() {
        "dir" => DIRECTORY = statement.value.clone(),
        "var" => {
            let value = statement.value.clone();
            VARIABLES.push((CLASS, DIRECTORY.clone(), value.clone()));
            VARIABLE_VALUE.push((value, Storage {
                value_type: VARIABLE_TYPE.parse().unwrap(),
                value: "".to_string(),
            }))
        }
        "type" => VARIABLE_TYPE = statement.value.clone(),
        "load" => LOADED_VARIABLE = statement.value.clone(),
        "set" => {
            for storage in VARIABLE_VALUE.iter_mut().filter(|(var_name, _)| var_name == &LOADED_VARIABLE).map(|(_, storage)| storage) {
                storage.value = statement.value.clone();
            }
        }
        "add" => {
            for storage in VARIABLE_VALUE.iter_mut().filter(|(var_name, _)| var_name == &LOADED_VARIABLE).map(|(_, storage)| storage) {
                if storage.value_type == "int" {
                    let current_value = storage.value.clone().parse::<i32>().unwrap();
                    let result = calculate(current_value, statement.value.parse::<i32>().unwrap());
                    storage.value = result.to_string();
                }
            }
        }
        "op" => OPERATION = statement.value.clone(),
        _ => print_error(String::from("WARNING"), String::from("Undefined statement"), statement.identifier.clone())
    }
}

pub unsafe fn print_variables() {
    for val in &VARIABLE_VALUE {
        println!("{} {} {}", val.1.value_type, val.0, val.1.value);
    }
}

unsafe fn calculate<T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T>>(x: T, y: T) -> T {
    match OPERATION.as_str() {
        "+" => { return x + y; }
        "-" => { return x - y; }
        "*" => { return x * y; }
        "/" => { return x / y; }
        "^" => { return x; }
        "#" => { return x; }
        _ => {
            print_error(String::from("ERROR"), String::from("Unexpected operator"), OPERATION.clone());
            panic!();
        }
    }
}

fn print_error(level: String, message: String, fail: String) {
    println!("{} | {}: {}", level, message, fail);
}
pub mod file_op;

use std::any::type_name;

/**
*   Print the type of a variable
*/ 
pub fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}
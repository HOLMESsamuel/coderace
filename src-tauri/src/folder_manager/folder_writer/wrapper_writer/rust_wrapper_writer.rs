use crate::models::{Argument, ImplementationFolder};
use std::process::Command;

pub fn write_rust_wrapper(implementation_folder: &ImplementationFolder) -> String {
    let argument_string = build_argument_string(&implementation_folder.arguments);
    let wrapper_code = format!(
        "extern crate time;
extern crate mem_info;
use {module_name}::{method_name};

fn main() {{
    let start_time = time::precise_time_ns();
    let result = {method_name}{argument_string};
    let end_time = time::precise_time_ns();
    let elapsed_time = (end_time - start_time) as f64 / 1_000_000_000.0;

    let mem_info = mem_info::get_mem_info().unwrap();
    let mem_usage = mem_info.total - mem_info.avail;

    println!(\"{{:?}}\", result);
    println!(\"{{:.3f}}\", elapsed_time);
    println!(\"{{:.1f}}\", mem_usage as f64 / 1024.0);
}}
", module_name = implementation_folder.module_name,
        method_name = implementation_folder.method_name,
        argument_string = argument_string);

    wrapper_code
}

fn build_argument_string(arguments: &[Argument]) -> String {
    let arg_strings = map_arguments_to_strings(arguments);

    format!("({})", arg_strings.join(", "))
}

fn map_arguments_to_strings(arguments: &[Argument]) -> Vec<String> {
    arguments
        .iter()
        .map(|arg| match arg.argument_type.as_str() {
            "int" => arg.value.clone(),
            "string" => format!("\"{}\"", arg.value.trim_matches('\'').to_string()),
            _ => String::new(),
        })
        .collect()
}

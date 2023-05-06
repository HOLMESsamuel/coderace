use crate::models::{Argument, ImplementationFolder};
pub fn write_python_wrapper(implementation_folder: &ImplementationFolder) -> String {
    let argument_string = build_argument_string(&implementation_folder.arguments);
    let argument_string_memory_usage = build_argument_string_for_memory_usage(&implementation_folder.arguments);
    let wrapper_code = format!(
        "import time
import sys
from memory_profiler import memory_usage
from {module_name} import {method_name}
from six import print_

def main():
    start_time = time.time()
    mem_usage = memory_usage((factorial, {argument_string_memory_usage}), max_usage=True)
    result = {method_name}{argument_string}
    end_time = time.time()
    elapsed_time = end_time - start_time
    print_(result)
    print_(elapsed_time)
    print_(mem_usage)

if __name__ == \"__main__\":
    main()
", module_name = implementation_folder.module_name,
    method_name = implementation_folder.method_name,
    argument_string = argument_string,
    argument_string_memory_usage = argument_string_memory_usage);

    wrapper_code
}

fn build_argument_string(arguments: &[Argument]) -> String {
    let arg_strings = map_arguments_to_strings(arguments);

    format!("({})", arg_strings.join(", "))
}

//the memory usage method takes a tuple as argument, it will look like (argument1,) if there is only one argument
fn build_argument_string_for_memory_usage(arguments: &[Argument]) -> String {
    let arg_strings = map_arguments_to_strings(arguments);

    if arguments.len() == 1 {
        format!("({},)", arg_strings.join(", "))
    } else {
        format!("({})", arg_strings.join(", "))
    }
}

fn map_arguments_to_strings(arguments: &[Argument]) -> Vec<String> {
    arguments
        .iter()
        .map(|arg| match arg.argument_type.as_str() {
            "int" => arg.value.clone(),
            "string" => arg.value.trim_matches('\'').to_string(),
            _ => String::new(),
        })
        .collect()
}

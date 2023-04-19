pub fn write_python_wrapper(implementation_name: &str) -> String {
    //TODO peut etre utiliser des command line argument pour les arguments des methodes
    let wrapper_code = format!(
"import time
import sys
from memory_profiler import memory_usage
from {implementation_name} import factorial

def main():
    input_number = 4
    start_time = time.time()
    mem_usage = memory_usage((factorial, (input_number,)), max_usage=True)
    result = factorial(input_number)
    end_time = time.time()
    elapsed_time = end_time - start_time
    print(f\"Result: {{result}}\")
    print(f\"Execution time: {{elapsed_time}} seconds\")
    print(f\"Memory usage: {{mem_usage}} MiB\")

if __name__ == \"__main__\":
    main()
", implementation_name = implementation_name);
    wrapper_code
}

pub fn write_python_wrapper(implementation_name: &str) -> String {
    let wrapper_code = format!(
        "import time
import sys
from memory_profiler import memory_usage
from {implementation_name} import factorial
from six import print_

def main():
    input_number = 4
    start_time = time.time()
    mem_usage = memory_usage((factorial, (input_number,)), max_usage=True)
    result = factorial(input_number)
    end_time = time.time()
    elapsed_time = end_time - start_time
    print_(result)
    print_(elapsed_time)
    print_(mem_usage)

if __name__ == \"__main__\":
    main()
", implementation_name = implementation_name);
    wrapper_code
}

import time
import sys
from memory_profiler import memory_usage
from factorial import factorial
from six import print_

def main():
    start_time = time.time()
    mem_usage = memory_usage((factorial, (4,)), max_usage=True)
    result = factorial(4)
    end_time = time.time()
    elapsed_time = end_time - start_time
    print_(result)
    print_(elapsed_time)
    print_(mem_usage)

if __name__ == "__main__":
    main()

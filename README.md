# CodeRace

CodeRace is a Rust-based project designed to measure the performance of different implementations of a method or small service across multiple programming languages. The project analyzes each implementation in terms of execution time, memory usage, and other relevant metrics. By using Docker containers, CodeRace ensures a consistent and isolated environment for each implementation.

## Features

- Automatically wraps the code for each implementation to measure execution time, memory usage, and other relevant metrics
- Generates Docker containers to provide a consistent environment for each implementation
- Compares the performance of various implementations and displays the results in a comprehensive manner

## Getting Started

To get started with CodeRace, follow these steps:

1. Clone the repository.

2. Add the implementation of the method or small service in different languages to the `implementations` folder with the following structure : /{language-name}/{version}/{implementation-name}. An example is in the repository. 
        
        /implementations
            /java
            /python
                /latest
                    /fibonacci
            /rust
                /latest
                    /fibonacci
                /1.68.2
                    /fibonacci

3. In each folder you need to place a file containing a method that you want to benchmark, this method will be the entry point. For example here with
python you will place a fibonacci.py file containing a fibonacci(n) method.
You also need to add a config.json file containing the name of the file containing the entry point method, the method name and a list of the arguments needed.
It will look like this : 

       {
         "method_name": "factorial",
         "module_name": "factorial",
         "arguments": [
           {
             "name": "n",
             "value": "4",
             "argument_type": "int"
           }
         ]
       }

4. Install Rust and Docker on your system if you haven't already.

5. Run the project:

    
    cargo run


6. CodeRace will build and run Docker containers for each implementation, and then analyze and compare the performance of each implementation.

7. After the benchmarking process has finished, CodeRace will output a detailed comparison of the implementations, including metrics such as execution time and memory usage.

## Contributing

Contributions to CodeRace are welcome! If you would like to contribute, please follow these steps:

1. Fork the repository and create your branch from the `main` branch.

2. Write your code and add or modify any necessary tests.

3. Ensure that your code is well-documented and follows the project's coding style.

4. Submit a pull request, and make sure to provide a clear and concise description of your changes.

## License

CodeRace is released under the MIT License.

# Documentation

## How does it work ?

In each folder containing some implementation to benchmark a custom wrapper is created, it will call the method to be benchmarked and gather metrics about it.
A dockerfile is also create to blueprint a container whose role is to run the wrapper file. The results are then gathered and displayed for each implementation to be compared.

The whole process can be decomposed in 4 phases : 

1. The implementation folder is readed to know what language, version and implementation will be used. In this phase the BenchlarkInstruction object is created and will be used in other modules to pass instructions.
2. Depending on the language different files are written in the implementation folders such as requirements.txt for python. A wrapper and a dockerfile are created in each folder.
3. One container is created for each dockerfile, then it is runned, during the process the wrapper will call the method that needs to be tested and gather metrics about its execution, then the container is stopped and the image removed so that the starting conditions are the same for each container.
4. Once all containers have been runned and destroyed the results are displayed.
## Project structure

Each functionality is separated in its own module : 

* command_runner module contains everything needed to run commands especially docker commands that are used to start the different executions.
* folder_manager contains folder_writer and folder_reader :
  * folder_writer : contains everything needed to write files in the folders such as requirements.txt, dockerfiles or wrappers
  * folder_reader : reads the implementation folder and create the BenchmarkInstruction Struct containing all the information to run the benchmark process.
* result_writer : this module will use command runner module to start each container and gather the results.

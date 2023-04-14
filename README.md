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
        

3. Install Rust and Docker on your system if you haven't already.

4. Run the project:

    
    cargo run


5. CodeRace will build and run Docker containers for each implementation, and then analyze and compare the performance of each implementation.

6. After the benchmarking process has finished, CodeRace will output a detailed comparison of the implementations, including metrics such as execution time and memory usage.

## Contributing

Contributions to CodeRace are welcome! If you would like to contribute, please follow these steps:

1. Fork the repository and create your branch from the `main` branch.

2. Write your code and add or modify any necessary tests.

3. Ensure that your code is well-documented and follows the project's coding style.

4. Submit a pull request, and make sure to provide a clear and concise description of your changes.

## License

CodeRace is released under the MIT License.


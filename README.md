# Step-by-step calculator

## Introduction
This is the project I chose for the Rust Course @FII in the second year of my bachelor's degree. I chose to do this project because I like mathematical expressions and wanted to create something that is both usefull and practical.

The Step-by-Step Calculator is a command-line application implemented in Rust that evaluates mathematical expressions step by step, providing detailed intermediate results for each calculation. The calculator supports a variety of operations, including basic arithmetic, trigonometric functions, logarithmic functions, and more. Additionally, the calculator allows users to save their evaluations to files for later reference.

### Key Features
- **Supported Operators**:
  - Basic arithmetic: `+`, `-`, `*`, `/`
  - Exponentiation: `^` (e.g., `2 ^ 3`)
  - Trigonometric functions: `sin`, `cos`, `tg`, `cotg`, `sec`, `csc`, `asin`, `acos`, `atg`, `actg` (all in degrees)
  - Logarithmic functions: `log(base, number)`, `ln`
  - Square root: `sqrt`
  - Absolute value: `abs`
  - Factorial: `!`
  - Constants: `pi` (3.14159), `e` (2.71828)
  - Parentheses for grouping: `( and )`

- **Step-by-Step Evaluation**:
  - Provides intermediate steps for every calculation.

- **File Management**:
  - Save evaluations to files.
  - Delete all saved evaluations.

## Getting Started

### Prerequisites
Ensure you have the following installed:
- Rust programming language and Cargo (available at [Rust's official website](https://www.rust-lang.org/)).

### Building the Project
To build the project, run the following command in the terminal from the project directory:
```bash
cargo build --release
```
This will generate an optimized executable in the `target/release` directory.

### Running the Application
To run the application, use the following command:
```bash
cargo run --release
```
Alternatively, execute the built binary directly:
```bash
./target/release/step_calc
```

### Help Command
To display the help message and a list of available commands, run:
```bash
./target/release/step_calc --help
```

## Usage
### Evaluating Expressions
- Enter a mathematical expression, and the calculator will provide a step-by-step evaluation of the result.
  Example:
  ```
  > 2 + 3 * (4 ^ 2)
  Step 1: 2 + 3 * 16
  Step 2: 2 + 48
  Result: 50
  ```

### Saving Evaluations
- Save the results of evaluations by entering the appropriate command during use. Saved files will be stored in the application directory.

### Deleting Saved Evaluations
- Use the delete command to clear all saved evaluations.

### Example Commands
- Evaluate expressions: `2 ^ 3 + log(10, 100)`
- Save evaluations: `save`
- Delete saved evaluations: `delete_all`

## Project Structure
- **src/**: Contains the Rust source files for the project.
  - `main.rs`: Entry point for the application.
  - `calculator.rs`: Core logic for parsing and evaluating expressions.
  - `file_manager.rs`: Handles saving and deleting evaluations.
- **Cargo.toml**: Specifies dependencies and project metadata.

## Future Improvements
- Add support for additional mathematical functions and constants.
- Enhance the user interface for improved usability.
- Introduce support for different output formats (e.g., JSON or CSV).

## License
This project is licensed under the MIT License.

```
MIT License

Copyright (C) 2025 Efros Ciprian

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---
Thank you for using Step-by-Step Calculator! We hope it simplifies your mathematical tasks.



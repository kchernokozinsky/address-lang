# Address Language

Address Language  developed by Katerina Logvynivna Yushchenko is a programming language designed to process complex hierarchical structures using an imperative programming style. This project includes a parser, a virtual machine (VM), and a code generator to facilitate executing of programs written in Address Language.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

## Installation

1. **Clone the repository:**
   ```sh
   git clone https://github.com/kchernokozinsky/address-lang.git
   cd address-lang
   ```

2. **Build the project:**
   ```sh
   cargo build
   ```

3. **Run the tests:**
   ```sh
   cargo test
   ```

## Usage

To run a program written in Address Language, you need to use adl-cli.

1. **Write your Address Language program:**

   Create a file named `program.adl` and write your Address Language code in it.

2. **Run adl-cli:**
   ```sh
   cargo run --bin adl-cli
   ```
   ![cli](https://github.com/kchernokozinsky/address-lang/blob/main/pics/cli.png)

3. **Execute your code:**

   Enter command: 
   ``` sh
   run -f path/to/program.adl  
   ```


## Examples

Here are some examples of programs written in Address Language and how to run them.

### Example 1: Hello World

**Program:**

```mathematica
Print{"Hello, World!"}
```

## Contributing

Contributions are welcome! Please fork the repository and submit pull requests for new features, bug fixes, or improvements.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

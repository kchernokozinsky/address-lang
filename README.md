# Compiler for Address Language
Rust-based compiler for the Address Language developed by Ekaterina Logvinovna Yushchenko.

## Development Plan

1. **Preliminary Research**
   - Study the Address Language specifications in detail.
   - Understand the underlying architecture for which the code will be compiled.

2. **Lexer Development**
   - Design token specifications for the Address Language.
   - Implement the lexer using libraries like `nom` or `pest`.
   - Test lexer against various Address Language code snippets.

3. **Parser Development**
   - Define grammar for the Address Language.
   - Implement the parser which generates a parse tree from tokens.
   - Test parser with diverse program constructs.

4. **Semantic Analysis**
   - Implement type checking, variable binding, and other semantic checks.
   - Design appropriate error messages for semantic errors.

5. **Intermediate Representation (IR) Generation**
   - Define an appropriate IR for the compiler.
   - Implement translation from parse tree to IR.

6. **Code Optimization (optional)**
   - Implement basic optimization techniques on the IR.
   - Ensure optimization correctness with tests.

7. **Code Generation**
   - Design the target architecture's machine code or bytecode specifications.
   - Translate IR to machine code or bytecode.
   
8. **Testing**
   - Write unit tests for each module: lexer, parser, semantic analyzer, and code generator.
   - Implement integration tests with sample Address Language programs.

9. **Documentation**
   - Document functions, structures, and other important constructs.
   - Write a user guide for the compiler with examples and usage guidelines.

10. **Deployment and Packaging**
   - Decide on a distribution method (e.g., Cargo package, binary release).
   - Package the compiler for release.



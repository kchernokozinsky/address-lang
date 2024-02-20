# LR(1) Grammar Specification

This document specifies the LR(1) grammar used in our project. The grammar is designed to parse expressions, which can be atoms or lists. Atoms can be numbers or symbols, while lists are composed of expressions enclosed in parentheses.

### Grammar Rules

```ebnf
Algorithm = { FileLine }

FileLine = LabelsDeclaration, Statements, ("END_OF_FILE" | "NEW_LINE")

LabelsDeclaration = Labels, "..."

Labels = { Identifier, "," }, Identifier?

OneLineStatement = CallSubProgram
                 | UnconditionalJump
                 | Loop
                 | Predicate
                 | "!"
                 | "RETURN"

CallSubProgram = "SUB_PROGRAM", Identifier, "{", Parameters, "}", Identifier?

Predicate = "PREDICATE", "{", ExpressionPrecedence8, "}", Statements, "|", Statements

Loop = "LOOP", "{", Expression, ",", Expression, ",", Expression, "=>", Expression, "}", Identifier, Identifier?

UnconditionalJump = "@", Identifier

SimpleStatement = Expression, "=", Expression
                 | Expression, "<=>", Expression
                 | Expression, "=>", Expression
                 | Expression

Expression = ExpressionPrecedence8

ExpressionPrecedence8 = ExpressionPrecedence8, "OR", ExpressionPrecedence7
                      | ExpressionPrecedence7

ExpressionPrecedence7 = ExpressionPrecedence7, "AND", ExpressionPrecedence6
                      | ExpressionPrecedence6

ExpressionPrecedence6 = ExpressionPrecedence6, "==", ExpressionPrecedence5
                      | ExpressionPrecedence6, "!=", ExpressionPrecedence5
                      | ExpressionPrecedence6, "<", ExpressionPrecedence5
                      | ExpressionPrecedence5

ExpressionPrecedence5 = ExpressionPrecedence5, "+", ExpressionPrecedence4
                      | ExpressionPrecedence5, "-", ExpressionPrecedence4
                      | ExpressionPrecedence4

ExpressionPrecedence4 = ExpressionPrecedence4, "*", ExpressionPrecedence3
                      | ExpressionPrecedence4, "/", ExpressionPrecedence3
                      | ExpressionPrecedence3

ExpressionPrecedence3 = ExpressionPrecedence2

ExpressionPrecedence2 = "NOT", ExpressionPrecedence2
                      | "'", ExpressionPrecedence2
                      | ExpressionPrecedence1

ExpressionPrecedence1 = LiteralExpression
                      | List
                      | FunctionCall
                      | MultipleDereference
                      | Variable
                      | "(", Expression, ")"

LiteralExpression = BoolLiteral
                  | IntLiteral
                  | StringLiteral
                  | FloatLiteral
                  | NullLiteral

FunctionCall = "IDENTIFIER", "{", Parameters, "}"

MultipleDereference = "DEREF", "{", Expression, ",", Expression, "}"

Statements = { SimpleStatement, ";" }
            | OneLineStatement

Parameters = { Expression, "," }

Variable = Identifier

List = "[", Parameters, "]"

NullLiteral = "NULL"

IntLiteral = "INTEGER_LITERAL"

FloatLiteral = "FLOAT_LITERAL"

BoolLiteral = "TRUE"
            | "FALSE"

StringLiteral = "STRING_LITERAL"

Identifier = "IDENTIFIER"
```

## Terminal Symbols Description

### Identifiers and Literals

- **IDENTIFIER**: An alphanumeric string used for naming variables, functions, etc. It starts with a letter or underscore, followed by letters, digits, or underscores.
- **INTEGER_LITERAL**: A sequence of digits representing an integer value.
- **FLOAT_LITERAL**: A numeric literal with a decimal point, representing a floating-point number.
- **STRING_LITERAL**: A sequence of characters enclosed in quotes, representing a string value.

### Special Tokens

- **NEW_LINE**: Represents a newline character in the source code, used to separate lines or statements.
- **END_OF_FILE**: A special token indicating the end of the input file.

### Symbolic Operators and Delimiters

- **(**
- **)**
- **[**
- **]**
- **:**
- **,**
- **;**
- **\***
- **+**
- **-**
- **/**
- **|**
- **&**
- **<**
- **>**
- **=**
- **.**
- **%**
- **{**
- **}**

### Logical and Control Symbols

- **==** 
- **!=**
- **=<**
- **>=**
- **>>**
- **<<**
- **=>**
- **...**
- **'**
- **<=>**
- **REPLACE**, **LOOP**, **PREDICATE**, **SUB_PROGRAM**, **@**, **!**, **RETURN**, **FALSE**, **NULL**, **TRUE**, **AND**, **DEL**, **NOT**, **OR**, **LET**, **CONST**, **DEREF**: Specific keywords or operators used in the language for various control structures, logical operations, or special functionality.




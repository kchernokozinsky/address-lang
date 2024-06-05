# LR(1) Grammar Specification

This document specifies the LR(1) grammar used in our project.
### Grammar Rules

```ebnf
<Algorithm> ::= <FileLine> { <FileLine> }

<FileLine> ::= <LabelsDeclaration> <Statements> ("END_OF_FILE" | "NEW_LINE")

<LabelsDeclaration> ::= <Labels> "..."

<Labels> ::= { <Identifier> "," } <Identifier>?

<Statements> ::= { <SimpleStatement> ";" }
                 | <OneLineStatement>

<SimpleStatement> ::= <Expression> "=" <Expression>
                    | <Expression> "<=>" <Expression>
                    | <Expression> "=>" <Expression>
                    | <Expression>

<OneLineStatement> ::= <CallSubProgram>
                     | <UnconditionalJump>
                     | <Loop>
                     | <Predicate>
                     | "!"
                     | "RETURN"

<CallSubProgram> ::= "SUB_PROGRAM" <Identifier> "{" <Parameters> "}" <Identifier>?

<Predicate> ::= "PREDICATE" "{" <ExpressionPrecedence8> "}" <Statements> "|" <Statements>

<Loop> ::= "LOOP" "{" <Expression> "," <Expression> "," <Expression> "=>" <Expression> "}" <Identifier> <Identifier>?

<UnconditionalJump> ::= "@" <Identifier>

<Expression> ::= <ExpressionPrecedence8>

<ExpressionPrecedence8> ::= <ExpressionPrecedence8> "OR" <ExpressionPrecedence7>
                          | <ExpressionPrecedence7>

<ExpressionPrecedence7> ::= <ExpressionPrecedence7> "AND" <ExpressionPrecedence6>
                          | <ExpressionPrecedence6>

<ExpressionPrecedence6> ::= <ExpressionPrecedence6> "==" <ExpressionPrecedence5>
                          | <ExpressionPrecedence6> "!=" <ExpressionPrecedence5>
                          | <ExpressionPrecedence6> "<" <ExpressionPrecedence5>
                          | <ExpressionPrecedence5>

<ExpressionPrecedence5> ::= <ExpressionPrecedence5> "+" <ExpressionPrecedence4>
                          | <ExpressionPrecedence5> "-" <ExpressionPrecedence4>
                          | <ExpressionPrecedence4>

<ExpressionPrecedence4> ::= <ExpressionPrecedence4> "*" <ExpressionPrecedence3>
                          | <ExpressionPrecedence4> "/" <ExpressionPrecedence3>
                          | <ExpressionPrecedence3>

<ExpressionPrecedence3> ::= <ExpressionPrecedence2>

<ExpressionPrecedence2> ::= "NOT" <ExpressionPrecedence2>
                          | "'" <ExpressionPrecedence2>
                          | <ExpressionPrecedence1>

<ExpressionPrecedence1> ::= <LiteralExpression>
                          | <List>
                          | <FunctionCall>
                          | <MultipleDereference>
                          | <Variable>
                          | "(" <Expression> ")"

<LiteralExpression> ::= <BoolLiteral>
                      | <IntLiteral>
                      | <StringLiteral>
                      | <FloatLiteral>
                      | <NullLiteral>

<FunctionCall> ::= <Identifier> "{" <Parameters> "}"

<MultipleDereference> ::= "DEREF" "{" <Expression> "," <Expression> "}"

<Parameters> ::= { <Expression> "," }

<Variable> ::= <Identifier>

<List> ::= "[" <Parameters> "]"

<NullLiteral> ::=  ...

<IntLiteral> ::= ...

<FloatLiteral> ::= ...

<BoolLiteral> ::= ...

<StringLiteral> ::= ...

<Identifier> ::= ...
```

## Syntax Transformation Table

The syntax of the Addressed Programming Language, although a powerful tool for manipulating complex hierarchical data structures, has proven not entirely convenient for writing programs on modern computers. The original syntax of the Addressing Language uses a lot of characters that are not always available on a standard keyboard, which makes writing code much more complicated and slower. To simplify and speed up writing programs in the Addressing Language, it was decided to make certain replacements in the notation.



| Element Name                   | Original Syntax                           | Updated Syntax                      |
|--------------------------------|-------------------------------------------|-------------------------------------|
| Formula of Relative Stop       | ᗺ                                        | return                              |
| Formula of Relative Transition | ↓n                                       | \|n                                 |
| Entry Formula                  | П α { a1, …, an } β                       | SP a { a1, …, an } b                |
| Predicate Formula              | P { L } α ↓ β                             | P { L } α \| β                      |
| Loop Formula                   | Ц{a, step, condition => pi } b l           | L{a, step, condition => pi } b l    |
| Empty Set                      | ∅                                         | null                                |
| Multiple Stroke Operation      | kn                                        | D { n, k }                          |
| Unconditional Transition Labels| label                                     | @label                              |
| Replacement Formula            | З{ …}                                     | R{ …}                               |

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



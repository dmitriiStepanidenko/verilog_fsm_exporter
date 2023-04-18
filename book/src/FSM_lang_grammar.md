# Грамматика FSM_lang

```
<FSM> ::= {<state_declaration>}+
<state_declaration> ::= "state" <state_name> ":" {<input_declaration> | <output_declaration> | <transition_action>}*
<state_name> ::= <identifier>
<input_declaration> ::= "input" "(" <bit_width> ")" <input_name> ";"
<output_declaration> ::= "output" "(" <bit_width> ")" <output_name> ";"
<transition_action> ::= "on" <condition> "->" <state_name> <action> ";"
<condition> ::= <input_name> <comparison_operator> <value>
<action> ::= "{" <output_name> <assignment_operator> <value> "}"

<identifier> ::= [a-zA-Z_][a-zA-Z0-9_]*
<input_name> ::= <identifier>
<output_name> ::= <identifier>
<bit_width> ::= [1-9][0-9]*

<comparison_operator> ::= "==" | "!=" | "<" | "<=" | ">" | ">="
<assignment_operator> ::= "="

<value> ::= <binary_value> | <decimal_value>
<binary_value> ::= [0-1]+ "b"[0-1]+
<decimal_value> ::= [0-9]+
```

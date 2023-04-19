use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::{
    bytes::complete::{tag, take_while},
    character::complete::multispace0,
    character::is_alphanumeric,
    combinator::recognize,
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};
use tree::structures::*;
// ```norust
// <FSM> ::= {<state_declaration>}+
// <state_declaration> ::= "state" <state_name> ":" {<input_declaration> | <output_declaration> | <transition_action>}*
// <state_name> ::= <identifier> | <identifier> ","
// <input_declaration> ::= "input" "(" <bit_width> ")" <input_name> ";"
// <output_declaration> ::= "output" "(" <bit_width> ")" <output_name> ";"
// <transition_action> ::= "on" <condition> "->" <state_name> <action> ";"
// <condition> ::= <input_name> <comparison_operator> <value>
// <action> ::= "{" <output_name> <assignment_operator> <value> "}"
//
// <identifier> ::= [a-zA-Z_][a-zA-Z0-9_]*
// <input_name> ::= <identifier>
// <output_name> ::= <identifier>
// <bit_width> ::= [1-9][0-9]*
//
// <comparison_operator> ::= "==" | "!=" | "<" | "<=" | ">" | ">="
// <assignment_operator> ::= "="
//
// <value> ::= <binary_value> | <decimal_value>
// <binary_value> ::= [0-1]+ "b"[0-1]+
// <decimal_value> ::= [0-9]+
// ```
fn is_alphabetic(c: char) -> bool {
    c.is_ascii_alphabetic()
}

/// # Нетерминал identifier
///
/// <identifier> ::= [a-zA-Z_][a-zA-Z0-9_]*
///
///
fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        nom::character::complete::satisfy(is_alphabetic),
        many0(nom::character::complete::satisfy(|c| {
            is_alphanumeric(c as u8) || c == '_'
        })),
    ))(input)
}

/// # Нетерминал decimal_value
///
/// <decimal_value> ::= [0-9]+
///
///
fn decimal_value(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

/// # Ключевые слова
fn keyword<'a>(kw: &'a str) -> impl Fn(&str) -> IResult<&str, String> + 'a {
    move |input| {
        let (input, _) = multispace0(input)?;
        let (input, _) = tag(kw)(input)?;
        let (input, _) = multispace0(input)?;
        Ok((input, kw.to_string()))
    }
}

fn symbol(input: &str) -> IResult<&str, char> {
    preceded(multispace0, nom::character::complete::one_of(";,()"))(input)
}

////////////////

#[derive(Debug, PartialEq)]
pub struct Transition<'a> {
    condition: (&'a str, &'a str, u32),
    target: &'a str,
    actions: Vec<(&'a str, u32)>,
}

fn transition(input: &str) -> IResult<&str, Transition> {
    let (input, source) = identifier(input)?;
    let (input, _) = tag("==")(input)?;
    let (input, value) = decimal_value(input)?;
    let (input, _) = tag("->")(input)?;
    let (input, target) = identifier(input)?;
    let (input, actions) = delimited(
        tag("{"),
        many1(terminated(
            separated_pair(identifier, tag("="), decimal_value),
            tag(";"),
        )),
        tag("}"),
    )(input)?;

    Ok((
        input,
        Transition {
            condition: (source, "==", value),
            target,
            actions,
        },
    ))
}

/// <state_declaration> ::= "state" <state_name> ":" {<input_declaration> | <output_declaration> | <transition_action>}*
/// <state_name> ::= <identifier>
fn state(
    input: &str,
) -> IResult<&str, (&str, Vec<(&str, u32)>, Vec<(&str, u32)>, Vec<Transition>)> {
    let kw_state = keyword("state");
    let kw_input = keyword("input");
    let kw_output = keyword("output");
    let kw_on = keyword("on");

    let (input, _) = kw_state(input)?;
    let (input, state_name) = identifier(input)?;
    let (input, _) = tag(":")(input)?;

    let (input, inputs) = delimited(
        kw_input,
        many1(terminated(
            separated_pair(identifier, tag(","), decimal_value),
            tag(";"),
        )),
        tag(";"),
    )(input)?;

    let (input, outputs) = delimited(
        kw_output,
        many1(terminated(
            separated_pair(identifier, tag(","), decimal_value),
            tag(";"),
        )),
        tag(";"),
    )(input)?;

    let (input, transitions) = many0(delimited(kw_on, transition, tag(";")))(input)?;

    Ok((input, (state_name, inputs, outputs, transitions)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier() {
        assert_eq!(identifier("stateA"), Ok(("", "stateA")));
        assert_eq!(identifier("Output_2"), Ok(("", "Output_2")));
        assert_eq!(
            identifier("123"),
            Err(nom::Err::Error(nom::error::make_error(
                "123",
                nom::error::ErrorKind::Satisfy
            )))
        );
    }

    #[test]
    fn test_decimal_value() {
        assert_eq!(decimal_value("42"), Ok(("", 42)));
        assert_eq!(decimal_value("0"), Ok(("", 0)));
        assert_eq!(decimal_value("123456789"), Ok(("", 123456789)));
        assert_eq!(
            decimal_value("abc"),
            Err(nom::Err::Error(nom::error::make_error(
                "abc",
                nom::error::ErrorKind::Digit
            )))
        );
    }

    #[test]
    fn test_keyword() {
        let kw_state = keyword("state");
        assert_eq!(kw_state("state"), Ok(("", "state".to_string())));
        assert_eq!(kw_state(" state "), Ok(("", "state".to_string())));
        assert_eq!(kw_state("stateA"), Ok(("A", "state".to_string())));
        assert_eq!(
            kw_state("input"),
            Err(nom::Err::Error(nom::error::make_error(
                "input",
                nom::error::ErrorKind::Tag
            )))
        );
    }

    #[test]
    fn test_symbol() {
        assert_eq!(symbol(";"), Ok(("", ';')));
        assert_eq!(symbol(", "), Ok((" ", ',')));
        assert_eq!(symbol("()"), Ok((")", '(')));
    }

    #[test]
    fn test_state() {
        assert_eq!(
            state(
                "state StateA:
                 input(1) Input1;
                 output(1) Output1;

                 on Input1 == 1 -> StateB {Output1 = 1};
                 on Input1 == 0 -> StateC {Output1 = 0};"
            ),
            Ok((
                "",
                (
                    "StateA",
                    vec![("Input1", 1)],
                    vec![("Output1", 1)],
                    vec![
                        Transition {
                            condition: ("Input1", "==", 1),
                            target: "StateB",
                            actions: vec![("Output1", 1)],
                        },
                        Transition {
                            condition: ("Input1", "==", 0),
                            target: "StateC",
                            actions: vec![("Output1", 0)],
                        }
                    ]
                )
            ))
        );
    }
}

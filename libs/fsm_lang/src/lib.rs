use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{multispace0, multispace1, space0, space1},
    character::is_alphanumeric,
    combinator::{map, recognize},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};

//use tree::structures::*;
//
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Comparison {
    EQ,
    NEQ,
    LE,
    LEQ,
    GE,
    GEQ,
}

impl ToString for Comparison {
    fn to_string(&self) -> String {
        match self {
            Comparison::EQ => "==".to_string(),
            Comparison::NEQ => "!=".to_string(),
            Comparison::LE => "<".to_string(),
            Comparison::LEQ => "<=".to_string(),
            Comparison::GE => ">".to_string(),
            Comparison::GEQ => ">=".to_string(),
        }
    }
}

/// # Нетерминал identifier
///
/// <identifier> ::= [a-zA-Z_][a-zA-Z0-9_]*
///
///
pub fn identifier(input: &str) -> IResult<&str, &str> {
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
pub fn decimal_value(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

/// # Ключевые слова
pub fn keyword<'a>(kw: &'a str) -> impl Fn(&str) -> IResult<&str, String> + 'a {
    move |input| {
        let (input, _) = multispace0(input)?;
        let (input, _) = tag(kw)(input)?;
        let (input, _) = multispace0(input)?;
        Ok((input, kw.to_string()))
    }
}

////////////////

pub fn FSM_parse(input: &str) -> IResult<&str, Vec<State>> {
    many1(state)(input)
}

#[derive(Debug, PartialEq)]
pub struct Transition<'a> {
    condition: (&'a str, Comparison, u32),
    target: &'a str,
    actions: Vec<(&'a str, u32)>,
}

///
fn action_single(input: &str) -> IResult<&str, (&str, u32)> {
    let (input, _) = multispace0(input)?;

    let (input, output_name) = identifier(input)?;
    let (input, _) = multispace0(input)?;

    let (input, _) = tag("=")(input)?;
    let (input, _) = multispace0(input)?;

    let (input, value) = decimal_value(input)?;

    Ok((input, (output_name, value)))
}

/// <action> ::= "{" <output_name> <assignment_operator> <value> "}"
pub fn action(input: &str) -> IResult<&str, Vec<(&str, u32)>> {
    let (input, _) = multispace0(input)?;

    let (input, _) = tag("{")(input)?;
    let (input, _) = multispace0(input)?;

    let (input, actions) =
        separated_list0(delimited(multispace0, tag(";"), multispace0), action_single)(input)?;
    let (input, _) = multispace0(input)?;

    let (input, _) = tag("}")(input)?;
    let (input, _) = multispace0(input)?;

    Ok((input, actions))
}

/// <comparison_operator> ::= "==" | "!=" | "<" | "<=" | ">" | ">="
pub fn comparison(input: &str) -> IResult<&str, Comparison> {
    alt((
        map(tag("=="), |_| Comparison::EQ),
        map(tag("!="), |_| Comparison::NEQ),
        map(tag("<="), |_| Comparison::LEQ),
        map(tag("<"), |_| Comparison::LE),
        map(tag(">="), |_| Comparison::GEQ),
        map(tag(">"), |_| Comparison::GE),
    ))(input)
}

/// <transition_action> ::= "on" <condition> "->" <state_name> <action> ";"
/// <condition> ::= <input_name> <comparison_operator> <value>
pub fn transition(input: &str) -> IResult<&str, Transition> {
    let (input, _) = multispace0(input)?;

    let (input, _) = tag("on")(input)?;
    let (input, _) = multispace0(input)?;

    let (input, source) = identifier(input)?;
    let (input, _) = multispace0(input)?;

    let (input, comp) = comparison(input)?;
    let (input, _) = multispace0(input)?;

    let (input, value) = decimal_value(input)?;
    let (input, _) = multispace0(input)?;

    let (input, _) = tag("->")(input)?;
    let (input, _) = multispace0(input)?;

    let (input, target) = identifier(input)?;
    let (input, _) = multispace0(input)?;

    let (input, actions) = action(input)?;
    let (input, _) = multispace0(input)?;

    Ok((
        input,
        Transition {
            condition: (source, comp, value),
            target,
            actions,
        },
    ))
}

/// <input_declaration> ::= "input" "(" <bit_width> ")" <input_name> ";"
/// <input_name> ::= <identifier>
/// TODO: отрабатывать лучше пробелы
pub fn input_declaration(input: &str) -> IResult<&str, (&str, u32)> {
    let (input, _) = keyword("input")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, size) = delimited(tag("("), decimal_value, tag(")"))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, (name, size)))
}

/// <output_declaration> ::= "output" "(" <bit_width> ")" <output_name> ";"
/// TODO: отрабатывать лучше пробелы
pub fn output_declaration(input: &str) -> IResult<&str, (&str, u32)> {
    let (input, _) = keyword("output")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, size) = delimited(tag("("), decimal_value, tag(")"))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, (name, size)))
}

/// <state_declaration> ::= "state" <state_name> ":" {<input_declaration> | <output_declaration> | <transition_action>}*
/// <state_name> ::= <identifier>
/// TODO: отрабатывать лучше пробелы
pub fn state(
    input: &str,
) -> IResult<&str, (&str, Vec<(&str, u32)>, Vec<(&str, u32)>, Vec<Transition>)> {
    let (input, _) = keyword("state")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, state_name) = identifier(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, inputs) = many0(terminated(input_declaration, tag(";")))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, outputs) = many0(terminated(output_declaration, tag(";")))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, transitions) = many0(terminated(transition, tag(";")))(input)?;
    let (input, _) = multispace0(input)?;
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
                            condition: ("Input1", Comparison::EQ, 1),
                            target: "StateB",
                            actions: vec![("Output1", 1)],
                        },
                        Transition {
                            condition: ("Input1", Comparison::EQ, 0),
                            target: "StateC",
                            actions: vec![("Output1", 0)],
                        }
                    ]
                )
            ))
        );
    }

    #[test]
    fn test_input_declaration() {
        assert_eq!(
            input_declaration("input(1) Input1"),
            Ok(("", ("Input1", 1)))
        );
        assert_eq!(
            input_declaration("input(8) Input_8"),
            Ok(("", ("Input_8", 8)))
        );
        assert_eq!(
            input_declaration("output(1) Output1"),
            Err(nom::Err::Error(nom::error::make_error(
                "output(1) Output1",
                nom::error::ErrorKind::Tag
            )))
        );
    }

    #[test]
    fn test_output_declaration() {
        assert_eq!(
            output_declaration("output(1) Output1"),
            Ok(("", ("Output1", 1)))
        );
        assert_eq!(
            output_declaration("output(8) Output_8"),
            Ok(("", ("Output_8", 8)))
        );
        assert_eq!(
            output_declaration("input(1) Input1"),
            Err(nom::Err::Error(nom::error::make_error(
                "input(1) Input1",
                nom::error::ErrorKind::Tag
            )))
        );
    }

    #[test]
    fn test_transition() {
        assert_eq!(
            transition("on Input1 == 1 -> StateB {Output1 = 1}"),
            Ok((
                "",
                Transition {
                    condition: ("Input1", Comparison::EQ, 1),
                    target: "StateB",
                    actions: vec![("Output1", 1)]
                }
            ))
        );

        assert_eq!(
            transition("on Input2 != 0 -> StateC {Output2 = 0; Output3 = 1}"),
            Ok((
                "",
                Transition {
                    condition: ("Input2", Comparison::NEQ, 0),
                    target: "StateC",
                    actions: vec![("Output2", 0), ("Output3", 1)]
                }
            ))
        );

        assert_eq!(
            transition("  on  Input1  ==  1  ->  StateB  { Output1  =  1  } "),
            Ok((
                "",
                Transition {
                    condition: ("Input1", Comparison::EQ, 1),
                    target: "StateB",
                    actions: vec![("Output1", 1)]
                }
            ))
        );

        assert_eq!(
            transition("on Input1 = 1 -> StateB {Output1 = 1}"),
            Err(nom::Err::Error(nom::error::make_error(
                "= 1 -> StateB {Output1 = 1}",
                nom::error::ErrorKind::Tag
            )))
        );
    }

    #[test]
    fn test_action_single() {
        assert_eq!(action_single("Output1 = 1"), Ok(("", ("Output1", 1))));
        assert_eq!(action_single("Output2=0"), Ok(("", ("Output2", 0))));
    }

    #[test]
    fn test_action() {
        assert_eq!(
            action("{Output1 = 1; Output2 = 0}"),
            Ok(("", vec![("Output1", 1), ("Output2", 0),]))
        );
        assert_eq!(action("{Output1 = 42}"), Ok(("", vec![("Output1", 42)])));
    }
}

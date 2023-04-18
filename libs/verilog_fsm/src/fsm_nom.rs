// use crate::fsm::{Output, State, Transition};
// use nom::{
//     bytes::complete::{tag, take_while},
//     character::complete::{alpha1, multispace0, multispace1},
//     combinator::{map, opt},
//     multi::many0,
//     sequence::{pair, preceded, tuple},
//     IResult,
// };
//
// fn identifier(input: &str) -> IResult<&str, String> {
//     map(alpha1, String::from)(input)
// }

// fn boolean_value(input: &str) -> IResult<&str, bool, nom::error::Error<&str>> {
//     map(tag("true"), |_| true)(input).or_else(|_| map(tag("false"), |_| false)(input))
// }

// fn output_definition(input: &str) -> IResult<&str, Output> {
//     let (input, _) = multispace0(input)?;
//     let (input, _) = tag("output")(input)?;
//     let (input, _) = multispace1(input)?;
//     let (input, signal) = identifier(input)?;
//     let (input, _) = multispace1(input)?;
//     let (input, value) = boolean_value(input)?;
//     Ok((input, Output { signal, value }))
// }
//
// fn transition_definition(input: &str) -> IResult<&str, Transition> {
//     let (input, _) = multispace0(input)?;
//     let (input, _) = tag("input")(input)?;
//     let (input, _) = multispace1(input)?;
//     let (input, input_signal) = identifier(input)?;
//     let (input, _) = multispace1(input)?;
//     let (input, input_value) = boolean_value(input)?;
//     let (input, _) = multispace1(input)?;
//     let (input, _) = tag("=>")(input)?;
//     let (input, _) = multispace1(input)?;
//     let (input, next_state) = identifier(input)?;
//     Ok((
//         input,
//         Transition {
//             input_signal,
//             input_value,
//             next_state,
//         },
//     ))
// }
//
// fn state_definition(input: &str) -> IResult<&str, State> {
//     let (input, _) = multispace0(input)?;
//     let (input, _) = tag("state")(input)?;
//     let (input, _) = multispace1(input)?;
//     let (input, name) = identifier(input)?;
//     let (input, outputs) = many0(preceded(multispace1, output_definition))(input)?;
//     let (input, transitions) = many0(preceded(multispace1, transition_definition))(input)?;
//     Ok((
//         input,
//         State {
//             name,
//             outputs,
//             transitions,
//         },
//     ))
// }

//pub fn parse_fsm(input: &str) -> Result<Vec<State>, nom::Err<&str>> {
//    match many0(preceded(multispace0, state_definition))(input) {
//        Ok((input, states)) => match multispace0(input) {
//            Ok(input1, _) => {
//                if input1.is_empty() {
//                    Ok(states)
//                } else {
//                    Err(nom::Err::Error(input1))
//                }
//            }
//            Err(e) => Err(e),
//        },
//        Err(e) => Err(e),
//    }
//}

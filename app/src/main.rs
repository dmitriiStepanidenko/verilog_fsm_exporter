use tree::fsm::*;
use tree::structures::*;

fn main() {
    // Создаем FSM
    //let fsm = MooreFSM {
    //    name: "m",
    //    states: vec![],
    //};

    //// Преобразуем FSM в Verilog модуль
    //let verilog_module = moore_fsm_to_verilog_module(&fsm);

    //// Дальше вы можете работать с verilog_module, например, экспортировать его в Verilog код
    //let result = verilog_module.export_verilog();
    //println!("{result}");
}

// let fsm = MooreFSM {
//     name: "my_fsm".to_string(),
//     states: vec![
//         State {
//             name: "s0".to_string(),
//             output: vec![
//                 (
//                     "out1".to_string(),
//                     Expression::Identifier("in1".to_string()),
//                 ),
//                 (
//                     "out2".to_string(),
//                     Expression::Identifier("in2".to_string()),
//                 ),
//             ],
//         },
//         State {
//             name: "s1".to_string(),
//             output: vec![
//                 (
//                     "out1".to_string(),
//                     Expression::Identifier("in2".to_string()),
//                 ),
//                 (
//                     "out2".to_string(),
//                     Expression::Identifier("in1".to_string()),
//                 ),
//             ],
//         },
//     ],
//     transitions: vec![
//         Transition {
//             from: "s0".to_string(),
//             to: "s1".to_string(),
//             condition: Expression::Binary(
//                 Box::new(Expression::Identifier("in1".to_string())),
//                 BinaryOp::Eq,
//                 Box::new(Expression::Number(Number::Decimal(0, "1".to_string()))),
//             ),
//         },
//         Transition {
//             from: "s1".to_string(),
//             to: "s0".to_string(),
//             condition: Expression::Binary(
//                 Box::new(Expression::Identifier("in2".to_string())),
//                 BinaryOp::Eq,
//                 Box::new(Expression::Number(Number::Decimal(0, "1".to_string()))),
//             ),
//         },
//     ],
// };

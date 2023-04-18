use crate::structures::*;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct State {
    pub name: String,
    pub output: Vec<(String, Option<Expression>)>,
}

#[derive(Clone, Debug)]
pub struct Transition {
    pub from: String,
    pub to: String,
    pub condition: Option<Expression>,
}

#[derive(Clone, Debug)]
pub struct MooreFSM {
    pub name: String,
    pub states: Vec<State>,
    pub transitions: Vec<Transition>,
}

//pub fn moore_fsm_to_verilog_module(fsm: &MooreFSM) -> Module {
//    let mut module = Module {
//        name: fsm.name.clone(),
//        statements: vec![],
//        ports: vec![],
//    };
//
//    let state_width = (fsm.states.len() as f64).log2().ceil() as u32;
//
//    for state in &fsm.states {
//        module.statements.push(Statement::LocalParam(LocalParam {
//            name: state.name.clone(),
//            value: Number::Decimal(0, state.name.clone()),
//            width: state_width,
//            is_signed: false,
//        }));
//    }
//
//    let mut always_statements = vec![];
//
//    for state in &fsm.states {
//        let mut case_items = vec![];
//        for (output_name, expression) in &state.output {
//            case_items.push((Some(output_name.clone()), expression.clone()));
//        }
//
//        always_statements.push(Statement::Case(Case {
//            expression: Expression::Identifier(state.name.clone()),
//            items: case_items,
//        }));
//    }
//
//    module.statements.push(Statement::Always(Always {
//        statements: always_statements,
//    }));
//
//    let mut transition_statements = vec![];
//
//    for transition in &fsm.transitions {
//        transition_statements.push(Statement::If(If {
//            condition: transition.condition.clone(),
//            then_statements: vec![Statement::Assign(Assign {
//                left: transition.from.clone(),
//                right: Expression::Identifier(transition.to.clone()),
//            })],
//            else_statements: vec![],
//        }));
//    }
//
//    module.statements.push(Statement::Always(Always {
//        statements: transition_statements,
//    }));
//
//    module
//}

/// TODO: доделать
fn get_width_from_state(output: &Option<Expression>) -> Option<u32> {
    None
}

pub fn mine_fsm_to_verilog_module(fsm: &MooreFSM) -> Module {
    let mut module = Module::new(&fsm.name, &vec![], &vec![]);

    /////////////////////////////////////////////// PORTS PART  ///////////////////////////////////////////////
    ////// Добавим все порты этого модуля
    module.ports.push(Port::Input(Input {
        name: "clk".to_string(),
        net_type: None,
        width: None,
        is_signed: false,
    }));

    module.ports.push(Port::Input(Input {
        name: "rst".to_string(),
        net_type: None,
        width: None,
        is_signed: false,
    }));

    let mut ports_names = HashSet::new();
    for (_i, state) in fsm.states.iter().enumerate() {
        for (port_name, output) in state.output.iter() {
            if ports_names.contains(&port_name) {
                continue;
            }
            module.ports.push(Port::Output(Output {
                name: port_name.to_string(),
                reg_net_type: Some(RegNetType::Reg(true)),
                width: get_width_from_state(output),
                is_signed: false,
            }));
            ports_names.insert(port_name);
        }
    }

    /////////////////////////////////////////////// STATES PART  ///////////////////////////////////////////////

    for state in &fsm.states {
        module.statements.push(Statement::LocalParam(LocalParam {
            name: state.name.clone(),
            value: Number::Binary(1, state.name[1..].to_string()),
            width: 1,
            is_signed: false,
        }));
    }

    for state in &fsm.states {
        for (output_name, output_expr) in &state.output {
            module.statements.push(Statement::Always(Always {
                statements: vec![Statement::If(If {
                    condition: Expression::Binary(
                        Box::new(Expression::Identifier("state".to_string())),
                        BinaryOp::Eq,
                        Box::new(Expression::Identifier(state.name.clone())),
                    ),
                    then_statements: vec![Statement::Assign(Assign {
                        left: output_name.clone(),
                        right: output_expr.clone().unwrap(),
                    })],
                    else_statements: vec![],
                })],
            }));
        }
    }

    module.statements.push(Statement::Register(Register {
        name: "state".to_string(),
        width: 1,
    }));

    for transition in &fsm.transitions {
        let condition = match transition.condition {
            None => vec![],
            Some(ref condition_expr) => vec![Statement::If(If {
                condition: condition_expr.clone(),
                then_statements: vec![],
                else_statements: vec![],
            })],
        };

        module.statements.push(Statement::Always(Always {
            statements: vec![Statement::If(If {
                condition: Expression::Binary(
                    Box::new(Expression::Identifier("state".to_string())),
                    BinaryOp::Eq,
                    Box::new(Expression::Identifier(transition.from.clone())),
                ),
                then_statements: vec![Statement::Assign(Assign {
                    left: "state".to_string(),
                    right: Expression::Identifier(transition.to.clone()),
                })],
                else_statements: condition,
            })],
        }));
    }

    return module;
}

#[cfg(test)]
mod tests {
    use super::*;

    mod moore_fsm_to_verilog_module_test_group {
        use super::*;

        #[test]
        ///  module two_state_counter (
        ///  input wire clk,
        ///  input wire rst,
        ///  output reg [1:0] out_state
        ///  );
        ///
        ///  localparam S0 = 1'b0; // начальное состояние
        ///  localparam S1 = l'b1; // второе состояние
        ///  
        ///  reg state; // регистр состояния
        ///
        ///  always @(posedge clk or posedge rst) begin
        ///    if (rst) begin
        ///      state <= S0;
        ///    end else begin
        ///      case (state)
        ///        S0: state <= S1;
        ///        S1: state <= S0;
        ///      endcase
        ///    end
        ///  end
        ///
        ///  always @(state) begin
        ///    case (state)
        ///      S0: out_state <= 0;
        ///      S1: out_state <= 1;
        ///    endcase
        ///  end
        ///
        /// endmodule
        fn test_export_verilog_for_localparam_two_counter() {
            let fsm = MooreFSM {
                name: "two_state_counter".to_string(),
                states: vec![
                    State {
                        name: "S0".to_string(),
                        output: vec![(
                            "out_state".to_string(),
                            Some(Expression::Number(Number::Binary(1, "0".to_string()))),
                        )],
                    },
                    State {
                        name: "S1".to_string(),
                        output: vec![(
                            "out_state".to_string(),
                            Some(Expression::Number(Number::Binary(1, "1".to_string()))),
                        )],
                    },
                ],
                transitions: vec![
                    Transition {
                        from: "S0".to_string(),
                        to: "S1".to_string(),
                        condition: None,
                    },
                    Transition {
                        from: "S1".to_string(),
                        to: "S0".to_string(),
                        condition: None,
                    },
                ],
            };
            let result = mine_fsm_to_verilog_module(&fsm);
            let expected_module = Module::new(
                &"two_state_counter".to_string(),
                &vec![
                    Statement::LocalParam(LocalParam {
                        name: "S0".to_string(),
                        value: Number::Binary(1, "0".to_string()),
                        width: 1,
                        is_signed: false,
                    }),
                    Statement::LocalParam(LocalParam {
                        name: "S1".to_string(),
                        value: Number::Binary(1, "1".to_string()),
                        width: 1,
                        is_signed: false,
                    }),
                    Statement::Register(Register {
                        name: "state".to_string(),
                        width: 1,
                    }),
                    Statement::Always(Always {
                        statements: vec![Statement::If(If {
                            condition: Expression::Identifier("rst".to_string()),
                            then_statements: vec![Statement::Assignment(Assignment {
                                name: "state".to_string(),
                                ass_type: OperationType::Sync,
                                right: Expression::Identifier("S0".to_string()),
                            })],
                            else_statements: vec![
                                Statement::Case(Case {
                                    expression: Expression::Identifier("state".to_string()),
                                    items: vec![(
                                        Some("S0".to_string()),
                                        Statement::Assignment(Assignment {
                                            name: "state".to_string(),
                                            right: Expression::Identifier("S1".to_string()),
                                            ass_type: OperationType::Sync,
                                        }),
                                    )],
                                }),
                                Statement::Case(Case {
                                    expression: Expression::Identifier("state".to_string()),
                                    items: vec![(
                                        Some("S1".to_string()),
                                        Statement::Assignment(Assignment {
                                            name: "state".to_string(),
                                            right: Expression::Identifier("S0".to_string()),
                                            ass_type: OperationType::Sync,
                                        }),
                                    )],
                                }),
                            ],
                        })],
                    }),
                    Statement::Always(Always {
                        statements: vec![Statement::Case(Case {
                            expression: Expression::Identifier("state".to_string()),
                            items: vec![
                                (
                                    Some("S0".to_string()),
                                    Statement::Assignment(Assignment {
                                        name: "out_state".to_string(),
                                        right: Expression::Number(Number::Binary(
                                            1,
                                            "0".to_string(),
                                        )),
                                        ass_type: OperationType::Sync,
                                    }),
                                ),
                                (
                                    Some("S1".to_string()),
                                    Statement::Assignment(Assignment {
                                        name: "out_state".to_string(),
                                        right: Expression::Number(Number::Binary(
                                            1,
                                            "1".to_string(),
                                        )),
                                        ass_type: OperationType::Sync,
                                    }),
                                ),
                            ],
                        })],
                    }),
                ],
                &vec![
                    Port::Input(Input {
                        name: "clk".to_string(),
                        net_type: None,
                        width: None,
                        is_signed: false,
                    }),
                    Port::Input(Input {
                        name: "rst".to_string(),
                        net_type: None,
                        width: None,
                        is_signed: false,
                    }),
                    Port::Output(Output {
                        name: "out_state".to_string(),
                        reg_net_type: Some(RegNetType::Reg(true)),
                        width: None,
                        is_signed: false,
                    }),
                ],
            );

            assert_eq!(result.name, expected_module.name);
            assert_eq!(result.ports, expected_module.ports);
            assert_eq!(result.statements, expected_module.statements);
        }

        #[test]
        ///  module three_state_counter (
        ///  input wire clk,
        ///  input wire rst,
        ///  output reg [1:0] out_state
        ///  );
        ///
        ///  parameter S0 = 2'b00; // начальное состояние
        ///  parameter S1 = 2'b01; // второе состояние
        ///  parameter S2 = 2'b10; // третье состояние
        ///  
        ///  reg [1:0] state; // регистр состояния
        ///
        ///  always @(posedge clk or posedge rst) begin
        ///    if (rst) begin
        ///      state <= S0;
        ///    end else begin
        ///      case (state)
        ///        S0: state <= S1;
        ///        S1: state <= S2;
        ///        S2: state <= S0;
        ///      endcase
        ///    end
        ///  end
        ///
        ///  always @(state) begin
        ///    case (state)
        ///      S0: out_state <= S0;
        ///      S1: out_state <= S1;
        ///      S2: out_state <= S2;
        ///    endcase
        ///  end
        ///
        /// endmodule
        fn test_export_verilog_for_localparam_three_counter() {}
    }
}

use crate::structures::*;

#[derive(Clone, Debug)]
pub struct State {
    pub name: String,
    pub output: Vec<(String, Expression)>,
}

#[derive(Clone, Debug)]
pub struct Transition {
    pub from: String,
    pub to: String,
    pub condition: Expression,
}

#[derive(Clone, Debug)]
pub struct MooreFSM {
    pub name: String,
    pub states: Vec<State>,
    pub transitions: Vec<Transition>,
}

pub fn moore_fsm_to_verilog_module(fsm: &MooreFSM) -> Module {
    let mut module = Module {
        name: fsm.name.clone(),
        statements: vec![],
        ports: vec![],
    };

    let state_width = (fsm.states.len() as f64).log2().ceil() as u32;

    for state in &fsm.states {
        module.statements.push(Statement::LocalParam(LocalParam {
            name: state.name.clone(),
            value: Number::Decimal(0, state.name.clone()),
            width: state_width,
            is_signed: false,
        }));
    }

    let mut always_statements = vec![];

    for state in &fsm.states {
        let mut case_items = vec![];
        for (output_name, expression) in &state.output {
            case_items.push((Some(output_name.clone()), expression.clone()));
        }

        always_statements.push(Statement::Case(Case {
            expression: Expression::Identifier(state.name.clone()),
            items: case_items,
        }));
    }

    module.statements.push(Statement::Always(Always {
        statements: always_statements,
    }));

    let mut transition_statements = vec![];

    for transition in &fsm.transitions {
        transition_statements.push(Statement::If(If {
            condition: transition.condition.clone(),
            then_statements: vec![Statement::Assign(Assign {
                left: transition.from.clone(),
                right: Expression::Identifier(transition.to.clone()),
            })],
            else_statements: vec![],
        }));
    }

    module.statements.push(Statement::Always(Always {
        statements: transition_statements,
    }));

    module
}

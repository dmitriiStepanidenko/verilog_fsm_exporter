//! # Базовые функции для струтур
//!
//! - new - создание
//! - add_(item) - добавленние элемента
//! - прочие

use crate::structures::*;
use std::fmt;

///////////// NEW

impl Module {
    pub fn new(name: &str, statements: &Vec<Statement>, ports: &Vec<Port>) -> Self {
        Module {
            name: name.to_string(),
            statements: statements.to_vec(),
            ports: ports.to_vec(),
        }
    }

    pub fn add_statement(&mut self, stmt: Statement) {
        self.statements.push(stmt);
    }

    pub fn add_port(&mut self, port: Port) {
        self.ports.push(port);
    }
    pub fn add_input(&mut self, input: Input) {
        self.ports.push(Port::Input(input));
    }
    pub fn add_output(&mut self, output: Output) {
        self.ports.push(Port::Output(output));
    }
    pub fn add_inout(&mut self, inout: Inout) {
        self.ports.push(Port::Inout(inout));
    }
}

impl Register {
    pub fn new(name: &str, width: u32) -> Self {
        Register {
            name: name.to_string(),
            width,
        }
    }
}

impl Wire {
    pub fn new(name: &str, width: u32) -> Self {
        Wire {
            name: name.to_string(),
            width,
        }
    }
}

impl Inout {
    pub fn new(name: &str, net_type: Option<NetType>, width: Option<u32>, is_signed: bool) -> Self {
        Inout {
            name: name.to_string(),
            net_type,
            width,
            is_signed,
        }
    }
}

impl Output {
    pub fn new(name: &str, net_type: Option<NetType>, width: Option<u32>, is_signed: bool) -> Self {
        Output {
            name: name.to_string(),
            net_type,
            width,
            is_signed,
        }
    }
}

impl Input {
    pub fn new(name: &str, net_type: Option<NetType>, width: Option<u32>, is_signed: bool) -> Self {
        Input {
            name: name.to_string(),
            net_type,
            width,
            is_signed,
        }
    }
}

impl LocalParam {
    pub fn new(name: &str, value: &Number, is_signed: Option<bool>, width: Option<u32>) -> Self {
        LocalParam {
            name: name.to_string(),
            value: value.clone(),
            is_signed: match is_signed {
                Some(x) => x,
                None => false,
            },
            width: match width {
                Some(x) => x,
                None => get_usize_from_number(value),
            },
        }
    }
}

//////////////

impl NetType {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "wire" => Some(NetType::Wire),
            _ => None,
        }
    }
}

impl ToString for NetType {
    fn to_string(&self) -> String {
        match *self {
            NetType::Wire => "wire".to_string(),
        }
    }
}

fn get_usize_from_number(number: &Number) -> u32 {
    match number {
        Number::Binary(width, _) => *width,
        Number::Octal(width, _) => *width,
        Number::Decimal(width, _) => *width,
        Number::Hex(width, _) => *width,
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Binary(size, value) => write!(f, "{}'b{}", size, value),
            Number::Octal(size, value) => write!(f, "{}'o{}", size, value),
            Number::Decimal(size, value) => write!(f, "{}'d{}", size, value),
            Number::Hex(size, value) => write!(f, "{}'h{}", size, value),
        }
    }
}

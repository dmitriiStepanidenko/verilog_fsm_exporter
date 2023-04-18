//! # Представление структур данных

pub trait ExportVerilog {
    fn export_verilog(&self) -> String;
}

/// net_type ::=
/// supply0 | supply1 | tri | triand | trior | tri0 | tri1 | wire | wand | wor
/// Right now support only wire
#[derive(Clone, Debug)]
pub enum NetType {
    // Supply0,
    // Supply1,
    // Tri,
    // Triand,
    // Trior,
    // Tri0,
    // Tri1,
    Wire,
    // Wand,
    // Wor,
}

#[derive(Clone, Debug)]
pub enum Statement {
    Wire(Wire),
    Register(Register),
    Assign(Assign),
    Always(Always),
    LocalParam(LocalParam),
    If(If),
    Case(Case),
}

pub fn statement_export_verilog(stmt: &Statement) -> String {
    match stmt {
        Statement::Register(reg) => reg.export_verilog(),
        Statement::Wire(wire) => wire.export_verilog(),
        Statement::Assign(assign) => assign.export_verilog(),
        Statement::Always(always) => always.export_verilog(),
        Statement::LocalParam(param) => param.export_verilog(),
        Statement::If(x) => x.export_verilog(),
        Statement::Case(x) => x.export_verilog(),
    }
}

#[derive(Clone, Debug)]
/// TODO: Заменить строку на ссылку
pub struct Assign {
    pub left: String,
    pub right: Expression,
}

#[derive(Clone, Debug)]
pub struct Always {
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug)]
/// TODO: Заменить строку на ссылку
pub enum Expression {
    Identifier(String),
    Unary(UnaryOp, Box<Expression>),
    Binary(Box<Expression>, BinaryOp, Box<Expression>),
}

#[derive(Clone, Debug)]
pub enum UnaryOp {
    Not,
}

#[derive(Clone, Debug)]
pub enum BinaryOp {
    And,
    Or,
    Eq,
}

#[derive(Clone, Debug)]
pub enum Port {
    Input(Input),
    Output(Output),
    Inout(Inout),
}

trait PortTrait {
    fn what_type(&self) -> String;
}

#[derive(Clone, Debug)]
pub struct If {
    pub condition: Expression,
    pub then_statements: Vec<Statement>,
    pub else_statements: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub struct Case {
    pub expression: Expression,
    pub items: Vec<(Option<String>, Expression)>,
}

/// input_declaration ::=
/// input ( net_type )? ( signed )? ( range )? list_of_port_identifiers
#[derive(Clone, Debug)]
pub struct Input {
    pub name: String,
    pub net_type: Option<NetType>,
    pub width: Option<u32>,
    pub is_signed: bool,
}

/// inout_declaration ::=
/// inout ( <net_type> )? ( signed )? ( <range> )? <list_of_port_identifiers>
#[derive(Clone, Debug)]
pub struct Inout {
    pub name: String,
    pub net_type: Option<NetType>,
    pub width: Option<u32>,
    pub is_signed: bool,
}

/// output_declaration ::=
/// output ( net_type )? ( signed )? ( range )? list_of_port_identifiers
/// |
/// output ( reg )? ( signed )? ( range )? list_of_port_identifiers
/// |
/// output reg ( signed )? ( range )? list_of_variable_port_identifiers
/// |
/// output ( output_variable_type )? list_of_port_identifiers
/// |
/// output output_variable_type list_of_variable_port_identifiers
#[derive(Clone, Debug)]
pub struct Output {
    pub name: String,
    pub net_type: Option<NetType>,
    pub width: Option<u32>,
    pub is_signed: bool,
}

#[derive(Clone, Debug)]
pub struct Assignment {
    pub name: String,
}

pub struct Module {
    pub name: String,
    pub statements: Vec<Statement>,
    pub ports: Vec<Port>,
}

#[derive(Clone, Debug)]
/// TODO: заменить на NetType. Wire-это частное
pub struct Wire {
    pub name: String,
    pub width: u32,
}

#[derive(Clone, Debug)]
pub struct Register {
    pub name: String,
    pub width: u32,
}

#[derive(Clone, Debug)]
pub struct LocalParam {
    pub name: String,
    pub value: Number,
    pub width: u32,
    pub is_signed: bool,
}

// number ::=
// decimal_number
// | octal_number
// | binary_number
// | hex_number
// | real_number real_number

#[derive(Clone, Debug)]
/// TODO: real_number
pub enum Number {
    Binary(u32, String),
    Octal(u32, String),
    Decimal(u32, String),
    Hex(u32, String),
}

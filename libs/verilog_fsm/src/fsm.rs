#[derive(Debug, PartialEq)]
pub struct State {
    pub name: String,
    pub outputs: Vec<Output>,
    pub transitions: Vec<Transition>,
}

#[derive(Debug, PartialEq)]
pub struct Output {
    pub signal: String,
    pub value: bool,
}

#[derive(Debug, PartialEq)]
pub struct Transition {
    pub input_signal: String,
    pub input_value: bool,
    pub next_state: String,
}

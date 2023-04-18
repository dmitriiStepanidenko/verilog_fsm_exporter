//! # Функции для структрур данных для экспорта
//!
//! в будущем отдельный crate

use crate::structures::*;

impl ExportVerilog for Module {
    fn export_verilog(&self) -> String {
        let mut output = format!("module {}", self.name);

        for (i, port) in self.ports.iter().enumerate() {
            if i == 0 {
                output += "\n(\n"
            }
            output += match port {
                Port::Input(input) => input.export_verilog(),
                Port::Output(output) => output.export_verilog(),
                Port::Inout(inout) => inout.export_verilog(),
            }
            .as_str();
            if i < self.ports.len() - 1 {
                output.push_str(",\n");
            }
            if i == self.ports.len() - 1 {
                output += "\n)"
            }
        }
        output += ";\n";

        for (_i, statement) in self.statements.iter().enumerate() {
            output += statement_export_verilog(statement).as_str();
            output += "\n";
        }

        output += "endmodule";
        output
    }
}

struct CommonPortDeclarations {}

impl CommonPortDeclarations {
    fn export_verilog(
        direction: &str,
        name: &String,
        net_type: &Option<NetType>,
        width: &Option<u32>,
        is_signed: &bool,
    ) -> String {
        let net_type_string = match net_type {
            Some(x) => format!("{} ", x.to_string()),
            None => String::new(),
        };
        let width_str = match width {
            Some(width) => format!("[{}:0] ", width - 1),
            None => String::new(),
        };
        let signed_str = if *is_signed { "signed " } else { "" };
        format!("{direction} {net_type_string}{signed_str}{width_str}{name}")
    }
}

impl ExportVerilog for Inout {
    /// inout_declaration ::=
    /// inout ( <net_type> )? ( signed )? ( <range> )? <list_of_port_identifiers>
    fn export_verilog(&self) -> String {
        CommonPortDeclarations::export_verilog(
            "inout",
            &self.name,
            &self.net_type,
            &self.width,
            &self.is_signed,
        )
    }
}

impl ExportVerilog for Input {
    /// input_declaration ::=
    /// input ( net_type )? ( signed )? ( range )? list_of_port_identifiers
    fn export_verilog(&self) -> String {
        CommonPortDeclarations::export_verilog(
            "input",
            &self.name,
            &self.net_type,
            &self.width,
            &self.is_signed,
        )
    }
}

impl ExportVerilog for Output {
    /// output_declaration ::=
    /// output ( net_type )? ( signed )? ( range )? list_of_port_identifiers
    fn export_verilog(&self) -> String {
        CommonPortDeclarations::export_verilog(
            "output",
            &self.name,
            &self.net_type,
            &self.width,
            &self.is_signed,
        )
    }
}

struct CommonRegAndWireFunctions {}

impl CommonRegAndWireFunctions {
    fn export_verilog(width: u32, type_name: String, name: String) -> String {
        // TODO: не помешал бы механизм, позволяющий объявлять в одной строке объявления с
        // одинковой битностью.
        if width == 1 {
            format!("{} {};", type_name, name)
        } else {
            format!("{} [{}:0] {};", type_name, width - 1, name)
        }
    }
}

struct CommonWidthExport {}
impl CommonWidthExport {
    fn export_width(width: u32) -> String {
        if width == 1 {
            "".to_string()
        } else {
            format!("[{}:0] ", width - 1)
        }
    }
}

impl ExportVerilog for Register {
    fn export_verilog(&self) -> String {
        // TODO: не помешал бы механизм, позволяющий объявлять в одной строке объявления с
        // одинковой битностью.
        // TODO: двумерные вектора
        CommonRegAndWireFunctions::export_verilog(self.width, "reg".to_string(), self.name.clone())
    }
}

impl ExportVerilog for Wire {
    fn export_verilog(&self) -> String {
        // TODO: не помешал бы механизм, позволяющий объявлять в одной строке объявления с
        // одинаковой битностью.
        CommonRegAndWireFunctions::export_verilog(self.width, "wire".to_string(), self.name.clone())
    }
}

impl ExportVerilog for LocalParam {
    // local_parameter_declaration ::=
    // localparam ( signed )? ( range )? list_of_param_assignments ;
    fn export_verilog(&self) -> String {
        let signed_str = if self.is_signed { "signed " } else { "" };
        let width_str = CommonWidthExport::export_width(self.width);
        format!(
            "localparam {signed_str}{width_str}{} = {};",
            self.name, self.value
        )
    }
}

impl ExportVerilog for Assign {
    fn export_verilog(&self) -> String {
        format!("assign {} = {};", self.left, self.right.export_verilog())
    }
}

impl ExportVerilog for Expression {
    fn export_verilog(&self) -> String {
        match self {
            Expression::Identifier(id) => id.clone(),
            Expression::Unary(op, expr) => {
                format!("{}{}", op.export_verilog(), expr.export_verilog())
            }
            Expression::Binary(lhs, op, rhs) => {
                format!(
                    "{} {} {}",
                    lhs.export_verilog(),
                    op.export_verilog(),
                    rhs.export_verilog()
                )
            }
        }
    }
}

impl ExportVerilog for UnaryOp {
    fn export_verilog(&self) -> String {
        match self {
            UnaryOp::Not => "~".to_string(),
        }
    }
}

impl ExportVerilog for BinaryOp {
    fn export_verilog(&self) -> String {
        match self {
            BinaryOp::And => "&".to_string(),
            BinaryOp::Or => "|".to_string(),
            BinaryOp::Eq => "==".to_string(),
        }
    }
}

impl ExportVerilog for Always {
    fn export_verilog(&self) -> String {
        let mut s = String::from("always @ (*) begin\n");

        for stmt in &self.statements {
            s.push_str(&format!("    {}\n", statement_export_verilog(stmt)));
        }

        s.push_str("end");
        s
    }
}

impl ExportVerilog for If {
    fn export_verilog(&self) -> String {
        let mut s = format!("if ({}) begin\n", self.condition.export_verilog());

        for stmt in &self.then_statements {
            s.push_str(&format!("    {}\n", statement_export_verilog(stmt)));
        }

        if !self.else_statements.is_empty() {
            s.push_str("end else begin\n");

            for stmt in &self.else_statements {
                s.push_str(&format!("    {}\n", statement_export_verilog(stmt)));
            }
        }

        s.push_str("end");
        s
    }
}

impl ExportVerilog for Case {
    fn export_verilog(&self) -> String {
        let mut s = format!("case ({})\n", self.expression.export_verilog());

        for (name, expr) in &self.items {
            if let Some(name) = name {
                s.push_str(&format!("    {}: {}\n", name, expr.export_verilog()));
            } else {
                s.push_str(&format!("    default: {}\n", expr.export_verilog()));
            }
        }

        s.push_str("endcase");
        s
    }
}

/////////////////////////////////////////// TESTS

#[cfg(test)]
mod tests {
    use super::*;

    mod ports_export_group {
        use super::*;

        /// Check signed output
        #[test]
        fn test_export_verilog_for_output_signed_width_2() {
            let testing_inout = Output::new("output_name", Some(NetType::Wire), Some(2), true);
            let expected_module_string = "output wire signed [1:0] output_name".to_string();
            assert_eq!(
                expected_module_string,
                testing_inout.export_verilog(),
                "We are testing empty module export"
            );
        }

        /// Check inout with type Wire
        #[test]
        fn test_export_verilog_for_inout_wire() {
            let testing_inout = Inout::new("inout_name", Some(NetType::Wire), None, false);
            let expected_module_string = "inout wire inout_name".to_string();
            assert_eq!(
                expected_module_string,
                testing_inout.export_verilog(),
                "We are testing empty module export"
            );
        }
        /// Check inout without type
        #[test]
        fn test_export_verilog_for_inout_without_nettype() {
            let testing_inout = Inout::new("inout_name", None, None, false);
            let expected_module_string = "inout inout_name".to_string();
            assert_eq!(
                expected_module_string,
                testing_inout.export_verilog(),
                "We are testing empty module export"
            );
        }
    }
    mod module_export_group {
        use super::*;

        /// Check empty module export
        #[test]
        fn test_export_verilog_for_empty_module() {
            let testing_module =
                Module::new("module_name", &Vec::<Statement>::new(), &Vec::<Port>::new());
            let expected_module_string = "module module_name;\nendmodule".to_string();
            assert_eq!(
                expected_module_string,
                testing_module.export_verilog(),
                "We are testing empty module export"
            );
        }

        /// Check module with inputs and outputs export
        #[test]
        fn test_export_verilog_for_input_output_module() {
            let testing_module = Module::new(
                "module_name",
                &Vec::<Statement>::new(),
                &vec![
                    Port::Output(Output::new(
                        "output_name",
                        Some(NetType::Wire),
                        Some(2),
                        true,
                    )),
                    Port::Input(Input::new("input_name", Some(NetType::Wire), None, false)),
                ],
            );
            let expected_module_string =
                "module module_name\n(\noutput wire signed [1:0] output_name,\ninput wire input_name\n);\nendmodule"
                    .to_string();
            assert_eq!(
                expected_module_string,
                testing_module.export_verilog(),
                "We are testing empty module export"
            );
        }

        /// Check module with wires and regs
        #[test]
        fn test_export_verilog_wire_reg_statements() {
            let testing_module = Module::new(
                "module_name",
                &vec![
                    Statement::Wire(Wire::new("wire_name", 3)),
                    Statement::Register(Register::new("reg_name", 2)),
                ],
                &Vec::<Port>::new(),
            );
            let expected_module_string =
                "module module_name;\nwire [2:0] wire_name;\nreg [1:0] reg_name;\nendmodule"
                    .to_string();
            assert_eq!(
                expected_module_string,
                testing_module.export_verilog(),
                "We are testing empty module export"
            );
        }
    }

    mod wire_export_group {
        use super::*;

        /// Check one dimension wire export
        #[test]
        fn test_export_verilog_for_wire_1_dimension() {
            let testing_wire = Wire::new("wire_name", 1);
            let expected_wire_string = "wire wire_name;".to_string();
            assert_eq!(
                expected_wire_string,
                testing_wire.export_verilog(),
                "We are testing 1-dimension wire definition"
            );
        }

        /// Check two-dimension wire export
        #[test]
        fn test_export_verilog_for_wire_two_dimension() {
            let testing_wire = Wire::new("wire_name", 2);
            let expected_wire_string = "wire [1:0] wire_name;".to_string();
            assert_eq!(
                expected_wire_string,
                testing_wire.export_verilog(),
                "We are testing 2-dimension wire definition"
            );
        }

        /// Check 999-dimension wire export
        #[test]
        fn test_export_verilog_for_wire_999_dimension() {
            let testing_wire = Wire::new("wire_name", 999);
            let expected_wire_string = "wire [998:0] wire_name;".to_string();
            assert_eq!(
                expected_wire_string,
                testing_wire.export_verilog(),
                "We are testing 999-dimension wire definition"
            );
        }
    }

    mod localparam_export_group {
        use super::*;

        #[test]
        fn test_export_verilog_for_localparam_binary() {
            let localparam = LocalParam::new(
                "param_name",
                &Number::Binary(4, "1010".to_string()),
                None,
                None,
            );
            let expected_output = "localparam [3:0] param_name = 4'b1010;".to_string();
            assert_eq!(
                expected_output,
                localparam.export_verilog(),
                "We are testing localparam with binary value"
            );
        }

        #[test]
        fn test_export_verilog_for_localparam_octal() {
            let localparam = LocalParam::new(
                "param_name",
                &Number::Octal(3, "123".to_string()),
                None,
                None,
            );
            let expected_output = "localparam [2:0] param_name = 3'o123;".to_string();
            assert_eq!(
                expected_output,
                localparam.export_verilog(),
                "We are testing localparam with octal value"
            );
        }

        #[test]
        fn test_export_verilog_for_localparam_decimal() {
            let localparam = LocalParam::new(
                "param_name",
                &Number::Decimal(3, "456".to_string()),
                None,
                None,
            );
            let expected_output = "localparam [2:0] param_name = 3'd456;".to_string();
            assert_eq!(
                expected_output,
                localparam.export_verilog(),
                "We are testing localparam with decimal value"
            );
        }

        #[test]
        fn test_export_verilog_for_localparam_hex() {
            let localparam = LocalParam::new(
                "param_name",
                &Number::Hex(4, "1A2B".to_string()),
                None,
                None,
            );
            let expected_output = "localparam [3:0] param_name = 4'h1A2B;".to_string();
            assert_eq!(
                expected_output,
                localparam.export_verilog(),
                "We are testing localparam with hex value"
            );
        }
    }
}

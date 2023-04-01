//! Hack to get const Parameter to parameter index mapping

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

include!("./src/parameters/list.rs");

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    write!(
        &mut file,
        "const fn parameter_to_index(parameter: Parameter) -> u8 {{ match parameter {{"
    )
    .unwrap();

    for (parameter_index, parameter) in PARAMETERS.iter().copied().enumerate() {
        match parameter {
            Parameter::None => unreachable!(),
            Parameter::Master(p) => writeln!(
                &mut file,
                "Parameter::Master(MasterParameter::{:?}) => {},",
                p, parameter_index
            )
            .unwrap(),
            Parameter::Operator(operator_index, p) => writeln!(
                &mut file,
                "Parameter::Operator({}, OperatorParameter::{:?}) => {},",
                operator_index, p, parameter_index
            )
            .unwrap(),
            Parameter::Lfo(lfo_index, p) => writeln!(
                &mut file,
                "Parameter::Lfo({}, LfoParameter::{:?}) => {},",
                lfo_index, p, parameter_index
            )
            .unwrap(),
        };
    }

    write!(&mut file, "_ => unreachable!(),").unwrap();
    writeln!(&mut file, "}}}}").unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}

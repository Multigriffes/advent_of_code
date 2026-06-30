use std::{cell::RefCell, collections::HashMap, env};

use input_file_lib::get_file_content_to_string;

#[allow(dead_code)]
struct Wire {
    id: String,
    has_value: RefCell<bool>,
    value: RefCell<Value>,
    provider: Provider,
}

impl Wire {
    fn new(id: String, provider: Provider) -> Self {
        Wire {
            id,
            has_value: RefCell::new(false),
            value: RefCell::new(Value::None),
            provider,
        }
    }

    fn get(&self, circuit: &HashMap<String, Wire>) -> u16 {
        if !*self.has_value.borrow() {
            *self.value.borrow_mut() = Value::Set(self.provider.get(circuit));
            //println!("Cached")
        }
        match *self.value.borrow() {
            Value::Set(value) => {
                //println!("Wire: {} Value: {}", self.id, value);
                *self.has_value.borrow_mut() = true;
                value
            }
            _ => 0,
        }
    }
}

enum Provider {
    Wire(String),
    Gate(Box<Gate>),
    Value(u16),
}

impl Provider {
    fn get(&self, circuit: &HashMap<String, Wire>) -> u16 {
        match self {
            Provider::Wire(wire_id) => circuit.get(wire_id).unwrap().get(circuit),
            Provider::Gate(gate) => gate.get(circuit),
            Provider::Value(value) => *value,
        }
    }
}

enum Gate {
    Not(Provider),
    And(Provider, Provider),
    Or(Provider, Provider),
    Rshift(Provider, Provider),
    Lshift(Provider, Provider),
}

impl Gate {
    fn get(&self, circuit: &HashMap<String, Wire>) -> u16 {
        match self {
            Self::Not(provider) => !provider.get(circuit),
            Self::And(left_provider, right_provider) => {
                left_provider.get(circuit) & right_provider.get(circuit)
            }
            Self::Or(left_provider, right_provider) => {
                left_provider.get(circuit) | right_provider.get(circuit)
            }
            Self::Lshift(left_provider, right_provider) => {
                left_provider.get(circuit) << right_provider.get(circuit)
            }
            Self::Rshift(left_provider, right_provider) => {
                left_provider.get(circuit) >> right_provider.get(circuit)
            }
        }
    }
}

enum Value {
    None,
    Set(u16),
}

#[derive(PartialEq, Eq)]
enum Word {
    Not,
    And,
    Or,
    Pipe,
    Id(String),
    Value(u16),
    Nothing,
    Rshift,
    Lshift,
}

fn get_type_of_word(word: &str) -> Word {
    match word {
        "AND" => Word::And,
        "OR" => Word::Or,
        "NOT" => Word::Not,
        "RSHIFT" => Word::Rshift,
        "LSHIFT" => Word::Lshift,
        "->" => Word::Pipe,
        _ => match word.parse::<u16>() {
            Ok(value) => Word::Value(value),
            Err(_) => {
                if word.trim() == "" {
                    Word::Nothing
                } else {
                    Word::Id(word.to_string())
                }
            }
        },
    }
}

fn main() -> Result<(), String> {
    let default_path: String = String::from("./solutions/year_2015/src/input/day7.txt");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: dayX <input_file>");
        println!("input_file could be 'x' or 'default' for the default one");
        std::process::exit(1);
    }

    let string_to_compute = match args[1].to_lowercase().as_str() {
        "x" | "default" => get_file_content_to_string(&default_path)?,
        _ => get_file_content_to_string(&args[1])?,
    };
    let mut circuit: HashMap<String, Wire> = HashMap::new();

    for line in string_to_compute.lines() {
        //println!("{}", line);
        let words: Vec<Word> = line
            .split(" ")
            .map(|x| get_type_of_word(x))
            .filter(|x| !(*x == Word::Nothing))
            .collect();

        match words.as_slice() {
            // NOT
            [
                Word::Not,
                Word::Id(wire_id),
                Word::Pipe,
                Word::Id(resultant_wire_id),
            ] => {
                let gate = Gate::Not(Provider::Wire(wire_id.clone()));
                let provider = Provider::Gate(Box::new(gate));
                let resultant_wire = Wire::new(resultant_wire_id.clone(), provider);
                circuit.insert(resultant_wire_id.clone(), resultant_wire);
            } // NOT
            [
                Word::Not,
                Word::Value(value),
                Word::Pipe,
                Word::Id(resultant_wire_id),
            ] => {
                let gate = Gate::Not(Provider::Value(*value));
                let provider = Provider::Gate(Box::new(gate));
                let resultant_wire = Wire::new(resultant_wire_id.clone(), provider);
                circuit.insert(resultant_wire_id.clone(), resultant_wire);
            } // NOT
            // ASSIGN
            [Word::Id(wire_id), Word::Pipe, Word::Id(resultant_wire_id)] => {
                let provider = Provider::Wire(wire_id.clone());
                let resultant_wire = Wire::new(resultant_wire_id.clone(), provider);
                circuit.insert(resultant_wire_id.clone(), resultant_wire);
            } // ASSIGN
            [Word::Value(value), Word::Pipe, Word::Id(resultant_wire_id)] => {
                let provider = Provider::Value(*value);
                let resultant_wire = Wire::new(resultant_wire_id.clone(), provider);
                circuit.insert(resultant_wire_id.clone(), resultant_wire);
            } // ASSIGN
            // AND
            [
                Word::Id(left_wire_id),
                Word::And,
                Word::Id(right_wire_id),
                Word::Pipe,
                Word::Id(resultant_wire_id),
            ] => {
                let (left_provider, right_provider) = (
                    Provider::Wire(left_wire_id.clone()),
                    Provider::Wire(right_wire_id.clone()),
                );
                let gate = Gate::And(left_provider, right_provider);
                let provider = Provider::Gate(Box::new(gate));
                let resultant_wire = Wire::new(resultant_wire_id.clone(), provider);
                circuit.insert(resultant_wire_id.clone(), resultant_wire);
            } // AND
            [
                Word::Value(left_value),
                Word::And,
                Word::Id(right_wire_id),
                Word::Pipe,
                Word::Id(resultant_wire_id),
            ] => {
                let (left_provider, right_provider) = (
                    Provider::Value(*left_value),
                    Provider::Wire(right_wire_id.clone()),
                );
                let gate = Gate::And(left_provider, right_provider);
                let provider = Provider::Gate(Box::new(gate));
                let resultant_wire = Wire::new(resultant_wire_id.clone(), provider);
                circuit.insert(resultant_wire_id.clone(), resultant_wire);
            } // AND
            [
                Word::Value(left_value),
                Word::And,
                Word::Value(right_value),
                Word::Pipe,
                Word::Id(resultant_wire_id),
            ] => {
                let (left_provider, right_provider) =
                    (Provider::Value(*left_value), Provider::Value(*right_value));
                let gate = Gate::And(left_provider, right_provider);
                let provider = Provider::Gate(Box::new(gate));
                let resultant_wire = Wire::new(resultant_wire_id.clone(), provider);
                circuit.insert(resultant_wire_id.clone(), resultant_wire);
            } // AND
            [
                Word::Id(left_wire_id),
                Word::And,
                Word::Value(right_value),
                Word::Pipe,
                Word::Id(resultant_wire_id),
            ] => {
                let (left_provider, right_provider) = (
                    Provider::Wire(left_wire_id.clone()),
                    Provider::Value(*right_value),
                );
                let gate = Gate::And(left_provider, right_provider);
                let provider = Provider::Gate(Box::new(gate));
                let resultant_wire = Wire::new(resultant_wire_id.clone(), provider);
                circuit.insert(resultant_wire_id.clone(), resultant_wire);
            } // AND
            // OR
            [
                Word::Id(left_wire_id),
                Word::Or,
                Word::Id(right_wire_id),
                Word::Pipe,
                Word::Id(resultant_wire_id),
            ] => {
                let (left_provider, right_provider) = (
                    Provider::Wire(left_wire_id.clone()),
                    Provider::Wire(right_wire_id.clone()),
                );
                let gate = Gate::Or(left_provider, right_provider);
                let provider = Provider::Gate(Box::new(gate));
                let resultant_wire = Wire::new(resultant_wire_id.clone(), provider);
                circuit.insert(resultant_wire_id.clone(), resultant_wire);
            } // OR
            [
                Word::Value(left_value),
                Word::Or,
                Word::Id(right_wire_id),
                Word::Pipe,
                Word::Id(resultant_wire_id),
            ] => {
                let (left_provider, right_provider) = (
                    Provider::Value(*left_value),
                    Provider::Wire(right_wire_id.clone()),
                );
                let gate = Gate::Or(left_provider, right_provider);
                let provider = Provider::Gate(Box::new(gate));
                let resultant_wire = Wire::new(resultant_wire_id.clone(), provider);
                circuit.insert(resultant_wire_id.clone(), resultant_wire);
            } // OR
            [
                Word::Value(left_value),
                Word::Or,
                Word::Value(right_value),
                Word::Pipe,
                Word::Id(resultant_wire_id),
            ] => {
                let (left_provider, right_provider) =
                    (Provider::Value(*left_value), Provider::Value(*right_value));
                let gate = Gate::Or(left_provider, right_provider);
                let provider = Provider::Gate(Box::new(gate));
                let resultant_wire = Wire::new(resultant_wire_id.clone(), provider);
                circuit.insert(resultant_wire_id.clone(), resultant_wire);
            } // OR
            [
                Word::Id(left_wire_id),
                Word::Or,
                Word::Value(right_value),
                Word::Pipe,
                Word::Id(resultant_wire_id),
            ] => {
                let (left_provider, right_provider) = (
                    Provider::Wire(left_wire_id.clone()),
                    Provider::Value(*right_value),
                );
                let gate = Gate::Or(left_provider, right_provider);
                let provider = Provider::Gate(Box::new(gate));
                let resultant_wire = Wire::new(resultant_wire_id.clone(), provider);
                circuit.insert(resultant_wire_id.clone(), resultant_wire);
            } // OR
            // RHSIFT
            [
                Word::Id(left_wire_id),
                Word::Rshift,
                Word::Id(right_wire_id),
                Word::Pipe,
                Word::Id(resultant_wire_id),
            ] => {
                let (left_provider, right_provider) = (
                    Provider::Wire(left_wire_id.clone()),
                    Provider::Wire(right_wire_id.clone()),
                );
                let gate = Gate::Rshift(left_provider, right_provider);
                let provider = Provider::Gate(Box::new(gate));
                let resultant_wire = Wire::new(resultant_wire_id.clone(), provider);
                circuit.insert(resultant_wire_id.clone(), resultant_wire);
            } // RHSIFT
            [
                Word::Value(left_value),
                Word::Rshift,
                Word::Id(right_wire_id),
                Word::Pipe,
                Word::Id(resultant_wire_id),
            ] => {
                let (left_provider, right_provider) = (
                    Provider::Value(*left_value),
                    Provider::Wire(right_wire_id.clone()),
                );
                let gate = Gate::Rshift(left_provider, right_provider);
                let provider = Provider::Gate(Box::new(gate));
                let resultant_wire = Wire::new(resultant_wire_id.clone(), provider);
                circuit.insert(resultant_wire_id.clone(), resultant_wire);
            } // RHSIFT
            [
                Word::Value(left_value),
                Word::Rshift,
                Word::Value(right_value),
                Word::Pipe,
                Word::Id(resultant_wire_id),
            ] => {
                let (left_provider, right_provider) =
                    (Provider::Value(*left_value), Provider::Value(*right_value));
                let gate = Gate::Rshift(left_provider, right_provider);
                let provider = Provider::Gate(Box::new(gate));
                let resultant_wire = Wire::new(resultant_wire_id.clone(), provider);
                circuit.insert(resultant_wire_id.clone(), resultant_wire);
            } // RHSIFT
            [
                Word::Id(left_wire_id),
                Word::Rshift,
                Word::Value(right_value),
                Word::Pipe,
                Word::Id(resultant_wire_id),
            ] => {
                let (left_provider, right_provider) = (
                    Provider::Wire(left_wire_id.clone()),
                    Provider::Value(*right_value),
                );
                let gate = Gate::Rshift(left_provider, right_provider);
                let provider = Provider::Gate(Box::new(gate));
                let resultant_wire = Wire::new(resultant_wire_id.clone(), provider);
                circuit.insert(resultant_wire_id.clone(), resultant_wire);
            } // RHSIFT
            // LSHIFT
            [
                Word::Id(left_wire_id),
                Word::Lshift,
                Word::Id(right_wire_id),
                Word::Pipe,
                Word::Id(resultant_wire_id),
            ] => {
                let (left_provider, right_provider) = (
                    Provider::Wire(left_wire_id.clone()),
                    Provider::Wire(right_wire_id.clone()),
                );
                let gate = Gate::Lshift(left_provider, right_provider);
                let provider = Provider::Gate(Box::new(gate));
                let resultant_wire = Wire::new(resultant_wire_id.clone(), provider);
                circuit.insert(resultant_wire_id.clone(), resultant_wire);
            } // LSHIFT
            [
                Word::Value(left_value),
                Word::Lshift,
                Word::Id(right_wire_id),
                Word::Pipe,
                Word::Id(resultant_wire_id),
            ] => {
                let (left_provider, right_provider) = (
                    Provider::Value(*left_value),
                    Provider::Wire(right_wire_id.clone()),
                );
                let gate = Gate::Lshift(left_provider, right_provider);
                let provider = Provider::Gate(Box::new(gate));
                let resultant_wire = Wire::new(resultant_wire_id.clone(), provider);
                circuit.insert(resultant_wire_id.clone(), resultant_wire);
            } // LSHIFT
            [
                Word::Value(left_value),
                Word::Lshift,
                Word::Value(right_value),
                Word::Pipe,
                Word::Id(resultant_wire_id),
            ] => {
                let (left_provider, right_provider) =
                    (Provider::Value(*left_value), Provider::Value(*right_value));
                let gate = Gate::Lshift(left_provider, right_provider);
                let provider = Provider::Gate(Box::new(gate));
                let resultant_wire = Wire::new(resultant_wire_id.clone(), provider);
                circuit.insert(resultant_wire_id.clone(), resultant_wire);
            } // LSHIFT
            [
                Word::Id(left_wire_id),
                Word::Lshift,
                Word::Value(right_value),
                Word::Pipe,
                Word::Id(resultant_wire_id),
            ] => {
                let (left_provider, right_provider) = (
                    Provider::Wire(left_wire_id.clone()),
                    Provider::Value(*right_value),
                );
                let gate = Gate::Lshift(left_provider, right_provider);
                let provider = Provider::Gate(Box::new(gate));
                let resultant_wire = Wire::new(resultant_wire_id.clone(), provider);
                circuit.insert(resultant_wire_id.clone(), resultant_wire);
            } // LSHIFT
            _ => continue,
        }
    }
    //println!("Processing finished");

    println!(
        "'a' wire value : {}",
        circuit.get("a").unwrap().get(&circuit)
    );

    Ok(())
}

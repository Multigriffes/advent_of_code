use std::{collections::HashMap, env};

use input_file_lib::get_file_content_to_string;

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

#[derive(Debug)]
enum GateKind {
    And,
    Or,
    Not,
    ProvidedNot(u16),
    Provided(u16),
    ProvidedById,
    Rshift,
    Lshift,
}

#[derive(Debug)]
struct Gate {
    kind: GateKind,
    provider: Vec<String>,
}

#[derive(Debug)]
struct Wire {
    provider: Gate,
}

impl Wire {
    fn get(&self, circuit: &HashMap<String, Wire>) -> u16 {
        self.provider.get(&circuit)
    }
}

impl Gate {
    fn get(&self, circuit: &HashMap<String, Wire>) -> u16 {
        match self.kind {
            GateKind::And => {
                circuit.get(&self.provider[0]).unwrap().get(&circuit)
                    & circuit.get(&self.provider[1]).unwrap().get(&circuit)
            }
            GateKind::Or => {
                circuit.get(&self.provider[0]).unwrap().get(&circuit)
                    | circuit.get(&self.provider[1]).unwrap().get(&circuit)
            }
            GateKind::Not => !circuit.get(&self.provider[0]).unwrap().get(&circuit),
            GateKind::ProvidedNot(value) => !value,
            GateKind::Provided(value) => value,
            GateKind::Lshift => {
                circuit.get(&self.provider[0]).unwrap().get(&circuit)
                    << circuit.get(&self.provider[1]).unwrap().get(&circuit)
            }
            GateKind::Rshift => {
                circuit.get(&self.provider[0]).unwrap().get(&circuit)
                    >> circuit.get(&self.provider[1]).unwrap().get(&circuit)
            }
            GateKind::ProvidedById => circuit.get(&self.provider[0]).unwrap().get(&circuit),
        }
    }
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
    let mut temp_cire_counter: u32 = 0;

    for line in string_to_compute.lines() {
        println!("{}", line);
        let words: Vec<Word> = line
            .split(" ")
            .map(|x| get_type_of_word(x))
            .filter(|x| !(*x == Word::Nothing))
            .collect();

        match words.len() {
            5 => {
                let Word::Id(new_wire_id) = &words[4] else {
                    continue;
                };

                let mut involved_wire_id = Vec::new();

                if let Word::Id(left_wire_id) = &words[0] {
                    involved_wire_id.push(left_wire_id.clone());
                }
                if let Word::Id(right_wire_id) = &words[2] {
                    involved_wire_id.push(right_wire_id.clone());
                }
                if let Word::Value(left_wire_value) = &words[0] {
                    involved_wire_id.push(temp_cire_counter.to_string());
                    circuit.insert(
                        temp_cire_counter.to_string(),
                        Wire {
                            provider: Gate {
                                kind: GateKind::Provided(*left_wire_value),
                                provider: Vec::new(),
                            },
                        },
                    );
                    temp_cire_counter += 1;
                }
                if let Word::Value(right_wire_value) = &words[2] {
                    involved_wire_id.push(temp_cire_counter.to_string());
                    circuit.insert(
                        temp_cire_counter.to_string(),
                        Wire {
                            provider: Gate {
                                kind: GateKind::Provided(*right_wire_value),
                                provider: Vec::new(),
                            },
                        },
                    );
                    temp_cire_counter += 1;
                }

                match &words[1] {
                    Word::And => {
                        circuit.insert(
                            new_wire_id.clone(),
                            Wire {
                                provider: Gate {
                                    kind: GateKind::And,
                                    provider: vec![
                                        involved_wire_id[0].clone(),
                                        involved_wire_id[1].clone(),
                                    ],
                                },
                            },
                        );
                    }
                    Word::Or => {
                        circuit.insert(
                            new_wire_id.clone(),
                            Wire {
                                provider: Gate {
                                    kind: GateKind::Or,
                                    provider: vec![
                                        involved_wire_id[0].clone(),
                                        involved_wire_id[1].clone(),
                                    ],
                                },
                            },
                        );
                    }
                    Word::Rshift => {
                        circuit.insert(
                            new_wire_id.clone(),
                            Wire {
                                provider: Gate {
                                    kind: GateKind::Rshift,
                                    provider: vec![
                                        involved_wire_id.pop().unwrap(),
                                        involved_wire_id.pop().unwrap(),
                                    ],
                                },
                            },
                        );
                    }
                    Word::Lshift => {
                        circuit.insert(
                            new_wire_id.clone(),
                            Wire {
                                provider: Gate {
                                    kind: GateKind::Lshift,
                                    provider: vec![
                                        involved_wire_id.pop().unwrap(),
                                        involved_wire_id.pop().unwrap(),
                                    ],
                                },
                            },
                        );
                    }
                    _ => {
                        continue;
                    }
                }
            }
            4 => {
                let Word::Id(new_wire_id) = &words[3] else {
                    continue;
                };
                match &words[1] {
                    Word::Value(value) => {
                        circuit.insert(
                            new_wire_id.clone(),
                            Wire {
                                provider: Gate {
                                    kind: GateKind::ProvidedNot(*value),
                                    provider: Vec::new(),
                                },
                            },
                        );
                    }
                    Word::Id(id) => {
                        circuit.insert(
                            new_wire_id.clone(),
                            Wire {
                                provider: Gate {
                                    kind: GateKind::Not,
                                    provider: vec![id.clone()],
                                },
                            },
                        );
                    }
                    _ => continue,
                }
            }
            3 => {
                let Word::Id(new_wire_id) = &words[2] else {
                    continue;
                };
                match &words[0] {
                    Word::Value(value) => {
                        circuit.insert(
                            new_wire_id.clone(),
                            Wire {
                                provider: Gate {
                                    kind: GateKind::Provided(*value),
                                    provider: Vec::new(),
                                },
                            },
                        );
                    }
                    Word::Id(id) => {
                        circuit.insert(
                            new_wire_id.clone(),
                            Wire {
                                provider: Gate {
                                    kind: GateKind::ProvidedById,
                                    provider: vec![id.clone()],
                                },
                            },
                        );
                    }
                    _ => continue,
                }
            }
            _ => continue,
        }
    }
    println!("Finished processing");
    println!("{:?}", circuit);

    println!("{}", circuit.get(&"a".to_string()).unwrap().get(&circuit));

    Ok(())
}

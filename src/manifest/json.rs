use std::collections::HashMap;

use eyre::{bail, Context};
use serde_json::{Map, Value};

use crate::manifest::{ArrayType, EntryPoint, Manifest, Type, ValueType};

pub fn load(manifest_file_content: &str) -> eyre::Result<Manifest> {
    let json: serde_json::Value =
        serde_json::from_str(&manifest_file_content).wrap_err("Failed to parse manifest JSON.")?;

    let mut types: HashMap<String, Type> = HashMap::new();

    types.extend(
        ValueType::variants()
            .iter()
            .map(|typ| (typ.name().to_string(), Type::Value(*typ))),
    );

    for (name, typ) in json["types"].as_object().unwrap() {
        let typ = typ.as_object().unwrap();
        let kind = typ["kind"].as_str().unwrap();

        match kind {
            "array" => types.insert(name.clone(), Type::Array(load_array(typ))),
            _ => bail!("Types of kind {kind} are not supported."),
        };
    }

    let mut entry_points: Vec<EntryPoint> = Vec::new();
    for (name, obj) in json["entry_points"].as_object().unwrap() {
        entry_points.push(load_entry_point(name, obj.as_object().unwrap(), &types));
    }

    let types = types.into_values().collect();

    Ok(Manifest {
        entry_points,
        types,
    })
}

fn load_array(obj: &Map<String, Value>) -> ArrayType {
    let elements = ValueType::from_manifest(obj["elemtype"].as_str().unwrap()).unwrap();
    let rank = obj["rank"].as_i64().unwrap() as usize;

    ArrayType {
        elements_type: elements,
        rank,
    }
}

fn load_entry_point(
    name: &str,
    obj: &Map<String, Value>,
    types: &HashMap<String, Type>,
) -> EntryPoint {
    let name = name.to_string();

    let inputs = obj["inputs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|input| input["type"].as_str().unwrap())
        .map(|input_type| types[input_type].clone())
        .collect::<Vec<_>>();

    let outputs = obj["outputs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|input| input["type"].as_str().unwrap())
        .map(|input_type| types[input_type].clone())
        .collect::<Vec<_>>();

    EntryPoint {
        name,
        inputs,
        outputs,
    }
}

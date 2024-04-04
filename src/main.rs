use serde_json::Value;
use std::fs;

fn main() {
    let file_path = "hello.json";

    let json_data = fs::read_to_string(file_path)
        .expect(&format!("Unable to read file {}", file_path));

    let data: Value = serde_json::from_str(&json_data)
        .expect("Unable to parse JSON");
    let x = "AUTO"; // For sake of simplicity
    let mut ts_code = String::new();

    if let Some(options_map) = data["input"]["options"].as_object() {
        ts_code.push_str(&format!("this.{}_options = {{\n", 
            data["name"].as_str().unwrap_or_default().to_lowercase()));

        for (option_name, option_value) in options_map {
            if let Some(type_name_str) = option_value["typeName"].as_str() {
                let allowed_value = option_value["allowedValues"]
                    .as_array()
                    .and_then(|arr| arr.iter().find(|v| v.as_str() == Some(x)))
                    .and_then(|v| v.as_str())
                    .unwrap_or(x);

                ts_code.push_str(&format!("  {}: {}.{};\n", 
                    option_name, type_name_str, allowed_value));
            }
        }
        ts_code.push_str("};\n");
    }

    if let Some(initialize) = data["methods"]["initialize"].as_object() {
        let return_type = initialize["returnType"].as_str().unwrap_or_default();
        let name = initialize["name"].as_str().unwrap_or_default();

        ts_code.push_str(&format!("this.{}_ = (() => {{ return {}.{}(\n", 
            return_type.to_lowercase(), return_type, name));

        if let Some(args_array) = initialize["args"].as_array() {
            for (index, arg) in args_array.iter().enumerate() {
                let arg_name = arg["name"].as_str().unwrap_or_default();

                ts_code.push_str(&format!("  this.{}", arg_name));
                if index < args_array.len() - 1 {
                    ts_code.push_str(",\n");
                }
            }
            ts_code.push_str("\n});\n");
        }
    }

    println!("{}", ts_code);
}

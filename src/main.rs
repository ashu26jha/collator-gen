use serde_json::Value;

fn main() {
    let json_data = r#"
    {
        "name": "ICU4XCollator",
        "input": {
            "options": {
                "type": "object",
                "strength": {
                    "typeName": "ICU4XCollatorStrength",
                    "type": "enum",
                    "allowedValues": [
                        "AUTO"
                    ]
                },
                "alternate_handling": {
                    "typeName": "ICU4XCollatorAlternateHandling",
                    "type": "enum",
                    "allowedValues": [
                        "AUTO"
                    ]
                }
            },
            "str1": {
                "type": "string"
            },
            "str2": {
                "type": "string"
            }
        },
        "methods": {
            "initialize": {
                "name": "create_v1",
                "args": [
                    {
                        "name": "dataProvider"
                    },
                    {
                        "name": "locale"
                    },
                    {
                        "name": "options"
                    }
                ],
                "returnType": "ICU4XCollator"
            }
        }
    }
    "#;

    let data: Value = serde_json::from_str(json_data).unwrap();
    let x = "Auto"; // We have Auto enum for the demo purposes

    let mut ts_code = String::new();

    
    
    let options = &data["input"]["options"];

    if let Value::Object(options_map) = options {

        ts_code.push_str(&format!(
            "this.#{}options = {{ \n",
            data["name"].as_str().unwrap_or_default()
        ));

        for (option_name, option_value) in options_map.iter() {
            if let Some(type_name_str) = option_value["typeName"].as_str() {
                let allowed_value = option_value["allowedValues"]
                    .as_array()
                    .and_then(|arr| arr.iter().find(|v| v.as_str() == Some(x)))
                    .and_then(|v| v.as_str())
                    .unwrap_or_else(|| x); 

                ts_code.push_str(&format!(" \t {}: {}.{};\n", option_name, type_name_str, allowed_value));
            }
        }
        ts_code.push_str("};\n");
    }

    let initialize = &data["methods"]["initialize"];
    let return_type = initialize["returnType"].as_str().unwrap_or_default();
    let name = initialize["name"].as_str().unwrap_or_default();
    ts_code.push_str(&format!("this.#{} = result (() => {}.{} ( \n", return_type, return_type, name));

    if let Value::Array(args_array) = &initialize["args"] {
        for (index, arg) in args_array.iter().enumerate() {
            let arg_name = arg["name"].as_str().unwrap_or_default();
    
            if arg_name == "locale" {
                ts_code.push_str(&format!("\tunwrap(this.#{})", arg_name)); 
            } else {
                ts_code.push_str(&format!("\tthis.#{}", arg_name)); 
            }
    
            if index < args_array.len() - 1 { 
                ts_code.push_str(",\n");
            }
        }
        ts_code.push_str("\n));\n"); 
    }
    println!("{}", ts_code);
}

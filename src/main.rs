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
                },
            },
            "methods": {
                "compare": {
                    "description": "Compares two strings according to the collator's rules",
                    "args": [
                        {
                            "name": "str1",
                        },
                        {
                            "name": "str2",
                        },
                        {
                            "name" "options",
                        }
                    ],
                    "returnType": "number"
                }
            }
        }
    "#;

    let data: Value = serde_json::from_str(json_data).unwrap();
    let x = "Auto"; // Replace with your actual variable

    let mut ts_code = String::new();
    let options = &data["input"]["options"];

    if let Value::Object(options_map) = options {

        ts_code.push_str(&format!(
            "this.#{}options = {{ \n",
            data["name"].as_str().unwrap_or_default()
        ));

        for (option_name, option_value) in options_map.iter() {
            if let Some(type_name_str) = option_value["typeName"].as_str() {
                // Find the value of 'x' in allowedValues:
                let allowed_value = option_value["allowedValues"]
                    .as_array()
                    .and_then(|arr| arr.iter().find(|v| v.as_str() == Some(x)))
                    .and_then(|v| v.as_str())
                    .unwrap_or_else(|| x); // Fallback to the value of 'x'

                ts_code.push_str(&format!(" \t {}: {}.{};\n", option_name, type_name_str, allowed_value));
            }
        }
        ts_code.push_str("};\n");
    }

    println!("{}", ts_code);
}

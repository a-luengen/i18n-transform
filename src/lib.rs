pub mod flattener {
    use serde_json::Value;
    use std::collections::HashMap;

    fn transform_to_flat_map_with_prefix(
        serde_res: &Value,
        prefix: &String,
    ) -> HashMap<String, String> {
        let mut flattened_map: HashMap<String, String> = HashMap::new();
        if serde_res.is_object() {
            let map = serde_res.as_object().unwrap();
            for key in map.keys() {
                match map.get(key) {
                    Some(value) => {
                        let new_prefix = match prefix.len() {
                            0 => key.to_string(),
                            _ => prefix.to_string() + "." + key,
                        };
                        if value.is_string() {
                            flattened_map.insert(new_prefix, value.as_str().unwrap().to_owned());
                        } else if value.is_object() {
                            let nested_map = transform_to_flat_map_with_prefix(value, &new_prefix);
                            flattened_map.extend(nested_map);
                        } else {
                            println!("Should handle this value differently");
                        }
                    }
                    None => println!("Couldn't find entry for {}", key),
                }
            }
        } else {
            println!("Input was not an object");
            println!("{:?}", serde_res);
        }
        return flattened_map;
    }
    pub fn transform_to_flat_map(serde_res: &Value) -> HashMap<String, String> {
        return transform_to_flat_map_with_prefix(serde_res, &String::from(""));
    }

    pub fn to_json_structure(flattened_map: &HashMap<String, String>) -> Value {
        let mut res_map = serde_json::Map::new();

        for key in flattened_map.keys() {
            let value_to_insert = Value::String(flattened_map.get(key).unwrap().to_string());

            if key.contains(".") {
                // split up key in subparts
                let mut test = key.split(".").peekable();
                let mut temp_res_map: &mut serde_json::Map<String, Value> = &mut res_map;
                let mut last_key: &str = "<fucked>";

                // iterate through parts
                while let Some(part) = test.next() {
                    if !test.peek().is_some() {
                        // add string to last object
                        last_key = part;
                    } else {
                        // is there already an entry in the flattened_map
                        // for the part?
                        if !temp_res_map.contains_key(part) {
                            temp_res_map
                                .insert(String::from(part), Value::Object(serde_json::Map::new()));
                        }
                        // go one level deeper into nested structure
                        println!("Looking for key: {}", part);
                        println!("Map: {:?}", temp_res_map);
                        temp_res_map = temp_res_map.get_mut(part).unwrap().as_object_mut().unwrap();
                        println!("New Map Reference: {:?}", temp_res_map);
                    }
                }
                temp_res_map.insert(String::from(last_key), value_to_insert);
            } else {
                res_map.insert(
                    key.to_string(),
                    Value::String(flattened_map.get(key).unwrap().to_string()),
                );
            }
        }
        /*
            println! 'Ich liebe dich!';
            string int(1)==true;
            else {int(1)==false
            boolean, dogger
            ; internetzeug, 100010100010111101001010101011011
            WELTHERRSCHAFT !!!! Hack hack hack hack hack hack hack hack hack
            if skipper == hungry
            then skipper == go eat
            else skipper == go sleep;
            unit tests 1, 2, 3.... run go erfolg
        */

        Value::Object(res_map)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn flattens_simple_object() {
            let test_input: &str = r#"{
                "text1": "Hello world"
            }"#;
            let result = transform_to_flat_map(&serde_json::from_str(test_input).unwrap());
            assert_eq!(result.keys().len(), 1);
            assert_eq!(result.contains_key("text1"), true);
            assert_eq!(result.get("text1").unwrap(), "Hello world");
        }
        #[test]
        fn flattens_multiple_keys_in_object() {
            let test_input: &str = r#"{
                "text1": "Hello world",
                "text2": "Hello world2" 
            }"#;
            let result = transform_to_flat_map(&serde_json::from_str(test_input).unwrap());
            assert_eq!(result.keys().len(), 2);
            assert_eq!(result.contains_key("text1"), true);
            assert_eq!(result.get("text1").unwrap(), "Hello world");
            assert_eq!(result.contains_key("text2"), true);
            assert_eq!(result.get("text2").unwrap(), "Hello world2");
        }
        #[test]
        fn flattens_nested_objects() {
            let test_input: &str = r#"{
                "top": {
                    "text2": "Hello world",
                    "text3": "Hello world2"
                },
                "top2": {
                    "text1": "Hello world",
                    "nested1": {
                        "text2": "Hello world",
                        "text3": "Hello world2"
                    }
                }
            }"#;
            let result = transform_to_flat_map(&serde_json::from_str(test_input).unwrap());
            assert_eq!(result.keys().len(), 5);
            assert_eq!(result.contains_key("top.text2"), true);
            assert_eq!(result.contains_key("top.text3"), true);
            assert_eq!(result.get("top.text2"), Some(&String::from("Hello world")));
            assert_eq!(result.get("top.text3").unwrap(), "Hello world2");
            assert_eq!(result.contains_key("top2.text1"), true);
            assert_eq!(result.get("top2.text1").unwrap(), "Hello world");
            assert_eq!(result.contains_key("top2.nested1.text2"), true);
            assert_eq!(result.contains_key("top2.nested1.text3"), true);
            assert_eq!(result.get("top2.nested1.text2").unwrap(), "Hello world");
            assert_eq!(result.get("top2.nested1.text3").unwrap(), "Hello world2");
        }

        #[test]
        fn serde_json_from_simple_flattened() {
            let mut test_flattened = HashMap::new();

            test_flattened.insert(String::from("top"), String::from("Hello world"));

            let result = to_json_structure(&test_flattened);

            assert_eq!(result.is_object(), true);
            let res_obj = result.as_object().unwrap();
            assert_eq!(res_obj.keys().len(), 1);
            assert_eq!(res_obj.contains_key("top"), true);
            assert_eq!(
                res_obj.get("top"),
                Some(&Value::String(String::from("Hello world")))
            );
        }

        #[test]
        fn serde_json_from_simple_flattened_with_multiple() {
            let mut test_flattened = HashMap::new();
            test_flattened.insert(String::from("top1"), String::from("Hello world"));
            test_flattened.insert(String::from("top2"), String::from("Hello world2"));

            let result = to_json_structure(&test_flattened);

            assert_eq!(result.is_object(), true);
            let res_obj = result.as_object().unwrap();

            assert_eq!(res_obj.keys().len(), 2);

            for key in test_flattened.keys() {
                assert_eq!(res_obj.contains_key(key), true);
            }
            assert_eq!(
                res_obj.get("top1"),
                Some(&Value::String("Hello world".to_owned()))
            );
            assert_eq!(
                res_obj.get("top2"),
                Some(&Value::String(String::from("Hello world2")))
            );
        }
        #[test]
        fn serde_json_from_nested_flattened() {
            let mut test_flattened = HashMap::new();
            test_flattened.insert(String::from("top.nested"), String::from("Hello world"));

            let result = to_json_structure(&test_flattened);
            assert!(result.is_object());

            let res_obj_top = result.as_object().unwrap();
            assert!(res_obj_top.contains_key("top"));
            println!("LAST TEST: {:?}", res_obj_top);
            assert!(res_obj_top.get("top").unwrap().is_object());

            let res_obj_nested = res_obj_top.get("top").unwrap().as_object().unwrap();
            assert!(res_obj_nested.contains_key("nested"));
            assert!(res_obj_nested.get("nested").unwrap().is_string())
        }

        #[test]
        fn serde_json_from_nested_flattened_with_multiple() {
            let mut test_flattened = HashMap::new();
            test_flattened.insert(String::from("top1.nested1"), String::from("Hello World"));
            test_flattened.insert(String::from("top1.nested2"), String::from("Hello World"));
            test_flattened.insert(String::from("top2.nested1"), String::from("Hello World"));
            test_flattened.insert(
                String::from("top1.nested3.deeper1"),
                String::from("Level 2"),
            );

            let result = to_json_structure(&test_flattened);
            assert!(result.is_object());

            // assert top level
            let top_level_obj = result.as_object().unwrap();
            assert_eq!(top_level_obj.keys().len(), 2);

            assert_eq!(
                serde_json::to_string(&result).unwrap(),
                "{\"top1\":{\"nested1\":\"Hello World\",\"nested2\":\"Hello World\",\"nested3\":{\"deeper1\":\"Level 2\"}},\"top2\":{\"nested1\":\"Hello World\"}}"
            );
        }
    }
}

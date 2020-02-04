// Telegram API's described in json
// The root object containe two array of objects: methods and objects
// Each methods and objects contain name, category and description. 
//
// Object description field value maybe: str, int, object name.
// 

#[macro_use]
extern crate quote;

use quote::__rt::TokenStream;

use std::io::BufReader;
use std::fs::File;
use serde_json::{self, Value, Map};
use std::fmt::Write;
use std::process::Command;

fn force_get<'a>(v: &'a Value, key: &str) -> &'a Value {
    v.get(key).expect(format!("field \"{}\" missed", key).as_str())
}

fn force_get_str<'a>(v: &'a Value, key: &str) -> &'a str {
    force_get(v, key).as_str().expect("value is't str")
}

fn force_get_float<'a>(v: &'a Value, key: &str) -> f64 {
    force_get(v, key).as_f64().expect("value is't float")
}

fn force_get_obj<'a>(v: &'a Value, key: &str) -> &'a Map<String, Value> {
    force_get(v, key).as_object().expect("value is't obj")
}

fn generait_type(v: &Value) -> TokenStream {
    let s = v.as_str().expect("value is't str");
    if s == "str" {
        return quote!(String);
    }
    if s == "int" {
        return quote!(u32);
    }
    let fs = format_ident!("{}", s);
    quote!(#fs)
}

fn generate_fields(v: &Value) -> Vec<TokenStream> {
    let mut ret = Vec::new();

    for kv in force_get_obj(v, "description") {
        let ident = format_ident!("{}", kv.0.clone());
        let ftype = generait_type(kv.1);
        ret.push(quote!(#ident: #ftype));
    }
    ret
}

fn generate_object(val: &Value) -> String {
    if !val.is_object() {
        panic!("for object generaited required Json object");
    }
    let struct_name = format_ident!("{}", force_get_str(val, "name"));
    let fields = generate_fields(val);
    let struct_def = quote! {
        struct #struct_name {
            #(#fields),*          
        }
    };

    let mut s = String::new();
    write!(&mut s, "{}", struct_def);
    s
}

fn main() {
    use std::io::Write;

    let input_file = File::open("src/requests/spec.json").unwrap();
    let mut reader = BufReader::new(input_file);
    let spec : Value = serde_json::from_reader(reader).unwrap();
    
    const DESCLAMER: &str = "// This file is generated automatically. Do not edit it";
    let mut output_file = File::create("src/requests/bot_types.rs").unwrap();
   
    writeln!(output_file, "{}", DESCLAMER);
    write!(output_file, "{}", "\n");

    let version = force_get_float(&spec, "version");
    let api_version = quote! {
        const API_VERSION: f64 = #version; 
    };
    writeln!(output_file, "{}", api_version);

    for obj in spec["objects"].as_array().unwrap() {
        write!(output_file, "{}", "\n\n");
        writeln!(output_file, "{}", generate_object(obj));
    }
    Command::new("rustfmt").arg("src/requests/bot_types.rs").output().expect("can't format file");
}
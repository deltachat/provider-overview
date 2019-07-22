#[macro_use]
extern crate lazy_static;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

extern crate glob;
use glob::glob;
use std::io::Read;
extern crate regex;
use regex::Regex;
extern crate yaml_rust;
use yaml_rust::YamlLoader;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("data.rs");
    let mut f = File::create(&dest_path).unwrap();
    println!("path: {:?}", dest_path);
    let (provider_count, provider_data, domain_count, domain_data) = gather_data();
    f.write_fmt(format_args!(
        "static DATABASE:[Provider;{}] = [{}];\n static DOMAIN_DB:[DomainDBEntry;{}] = [{}];",
        provider_count, provider_data, domain_count, domain_data
    ))
    .unwrap();
    println!("done");
}

fn gather_data() -> (u32, String, u32, String) {
    println!("gather data");
    let mut provider_data = Vec::new();
    let mut provider_count: u32 = 0;
    let mut domain_data = Vec::new();
    let mut domain_count: u32 = 0;

    for e in glob("./_providers/*.md").expect("Failed to read glob pattern") {
        let pathbuf = e.unwrap();
        let path = pathbuf.as_path();
        //println!("{}", path.display());
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        //println!("{}", contents);
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?ims)^---\n(.+)\n---(.*)").unwrap();
        }
        let cap = RE.captures(&contents).unwrap();
        let yaml_part = &cap[1];
        let md_part = &cap[2];
        //println!("{} -> {}", yaml_part, md_part);
        let yaml = &YamlLoader::load_from_str(yaml_part).unwrap()[0];

        let p_name = yaml["name"].as_str().unwrap();
        println!("{}", p_name);
        let p_domains = parse_yml_string_array(yaml["domains"].clone());
        let p_status_state = yaml["status"]["state"].as_str().unwrap(); //TODO convert to ENUM
        let p_status_date = yaml["status"]["date"].as_str().unwrap();
        //println!("{} on {}; {:?}", p_status_state, p_status_date, p_domains);

        provider_data.push(format!(
            r###"Provider {{
        name: "{}",
        status: Status {{ state: {}, date: "{}" }},
        markdown: r##"{}"## }}"###,
            p_name,
            status_state_source(p_status_state),
            p_status_date,
            md_part
        ));
        provider_data.push(",".to_string());

        for domain in &p_domains {
            // remove (€) from domains
            // and only let domains through (contains a dot, no spaces in between and no parentrethese)
            domain_data.push(format!(
                "DomainDBEntry {{ domain: \"{}\", list_index: {} }}",
                domain, provider_count
            ));

            domain_data.push(",\n".to_string());
            domain_count = domain_count + 1;
        }

        provider_count = provider_count + 1;
    }

    //remove last commas
    provider_data.pop();
    domain_data.pop();

    let provider_string: String = provider_data.into_iter().collect();
    let domain_string: String = domain_data.into_iter().collect();
    return (provider_count, provider_string, domain_count, domain_string);
}

fn parse_yml_string_array(array: yaml_rust::yaml::Yaml) -> Vec<String> {
    //? could be one string or an array of strings -> eitherway please convert to vector?
    if !array.is_array() {
        return vec![array.as_str().unwrap().to_string()];
    } else {
        let a: Vec<String> = array
            .into_vec()
            .unwrap()
            .into_iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect::<Vec<String>>();
        return a;
    }
}

fn status_state_source(state: &str) -> String {
    let status = match state {
        "OK" => "OK",
        "PREP" => "PREPARATION",
        "BROKEN" => "BROKEN",
        _ => "UNKNOWN", // When you get an error regarding StatusState::UNKNOWN you have a problem with your data file
    };
    return format!("StatusState::{}", status);
}

/*
idea:
- [ ] Error on missing yml/invalid value? / ci test to run on pull requests?
*/

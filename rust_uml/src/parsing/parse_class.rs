use regex::Regex;
use std::option;

pub struct Klasse {
    pub _name: String,
    pub _property: String,
    pub _keywords: String,
    pub _attribute: Vec<Attribut>,
    pub _methoden: Vec<Methode>
}
pub struct Attribut {
    pub _modifikator: char,
    pub _final: bool,
    pub _static: bool,
    pub _name: String,
    pub _datentyp: String,
    pub _wert: String
}
pub struct Methode {
    pub _modifikator: char,
    pub _final: bool,
    pub _static: bool,
    pub _name: String,
    pub _returntyp: String,
    pub _parameter: Vec<String>
}
pub struct Beziehung {
    pub _beziehungstyp: Beziehungstyp,
    pub _von_klasse_name: String,
    pub _von_klasse_pfeil: bool,
    pub _von_klasse_mult: String,
    pub _zu_klasse_name: String,
    pub _zu_klasse_pfeil: bool,
    pub _zu_klasse_mult: String
}
pub enum Beziehungstyp {
    EXTENDS,
    IMPLEMENTS,
    ASSOCIATION,
    AGGREGATION,
    COMPOSITION,
    DEPENDENCY
}

fn build_klasse(_name: String, _property: String, _keywords:String) -> Klasse {
    Klasse {
        _name,
        _property,
        _keywords,
        _attribute: Vec::new(),
        _methoden: Vec::new()
    }
}
fn build_attribut(_modifikator: char, _final: bool, _static: bool, _name: String, _datentyp: String, _wert: String) -> Attribut {
    Attribut {
        _modifikator,
        _final,
        _static,
        _name,
        _datentyp,
        _wert
    }
}
fn build_methode(_modifikator: char, _final: bool, _static: bool, _name: String, _returntyp: String) -> Methode {
    Methode {
        _modifikator,
        _final,
        _static,
        _name,
        _returntyp,
        _parameter: Vec::new()
    }
}









pub fn parse(string_vec: &mut Vec<&'static str>, klassen: &mut Vec<Klasse>, beziehungen : &mut Vec<Beziehung>) {


    let mut class : Klasse = build_klasse(String::from(""), String::from(""), String::from(""));

    for s in string_vec {
        if parse_classname(s) != "" {
            if class._name != "" {
                klassen.push(class);
            }
            class = build_klasse(parse_classname(s), String::from(""), String::from(""));
        }
        if parse_property(s) != "" {
            class._property = parse_property(s);
        }
        if parse_keywords(s) != "" {
            class._keywords = parse_keywords(s);
        }
        match parse_attribute(s) {
            Some(x) => {class._attribute.push(x)}
            None    => {}
        }
        match parse_method(s) {
            Some(x) => {class._methoden.push(x)}
            None    => {}
        }

    }

    klassen.push(class);

}

fn parse_classname(s: &str) -> String {
    let re = Regex::new("^classname\"([^\"]+?)\"").unwrap();
    let mut name = String::from("");
    if re.is_match(s) {
        let caps = re.captures(s).unwrap();
        name = caps.get(1).map_or(String::from(""), |m| String::from(m.as_str()));
    }
    name
}
fn parse_property(s: &str) -> String {
    let re = Regex::new("^property\"([^\"]+?)\"").unwrap();
    let mut property = String::from("");
    if re.is_match(s) {
        let caps = re.captures(s).unwrap();
        let u = caps.get(1).map_or(String::from(""), |m| String::from(m.as_str()));
        if u.contains(","){
            let v: Vec<&str> = u.split(',').collect();
            for t in v {
                property.push_str(t);
                property.push_str(" ");
            }
            property = String::from(property.trim());
        } else {
            property = u;
        }
    }
    property
}
fn parse_keywords(s: &str) -> String {
    let re = Regex::new("^keywords\"([^\"]+?)\"").unwrap();
    let mut keywords = String::from("");
    if re.is_match(s) {
        let caps = re.captures(s).unwrap();
        let u = caps.get(1).map_or(String::from(""), |m| String::from(m.as_str()));
        if u.contains(","){
            let v: Vec<&str> = u.split(',').collect();
            for t in v {
                keywords.push_str(t);
                keywords.push_str(" ");
            }
            keywords = String::from(keywords.trim());
        } else {
            keywords = u;
        }
    }
    keywords
}
fn parse_attribute(s: &str) -> Option<Attribut> {
    let (mut _modifikator, mut _final, mut _static, mut _name, mut _datentyp, mut _wert) : (char, bool, bool, String, String, String) = (' ', false, false, String::from(""), String::from(""), String::from(""));

    let re = Regex::new("^attribute\"([^\"]+?)\"").unwrap();
    if re.is_match(s) {
        let caps = re.captures(s).unwrap();
        let u = caps.get(1).map_or(String::from(""), |m| String::from(m.as_str()));
        let v: Vec<&str> = u.split(',').collect();
        for (i, item) in v.iter().enumerate() {
            if i == 0 {
                _modifikator = item.chars().next().unwrap();
            } else if item.to_string() == "static" {
                _static = true;
            } else if item.to_string() == "final" {
                _final = true;
            } else if _name == "" {
                _name = item.to_string();
            } else if _datentyp == "" {
                _datentyp = item.to_string();
            } else if _wert == "" {
                _wert = item.to_string();
            } else {
                println!("Zu viele Eingaben! Hier stimmt was nicht!");
            }
        }
        return Some(build_attribut(_modifikator, _final, _static, _name, _datentyp, _wert));
    }
    None
}
fn parse_method(s: &str) -> Option<Methode> {
    let re = Regex::new("^method\"([^\"]+?)\"").unwrap();
    if re.is_match(s) {
        let mut method : Methode = build_methode(' ', false, false, String::from(""), String::from(""));
        let caps = re.captures(s).unwrap();
        let u = caps.get(1).map_or(String::from(""), |m| String::from(m.as_str()));
        let v: Vec<&str> = u.split(',').collect();
        for (i, item) in v.iter().enumerate() {
            if i == 0 {
                method._modifikator = item.chars().next().unwrap();
            } else if item.to_string() == "static" {
                method._static = true;
            } else if item.to_string() == "final" {
                method._final = true;
            } else if method._name == "" {
                method._name = item.to_string();
            } else if method._returntyp == "" {
                method._returntyp = item.to_string();
            } else {
                method._parameter.push(item.to_string());
            }
        }
        return Some(method);
        
    }
    None
}


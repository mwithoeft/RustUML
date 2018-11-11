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
    pub _wert: String,
    pub _parameter: Vec<Methodenparameter>
}
pub struct Methodenparameter {
    pub _name: String,
    pub _datentyp: String
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
fn build_methode(_modifikator: char, _final: bool, _static: bool, _name: String, _returntyp: String, _wert: String) -> Methode {
    Methode {
        _modifikator,
        _final,
        _static,
        _name,
        _returntyp,
        _wert,
        _parameter: Vec::new()
    }
}
fn build_methodenparamter(_name: String, _datentyp: String) -> Methodenparameter {
    Methodenparameter {
        _name,
        _datentyp
    }
}









pub fn parse(string_vec: &mut Vec<&'static str>, klassen: &mut Vec<Klasse>, beziehungen : &mut Vec<Beziehung>) {


    let mut class : Klasse = build_klasse(String::from(""), String::from(""), String::from(""));

    for s in string_vec {
        if parse_classname(s) != "" {
            //Hier muss noch getestet werden ob bisher bestehende Klasse regulär ist und gepusht werden kann
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

    }


    class._attribute.push(build_attribut('a', false, false, String::from("Name"), String::from("Property"), String::from("Name")));

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
    let (_modifikator, _final, _static, _name, _datentyp, _wert) : (char, bool, bool, String, String, String);

    let re = Regex::new("^attribute\"([^\"]+?)\"").unwrap();
    if re.is_match(s) {
        let caps = re.captures(s).unwrap();
        let u = caps.get(1).map_or(String::from(""), |m| String::from(m.as_str()));
        let v: Vec<&str> = u.split(',').collect();
        for (i, item) in v.iter().enumerate() {
            println!("{}", i);
        }
    }

    None
}


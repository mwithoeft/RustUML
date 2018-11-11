use regex::Regex;

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
    pub _vonKlasse: String,
    pub _vonKlassePfeil: bool,
    pub _vonKlasseMult: String,
    pub _zuKlasse: String,
    pub _zuKlassePfeil: bool,
    pub _zuKlasseMult: String
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

    let (mut name, mut property, mut keywords) : (String, String, String);

    let mut class = build_klasse(String::from("Name"), String::from("Property"), String::from("Keywords"));

    for s in string_vec {
        //Neue Klasse
        let re = Regex::new("^classname\"([^\"]+?)\"").unwrap();
        if re.is_match(s) {
            let caps = re.captures(s).unwrap();
            name = caps.get(1).map_or(String::from(""), |m| String::from(m.as_str()));
            println!("{}", name);
        }
    }

    class._attribute.push(build_attribut('a', false, false, String::from("Name"), String::from("Property"), String::from("Name")));

    klassen.push(class);

}


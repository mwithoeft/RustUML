pub struct Klasse {
    _name: String,
    _property: String,
    _keywords: String,
    _attribute: Vec<Attribut>,
    _methoden: Vec<Methode>
}
struct Attribut {
    _modifikator: char,
    _final: bool,
    _static: bool,
    _name: String,
    _datentyp: String,
    _wert: String
}
struct Methode {
    _modifikator: char,
    _final: bool,
    _static: bool,
    _name: String,
    _returntyp: String,
    _wert: String,
    _methoden: Vec<Methodenparameter>
}
struct Methodenparameter {
    _name: String,
    _datentyp: String
}



pub struct Beziehung {
    _beziehungstyp: Beziehungstyp,
    _klasse1: String,
    _klasse2: String,
    _klasse1mult: String,
    _klasse2_mult: String
}
enum Beziehungstyp {
    EXTENDS,
    IMPLEMENTS,
    ASSOCIATION,
    AGGREGATION,
    COMPOSITION
}
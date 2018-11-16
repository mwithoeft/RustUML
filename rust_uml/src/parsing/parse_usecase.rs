pub struct Typ {
    pub _elementtyp: TypEnum,
    pub _elementname: String,
    pub _behaelter: String
}
pub enum TypEnum {
    SUBJECT,
    ACTOR,
    USECASE,
    EXTPOINT,
    UNDEFINED
}
pub struct Beziehung {
    pub _beziehungstyp: Beziehungstyp,
    pub _von_element_name: Vec<String>,
    pub _von_element_mult: String,
    pub _zu_element_name: String,
    pub _zu_element_mult: String,
    pub _notiz: String
}
pub enum Beziehungstyp {
    ASSOCIATION,
    GENERALIZATION,
    INCLUDE,
    EXTEND,
    UNDEFINED
}
fn build_typ(_elementtyp: TypEnum, _elementname: String, _behaelter:String) -> Typ {
    Typ {
        _elementtyp,
        _elementname,
        _behaelter
    }
}
fn build_beziehung(_beziehungstyp: Beziehungstyp, _von_element_mult: String, _zu_element_name: String,_zu_element_mult: String,_notiz: String) -> Beziehung {
    Beziehung {
        _beziehungstyp,
        _von_element_name: Vec::new(),
        _von_element_mult,
        _zu_element_name,
        _zu_element_mult,
        _notiz
    }
}








pub fn parse(string_vec: &mut Vec<&'static str>, mut klassen: &mut Vec<Typ>, beziehungen : &mut Vec<Beziehung>) {

}






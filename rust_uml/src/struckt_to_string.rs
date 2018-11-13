use std::string::String

mod parsing;

fn attribut_to_string(Attribut : Attribut)-> String{
    let s= String::from(Attribut._modifikator);
    if (Attribut.final){
        s.push("");
    }
    

}
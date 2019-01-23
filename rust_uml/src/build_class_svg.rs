use std::string::String;
use svg::Document;
use parsing;
use svglib;


const WIDTH: i32 = 1680;
const HEIGHT: i32 = 720;
const X_MIN: i32 = 300; // mindest abstand x-Achse
const Y_MIN: i32 = 200; // mindest abstand y-Achse
const FONT_SIZE_TITLE: i32 = 20;
const FONT_SIZE: i32 = 12;
const TITEL_MIN: i32 = 30;
const MIN_RAND_LINKS: i32 = 2;
const FONT_SPACE: i32 = 12;
const ROW_LENGTH: i32 = 5;


struct Pngclass{
    id:i32,
    breite: i32,
    hoehe_kopf: i32,
    hoehe_att: i32,
    hoehe_meth: i32,
    rel_point_loru: ((i32,i32),(i32,i32),(i32,i32),(i32,i32)),
    attribute: Vec<String>,
    methoden: Vec<String>,
    name: String,
    property: String,
    keywords: String
}
fn buildclass(id: i32)->Pngclass{
    Pngclass {
        id,
        breite: 1,
        hoehe_kopf: 0,
        hoehe_att: 0,
        hoehe_meth: 0,
        rel_point_loru:((0,0),(0,0),(0,0),(0,0)),
        attribute: Vec::new(),
        methoden: Vec::new(),
        name: String::new(),
        property: String::new(),
        keywords: String::new()
    }
}
pub fn build_klassendiagramm(klassen: &mut Vec<parsing::parse_class::Klasse>, mut beziehungen : &mut Vec<parsing::parse_class::Beziehung>)->Document{
    let mut document = Document::new()
            .set("viewBox", (WIDTH-WIDTH, HEIGHT - HEIGHT , WIDTH, HEIGHT));
    let mut class: Vec<Pngclass>= Vec::new();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut id: i32= 0;
    for k in klassen {
        class.push(buildclass(id));
        berechne_werte(&k._name ,&k._property ,&k._keywords , &mut k._attribute, &mut k._methoden,  id, &mut class );
        document = draw_class(document, &mut class, id, i, j);
        i=i+1;
        id = id + 1;
        if i> 4 {
            i=0;
            j+=1;
        }
    }
    document = draw_relation(document, &mut beziehungen, class);
    
    document
}
fn breite(name: String, attribute: &Vec<String> , methoden: &Vec<String>)-> i32{
    let mut breite:i32;
    breite = name.chars().count()as i32 * FONT_SIZE_TITLE;
    for a in attribute {
        if a.chars().count() as i32 * FONT_SIZE / 2 > breite {
            breite = a.chars().count() as i32 * FONT_SIZE / 2 ;
        }
    }
    for m in methoden{
        if m.chars().count() as i32 * FONT_SIZE / 2  > breite {
            breite = m.chars().count() as i32 * FONT_SIZE / 2;
        }
    }
   
    breite as i32
}
fn hoehe(property: &str ,keywords: &str, attribute: &Vec<String> , methoden: &Vec<String>)-> (i32,i32,i32){
    //Default werte
    let mut hoehe_kopf:i32 = TITEL_MIN;
    let mut hoehe_att:i32 = 0;
    let mut hoehe_meth:i32 = 0;

    if !property.is_empty(){
        hoehe_kopf += FONT_SPACE;
    }
    if !keywords.is_empty(){
        hoehe_kopf += FONT_SPACE;
    }
    hoehe_att += FONT_SPACE;
    for _a in attribute{
        hoehe_att += FONT_SPACE;
    }
    hoehe_meth += FONT_SPACE;
    for _m in methoden{
        hoehe_meth += FONT_SPACE;
    }

    (hoehe_kopf, hoehe_att , hoehe_meth)
}
fn berechne_werte(name: &str,property: &str , keywords: &str, attribute: &mut Vec<parsing::parse_class::Attribut>, 
                methoden: &mut Vec<parsing::parse_class::Methode>, id: i32, pngclass: &mut Vec<Pngclass>){
   
    for class in pngclass{
        if class.id == id{
            class.attribute = attribut_to_string(attribute);
            class.methoden = methode_to_string(methoden);
            class.breite = breite(name.to_string(), &class.attribute, &class.methoden);
            class.hoehe_kopf = hoehe(property, keywords, &class.attribute, &class.methoden).0;
            class.hoehe_att = hoehe(property, keywords, &class.attribute, &class.methoden).1;
            class.hoehe_meth = hoehe(property, keywords, &class.attribute, &class.methoden).2;
            class.name =name.to_string();
            class.property = property.to_string();
            class.keywords = keywords.to_string();
        }
    }  
}
fn attribut_to_string(attribute: &mut Vec<parsing::parse_class::Attribut>)-> Vec<String>{
    let mut attribute_string: Vec<String> = Vec::new();
    for a in attribute {
        let mut string = String::from("");
        string.push_str(&a._name);
        if a._final {
            string = string.to_uppercase();
        }
        string.insert_str(0, &a._modifikator.to_string());
        string.push_str(":");
        string.push_str(&a._datentyp);
        if !a._wert.is_empty(){
            string.push_str(" = ");
            string.push_str(&a._wert);
        }
        attribute_string.push(string);
    }
    attribute_string
}
fn methode_to_string(methode: &mut Vec<parsing::parse_class::Methode>)-> Vec<String>{
    let mut methoden_string: Vec<String> = Vec::new();
    for a in methode {
        let mut string = String::from("");
        string.push_str(&a._name);
        if a._final {
            string = string.to_uppercase();
        }
        string.insert_str(0, &a._modifikator.to_string());
        string.push_str("(");
        if !a._parameter.is_empty(){
            for p in &a._parameter {
                string.push_str(&p.to_string());
                string.push_str(",");

            }
        }
        string.push_str("):");
        string.push_str(&a._returntyp);
        
        methoden_string.push(string);
    }
    methoden_string
}  
fn draw_class(mut document: Document, klasse: &mut Vec<Pngclass>, id:i32, i:i32, j:i32)-> Document{
    let mut x: i32 = 20+i*X_MIN;
    let y: i32 = 70+j*X_MIN;
    let mut counter: i32 = 2;
   if id!=0{
       if id % 5 == 0{
           x = 20;
       }else{
            x = (klasse[id as usize - 1].rel_point_loru.3).0 + klasse[id as usize - 1].breite;
       }
   }
        //berechnete werte für Klassen namen einsetzten und kasten zeichnen
        document = svglib::rectangle(document,x, y, klasse[id as usize].breite, klasse[id as usize].hoehe_kopf);
        //berechnete werte für Attribute namen einsetzten und kasten zeichnen
        document = svglib::rectangle(document, x, y+klasse[id as usize].hoehe_kopf as i32, klasse[id as usize].breite, klasse[id as usize].hoehe_att);
        //berechnete werte für Methoden namen einsetzten und kasten zeichnen
        document = svglib::rectangle(document,x, y+klasse[id as usize].hoehe_kopf as i32+klasse[id as usize].hoehe_att as i32,klasse[id as usize].breite, klasse[id as usize].hoehe_meth);

        klasse[id as usize].rel_point_loru= ((x  , y + (klasse[id as usize].hoehe_kopf+klasse[id as usize].hoehe_att+klasse[id as usize].hoehe_meth)/2),
                                (x  +(klasse[id as usize].breite/2) ,y ),
                                (x  +klasse[id as usize].breite ,y+(klasse[id as usize].hoehe_kopf+klasse[id as usize].hoehe_att+klasse[id as usize].hoehe_meth)/2),
                                (x  +(klasse[id as usize].breite/2) ,(y +(klasse[id as usize].hoehe_kopf+klasse[id as usize].hoehe_att+klasse[id as usize].hoehe_meth))));
        document = svglib::write(document, (x + klasse[id as usize].breite / 2 - klasse[id as usize].keywords.chars().count() as i32 * FONT_SIZE / 8, 
                                            y + FONT_SIZE) , klasse[id as usize].keywords.to_string(), FONT_SIZE);

        //Falls es eine property gibt muss die bei der Platzierung des Klassennamens berücksichtigt werden
        let mut property_space = 0;
        if !klasse[id as usize].property.is_empty(){
            property_space = FONT_SIZE ;
        }
        //Klassen namen schreiben x wert = x des linken randes + die Differenz aus der hälfte der breite und ein viertel der Wortlänge
        // y wert = y wert des oberen randes + die Differenz aus 3/4 der kopf höhe und der konstanten property_space
        document = svglib::write(document, (x + klasse[id as usize].breite / 2 - klasse[id as usize].name.chars().count() as i32 * FONT_SIZE_TITLE / 4, 
                                            y + (klasse[id as usize].hoehe_kopf / 4) * 3 - property_space), klasse[id as usize].name.to_string(), FONT_SIZE_TITLE);
        
        document = svglib::write(document, (x + klasse[id as usize].breite / 2 - klasse[id as usize].property.chars().count() as i32 * FONT_SIZE / 5 , 
                                            y + klasse[id as usize].hoehe_kopf - FONT_SIZE), klasse[id as usize].property.to_string(), FONT_SIZE);
        
        //Attribute schreiben
        for att in &klasse[id as usize].attribute {
            document = svglib::write(document, (x + MIN_RAND_LINKS, y + klasse[id as usize].hoehe_kopf + counter + FONT_SIZE), att.to_string(), FONT_SIZE);
            counter += FONT_SPACE;
        }
        counter = 2;
        for meth in &klasse[id as usize].methoden {
            document = svglib::write(document, (x  + MIN_RAND_LINKS  , y + klasse[id as usize].hoehe_kopf + klasse[id as usize].hoehe_att + counter + FONT_SIZE), meth.to_string(), FONT_SIZE);
            counter+= FONT_SPACE;
        }
    
    document
}

fn draw_relation(mut document: Document, relation: &mut Vec<parsing::parse_class::Beziehung>, class:  Vec<Pngclass> )-> Document{
    let mut from: (i32, String, (i32,i32)) = (-1,"".to_string(),(-1,-1));
    let mut to: (i32, String, (i32,i32)) = (-1,"".to_string(),(-1,-1));
    let mut temp_point_from: ((i32,i32),(i32,i32),(i32,i32),(i32,i32)) = ((1,1),(1,1),(1,1),(1,1));
    let mut temp_point_to: ((i32,i32),(i32,i32),(i32,i32),(i32,i32)) = ((1,1),(1,1),(1,1),(1,1));
    let _start_point: i32;
    let _end_point: i32;
    let mut horizontal:bool = true;
    let mut relation_finish:bool;
    let mut tausch:bool = false;
    let mut nebeneinander:bool = true;
    let mut max_height:i32 = 0;

    for r in relation{
        relation_finish = false;
        for c in &class{
            if !relation_finish{
                
                if c.hoehe_kopf + c.hoehe_att + c.hoehe_meth > max_height{
                    max_height = c.hoehe_kopf + c.hoehe_att + c.hoehe_meth;
                }
               
                // Wenn der Klassen name zum beziehungs von_klasse_namen passt
                //werden der name, die ID und die relation points zwischengespeichert
                if r._von_klasse_name == c.name{
                    //println!("Von gefunden");
                    from.0 = c.id;
                    from.1 = c.name.to_string();
                    temp_point_from = c.rel_point_loru;
                    
                }
                // Wenn der Klassen name zum beziehungs zu_klasse_namen passt
                //werden der name, die ID und die relation points zwischengespeichert
                else if r._zu_klasse_name == c.name{
                    //println!("Zu gefunden");
                    to.0 = c.id;
                    to.1 = c.name.to_string();
                    temp_point_to = c.rel_point_loru;
                    
                }
                //wenn von und zu klasse gefunden wurden
                if from.0 != -1 && to.0 != -1 {
                    //Wenn die beiden Klassen nicht nebeneinander aber in der selben Reihe sind
                    if !(from.0 + 1 == to.0 || from.0 - 1 == to.0) && same_row(from.0,to.0) {
                        nebeneinander = false;
                        horizontal = false;
                        from.2 = temp_point_from.3;
                        to.2 = temp_point_to.3;
                        if (from.2).1 < (to.2).1 {
                             max_height = max_height - ((from.2).1 - (temp_point_from.1).1)/2;
                        }else {
                              max_height = max_height - ((to.2).1 - (temp_point_to.1).1)/2;
                          }
                        document = svglib::around_the_corner_arrow(document, from.2, to.2, r._von_klasse_pfeil, r._zu_klasse_pfeil, svglib::class_enum_to_string(&r._beziehungstyp), max_height);
                        max_height = 0;
                        relation_finish = true;
                    }
                    //Wenn die From id kleiner als die to id ist und to und from in der selben reihe stehen
                    //same_row(from.0,to.0)
                    else if from.0 < to.0 && same_row(from.0,to.0) {
                        from.2 = temp_point_from.2;
                        to.2 = temp_point_to.0;
                        relation_finish = true;
                        //println!("{}",relation_finish );
                        //println!("0");
                        
                        
                    }
                    //Wenn die From id größer als die to id ist und to und from in der selben reihe stehen
                    else if from.0 > to.0 && same_row(from.0,to.0) {
                        //from und to koordinaten und ob sie Pfeile haben müssen getauscht werden, weil sonst der Pfeil falsch dargestellt wird
                        from.2 = temp_point_to.2;
                        to.2 = temp_point_from.0;
                        let tmp = r._von_klasse_pfeil;
                        r._von_klasse_pfeil = r._zu_klasse_pfeil;
                        r._zu_klasse_pfeil = tmp;
                        relation_finish = true;
                        tausch = true;
                        
                    }
                    //Wenn die From id kleiner als die to id ist und to unter from steht
                    else if from.0 < to.0 && !same_row(from.0,to.0) {
                        from.2 = temp_point_from.3;
                        to.2 = temp_point_to.1;
                        horizontal = false;
                        relation_finish = true;
                        
                    }
                    //Wenn die From id größer als die to id ist und to über from steht
                    else if from.0 > to.0 && !same_row(from.0,to.0) {
                        from.2 = temp_point_from.1;
                        to.2 = temp_point_to.3;
                        horizontal = false;
                        relation_finish = true;
                        
                    }
                    
                }
            }
        }
        
        if nebeneinander{
            match r._beziehungstyp{
                parsing::parse_class::Beziehungstyp::EXTENDS => {
                    document = svglib::extends(document, from.2, to.2, r._von_klasse_pfeil, r._zu_klasse_pfeil, horizontal);
                }
                parsing::parse_class::Beziehungstyp::IMPLEMENTS=>{ 
                    document = svglib::implements(document, from.2, to.2, r._von_klasse_pfeil, r._zu_klasse_pfeil, horizontal);
                }
                parsing::parse_class::Beziehungstyp::ASSOCIATION=>{
                    document = svglib::association(document, from.2, to.2, r._von_klasse_pfeil, r._zu_klasse_pfeil, horizontal);
                }
                parsing::parse_class::Beziehungstyp::AGGREGATION=>{
                    document = svglib::aggregation(document, from.2, to.2, r._von_klasse_pfeil, r._zu_klasse_pfeil, horizontal);
                }
                parsing::parse_class::Beziehungstyp::COMPOSITION=>{
                    document = svglib::composition(document, from.2, to.2, r._von_klasse_pfeil, r._zu_klasse_pfeil, horizontal);
                }
                parsing::parse_class::Beziehungstyp::DEPENDENCY=>{
                    document = svglib::dependency(document, from.2, to.2, r._von_klasse_pfeil, r._zu_klasse_pfeil, horizontal);
                }
                parsing::parse_class::Beziehungstyp::UNDEFINED=>{}
            }
        }
        

        
        if !(r._von_klasse_mult.is_empty()) {
            let mut p: (i32,i32) = (0,0);
                p.0 = (from.2).0 + FONT_SIZE * 2;
            if horizontal{
                p.1 = (from.2).1 - FONT_SIZE;
            }else {
                p.1 = (from.2).1 + FONT_SIZE;
            }

            document = svglib::write(document, p, r._von_klasse_mult.to_string(), FONT_SIZE);;

        }
        if !(r._zu_klasse_mult.is_empty()){
            let mut p: (i32,i32) = (0,0);
                p.0 = (to.2).0  - (FONT_SIZE * r._zu_klasse_mult.chars().count() as i32 ) ;
            if horizontal{
                p.1 = (to.2).1  - FONT_SIZE;  
            }else if !nebeneinander{
                p.1 = (to.2).1  + FONT_SIZE; 
            }else {
                p.0 = (to.2).0 + FONT_SIZE * 2;
                p.1 = (to.2).1  - FONT_SIZE;
                 
            }


            document = svglib::write(document, p, r._zu_klasse_mult.to_string(), FONT_SIZE);;
        }
        //reset
        from= (-1,"".to_string(),(-1,-1));
        to= (-1,"".to_string(),(-1,-1));
        temp_point_from = ((1,1),(1,1),(1,1),(1,1));
        temp_point_to = ((1,1),(1,1),(1,1),(1,1));
        horizontal = true;
        nebeneinander = true;
        //zurücktauschen, wenn getausch wurde
        if tausch {
            let tmp = r._von_klasse_pfeil;
            r._von_klasse_pfeil = r._zu_klasse_pfeil;
            r._zu_klasse_pfeil = tmp;
            
            tausch = false;
        }
        
        
    }
    document
}

fn same_row(id1:i32,id2:i32)->bool{
    let is_same_row:bool = false;
    let max_id = if id1 > id2 { id1 } else { id2 };  
    let idrow = (max_id-(max_id%ROW_LENGTH))/ROW_LENGTH;

    for x in 1..idrow+2{

        if (id1 < ROW_LENGTH*x && id1 >= ROW_LENGTH*(x-1)) && (id2 < ROW_LENGTH*x && id2 >= ROW_LENGTH*(x-1)){
            return true;
        }
    }

    is_same_row
}
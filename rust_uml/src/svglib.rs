use std::string::String;
use parsing;
use svg::Document;
use svg::node::element::Path;
use svg::node::element::Rectangle;
use svg::node::element::Line;
use svg::node::element::Text;
use svg::node::element::path::Data;
use svg::node::element::Ellipse;

const ARROW:i32 = 10;
const ACTOR_LINE:i32 = 30;

pub fn rectangle(mut document:Document,x:i32, y:i32, width:i32, height:i32)->Document{
    let rechteck = Rectangle::new()
                .set("x", x)                            // x-Achse
                .set("y", y)                            // y-Achse
                .set("height", height)                  //höhe
                .set("width", width)                    //breite
                .set("fill", "none")                    //füll farbe
                .set("stroke", "black")                 //Rand farbe
                .set("stroke-width", 2);                //Rand breite

                document=document.add(rechteck);

        document
}
pub fn association(mut document:Document,from:(i32,i32), to:(i32,i32), _f_arrow:bool, _t_arrow:bool, horizontal:bool)->Document{
        let mut tmp_addition0:i32 = 0;
        let mut tmp_addition1:i32 = 0;
        let tmp_from:(i32,i32);
        let tmp_to:(i32,i32);

        //Jenachdem ob die Pfeile Horizontal oder Vertikal sind muss die x oder y achse und 10 verkleinert bzw vergrößert werden
        if horizontal{
                tmp_addition0 = ARROW;
        }else {
                tmp_addition1 = ARROW;
        }
        //Berechnung der neuen x/y Koordinaten für die Pfeile
        tmp_from = (from.0 + tmp_addition0,from.1 + tmp_addition1);
        tmp_to = (to.0 - tmp_addition0, to.1 - tmp_addition1);
        //Ein Pfeil an From
        if _f_arrow && !_t_arrow{
                document = document.add(dependency_arrow(tmp_from, from, horizontal));
                document=document.add(line(tmp_from, to));
        }
        //Ein Pfeil an To
        else if _t_arrow && !_f_arrow{
                document = document.add(dependency_arrow(tmp_to, to, horizontal));
                document=document.add(line(from, tmp_to ));
        }
        //Pfeile in beiden Richtungen
        else if  _t_arrow && _f_arrow{
                document = document.add(dependency_arrow(tmp_from, from, horizontal));
                document = document.add(dependency_arrow(tmp_to, to, horizontal));
                document=document.add(line(tmp_from, tmp_to));
        }
        //keine Pfeile
        else{
                document=document.add(line(from, to));
        }

        document
}
pub fn extends(mut document:Document,from:(i32,i32), to:(i32,i32), _f_arrow:bool, _t_arrow:bool, horizontal:bool)->Document{
        let mut tmp_addition0:i32 = 0;
        let mut tmp_addition1:i32 = 0;
        let tmp_from:(i32,i32);
        let tmp_to:(i32,i32);

        //Jenachdem ob die Pfeile Horizontal oder Vertikal sind muss die x oder y achse und 10 verkleinert bzw vergrößert werden
        if horizontal{
                tmp_addition0 = ARROW;
        }else {
                tmp_addition1 = ARROW;
        }
        //Berechnung der neuen x/y Koordinaten für die Pfeile
        tmp_from = (from.0 + tmp_addition0,from.1 + tmp_addition1);
        tmp_to = (to.0 - tmp_addition0, to.1 - tmp_addition1);
        //Ein Pfeil an From
        if _f_arrow && !_t_arrow{
                document = document.add(extends_arrow(tmp_from, from, horizontal));
                document=document.add(line(tmp_from, to));
        }
        //Ein Pfeil an To
        else if _t_arrow && !_f_arrow{
                document = document.add(extends_arrow(tmp_to, to, horizontal));
                document=document.add(line(from, tmp_to ));
        }
        //Pfeile in beiden Richtungen
        else if  _t_arrow && _f_arrow{
                document = document.add(extends_arrow(tmp_from, from, horizontal));
                document = document.add(extends_arrow(tmp_to, to, horizontal));
                document=document.add(line(tmp_from, tmp_to));
        }
        //keine Pfeile
        else{
                document=document.add(line(from, to));
        }

        document
}
pub fn implements(mut document:Document, from:(i32,i32), to:(i32,i32), _f_arrow:bool, _t_arrow:bool, horizontal:bool)->Document{
        let mut tmp_addition0:i32 = 0;
        let mut tmp_addition1:i32 = 0;
        let tmp_from:(i32,i32);
        let tmp_to:(i32,i32);

        //Jenachdem ob die Pfeile Horizontal oder Vertikal sind muss die x oder y achse und 10 verkleinert bzw vergrößert werden
        if horizontal{
                tmp_addition0 = ARROW;
        }else {
                tmp_addition1 = ARROW;
        }
        //Berechnung der neuen x/y Koordinaten für die Pfeile
        tmp_from = (from.0 + tmp_addition0,from.1 + tmp_addition1);
        tmp_to = (to.0 - tmp_addition0, to.1 - tmp_addition1);
        //Ein Pfeil an From
        if _f_arrow && !_t_arrow{
                document = document.add(extends_arrow(tmp_from, from, horizontal));
                document=document.add(broken_line((tmp_from.0 + tmp_addition0 ,tmp_from.1 + tmp_addition1), to));
        }
        //Ein Pfeil an To
        else if _t_arrow && !_f_arrow{
                document = document.add(extends_arrow(tmp_to, to, horizontal));
                document=document.add(broken_line(from, (tmp_to.0 - tmp_addition0 ,tmp_to.1 - tmp_addition1)));
        }
        //Pfeile in beiden Richtungen
        else if  _t_arrow && _f_arrow{
                document = document.add(extends_arrow(tmp_from, from, horizontal));
                document = document.add(extends_arrow(tmp_to, to, horizontal));
                document=document.add(broken_line((tmp_from.0 + tmp_addition0 ,tmp_from.1 + tmp_addition1), (tmp_to.0 - tmp_addition0 ,tmp_to.1 - tmp_addition1)));
        }
        //keine Pfeile
        else{
                document=document.add(broken_line(from, to));
        }

        document
}
pub fn aggregation(mut document:Document, from:(i32,i32), to:(i32,i32), _f_arrow:bool, _t_arrow:bool, horizontal:bool)->Document{
        let mut tmp_addition0:i32 = 0;
        let mut tmp_addition1:i32 = 0;
        let tmp_from:(i32,i32);
        let tmp_to:(i32,i32);

        //Jenachdem ob die Pfeile Horizontal oder Vertikal sind muss die x oder y achse und 10 verkleinert bzw vergrößert werden
        if horizontal{
                tmp_addition0 = ARROW;
        }else {
                tmp_addition1 = ARROW;
        }
        //Berechnung der neuen x/y Koordinaten für die Pfeile
        tmp_from = (from.0 + tmp_addition0,from.1 + tmp_addition1);
        tmp_to = (to.0 - tmp_addition0, to.1 - tmp_addition1);
        //Ein Pfeil an From
        if _f_arrow && !_t_arrow{
                document = document.add(aggregation_arrow(tmp_from, from, horizontal));
                document=document.add(line((tmp_from.0 + tmp_addition0 ,tmp_from.1 + tmp_addition1), to));
        }
        //Ein Pfeil an To
        else if _t_arrow && !_f_arrow{
                document = document.add(aggregation_arrow(tmp_to, to, horizontal));
                document=document.add(line(from, (tmp_to.0 - tmp_addition0 ,tmp_to.1 - tmp_addition1)));
        }
        //Pfeile in beiden Richtungen
        else if  _t_arrow && _f_arrow{
                document = document.add(aggregation_arrow(tmp_from, from, horizontal));
                document = document.add(aggregation_arrow(tmp_to, to, horizontal));
                document=document.add(line((tmp_from.0 + tmp_addition0 ,tmp_from.1 + tmp_addition1), (tmp_to.0 - tmp_addition0 ,tmp_to.1 - tmp_addition1)));
        }
        //keine Pfeile
        else{
                document=document.add(line(from, to));
        }

        document
}
pub fn composition(mut document:Document, from:(i32,i32), to:(i32,i32), _f_arrow:bool, _t_arrow:bool, horizontal:bool)->Document{
        let mut tmp_addition0:i32 = 0;
        let mut tmp_addition1:i32 = 0;
        let tmp_from:(i32,i32);
        let tmp_to:(i32,i32);

        //Jenachdem ob die Pfeile Horizontal oder Vertikal sind muss die x oder y achse und 10 verkleinert bzw vergrößert werden
        if horizontal{
                tmp_addition0 = ARROW;
        }else {
                tmp_addition1 = ARROW;
        }
        //Berechnung der neuen x/y Koordinaten für die Pfeile
        tmp_from = (from.0 + tmp_addition0,from.1 + tmp_addition1);
        tmp_to = (to.0 - tmp_addition0, to.1 - tmp_addition1);
        //Ein Pfeil an From
        if _f_arrow && !_t_arrow{
                document = document.add(composition_arrow(tmp_from, from, horizontal));
                document=document.add(line((tmp_from.0 + tmp_addition0 ,tmp_from.1 + tmp_addition1), to));
        }
        //Ein Pfeil an To
        else if _t_arrow && !_f_arrow{
                document = document.add(composition_arrow(tmp_to, to, horizontal));
                document=document.add(line(from, (tmp_to.0 - tmp_addition0 ,tmp_to.1 - tmp_addition1)));
        }
        //Pfeile in beiden Richtungen
        else if  _t_arrow && _f_arrow{
                document = document.add(composition_arrow(tmp_from, from, horizontal));
                document = document.add(composition_arrow(tmp_to, to, horizontal));
                document=document.add(line((tmp_from.0 + tmp_addition0 ,tmp_from.1 + tmp_addition1), (tmp_to.0 - tmp_addition0 ,tmp_to.1 - tmp_addition1)));
        }
        //keine Pfeile
        else{
                document=document.add(line(from, to));
        }
        document
}
pub fn dependency(mut document:Document, from:(i32,i32), to:(i32,i32), _f_arrow:bool, _t_arrow:bool, horizontal:bool)->Document{ 
        let mut tmp_addition0:i32 = 0;
        let mut tmp_addition1:i32 = 0;
        let tmp_from:(i32,i32);
        let tmp_to:(i32,i32);

        //Jenachdem ob die Pfeile Horizontal oder Vertikal sind muss die x oder y achse und 10 verkleinert bzw vergrößert werden
        if horizontal{
                tmp_addition0 = ARROW;
        }else {
                tmp_addition1 = ARROW;
        }
        //Berechnung der neuen x/y Koordinaten für die Pfeile
        tmp_from = (from.0 + tmp_addition0,from.1 + tmp_addition1);
        tmp_to = (to.0 - tmp_addition0, to.1 - tmp_addition1);
        //Ein Pfeil an From
        if _f_arrow && !_t_arrow{
                document = document.add(dependency_arrow(tmp_from, from, horizontal));
                document=document.add(broken_line((tmp_from.0 + tmp_addition0 ,tmp_from.1 + tmp_addition1), to));
        }
        //Ein Pfeil an To
        else if _t_arrow && !_f_arrow{
                document = document.add(dependency_arrow(tmp_to, to, horizontal));
                document=document.add(broken_line(from, (tmp_to.0 - tmp_addition0 ,tmp_to.1 - tmp_addition1)));
        }
        //Pfeile in beiden Richtungen
        else if  _t_arrow && _f_arrow{
                document = document.add(dependency_arrow(tmp_from, from, horizontal));
                document = document.add(dependency_arrow(tmp_to, to, horizontal));
                document=document.add(broken_line((tmp_from.0 + tmp_addition0 ,tmp_from.1 + tmp_addition1), (tmp_to.0 - tmp_addition0 ,tmp_to.1 - tmp_addition1)));
        }
        //keine Pfeile
        else{
                document=document.add(broken_line(from, to));
        }

        document
}
fn line(from:(i32,i32), to:(i32,i32))-> Line{
        let line = Line::new()
                .set("x1", from.0)                      //von x-Achse
                .set("y1", from.1)                      //von y-Achse
                .set("x2", to.0)                        //zu x-Achse
                .set("y2", to.1)                        //zu y-Achse
                .set("stroke", "black")                 //Rand farbe
                .set("stroke-width", 2);               //breite
        line
}
fn broken_line(from:(i32,i32), to:(i32,i32))-> Line {
        let line = Line::new()
                .set("x1", from.0)                      //von x-Achse
                .set("y1", from.1)                      //von y-Achse
                .set("x2", to.0)                        //zu x-Achse
                .set("y2", to.1)                        //zu y-Achse
                .set("stroke", "black")                 //Rand farbe
                .set("stroke-width", 2)               //breite
                .set ("stroke-dasharray", 10);
        line
}
fn ellipse(from:(i32,i32), r:(i32,i32))-> Ellipse{
        let ellipse = Ellipse::new()
                .set("cx",from.0)
                .set("cy",from.1)
                .set("rx",r.0)
                .set("ry",r.1)
                .set("style", "fill:white;stroke:black;stroke-width:2");

        ellipse
}
pub fn actor(mut document:Document,from:(i32,i32), name:String, size:i32)-> Document{
        //Körper
        document = document.add(line(from, (from.0, from.1 - ACTOR_LINE)));
        //Linker Fuß
        document = document.add(line(from, (from.0 - ACTOR_LINE, from.1 + ACTOR_LINE)));
        //Rechter Fuß
        document = document.add(line(from, (from.0 + ACTOR_LINE, from.1 + ACTOR_LINE)));
        //Kopf
        document = document.add(ellipse((from.0, from.1 - (ACTOR_LINE as f32 * 1.5) as i32),
                                        (ACTOR_LINE / 2,ACTOR_LINE / 2)));
        //Arme
        document = document.add(line((from.0 - ACTOR_LINE, from.1 - ACTOR_LINE / 2), (from.0 + ACTOR_LINE, from.1 - ACTOR_LINE / 2)));
        //Namen
        document = write(document, (from.0 - (name.chars().count() as i32) * size / 4
                                , (from.1 + size + ACTOR_LINE)), name, size);
        document
}
pub fn usecase(mut document:Document, from:(i32,i32),width:i32, height:i32, name:String, size:i32)-> Document{
        document = document.add(ellipse(from, (width,height)));
        document = write(document, (from.0 - (name.chars().count() as i32) * size / 4 ,from.1), name, size);
        
        document
}
pub fn extpoint(mut document:Document, from:(i32,i32), width:i32, height:i32, case_name:String, point_name:String, size:i32)-> Document{
        document = document.add(ellipse(from, (width, height)));
        document = document.add(line( (from.0 - width , from.1),(from.0 + width,from.1)));
        //write usecase
        document = write(document, (from.0 - (case_name.chars().count() as i32) * size / 4 ,from.1 + ARROW), case_name, size);
        //write expoint
        document = write(document, (from.0 - (point_name.chars().count() as i32) * size / 4 ,from.1 - ARROW), point_name, size);

        document
}
pub fn around_the_corner_arrow(mut document:Document, from:(i32,i32), to:(i32,i32), _f_arrow:bool, _t_arrow:bool, rel:String, max_height:i32)->Document{
        
        match rel.as_str(){
            "extends" | "extend" => {
                document = extends(document, from, (from.0,from.1+max_height), _f_arrow, false, false);
                document = document.add(line((from.0,from.1+max_height),(to.0,from.1+max_height)));
                document = extends(document, to, (to.0,from.1+max_height), _t_arrow,false , false);
            }
            "implements"=>{
                document = implements(document, from, (from.0,from.1+max_height), _f_arrow, false, false);
                document = document.add(broken_line((from.0,from.1+max_height),(to.0,from.1+max_height)));
                document = implements(document, to, (to.0,from.1+max_height), _t_arrow,false , false);
            }
            "association"=>{     
                document = association(document, from, (from.0,from.1+max_height), _f_arrow, false, false);
                document = document.add(line((from.0,from.1+max_height),(to.0,from.1+max_height)));
                document = association(document, to, (to.0,from.1+max_height), _t_arrow,false , false);
            }
            "aggregation"=>{
                document = aggregation(document, from, (from.0,from.1+max_height), _f_arrow, false, false);
                document = document.add(line((from.0,from.1+max_height),(to.0,from.1+max_height)));
                document = aggregation(document, to, (to.0,from.1+max_height), _t_arrow,false , false);
            }
            "composition"=>{
                document = composition(document, from, (from.0,from.1+max_height), _f_arrow, false, false);
                document = document.add(line((from.0,from.1+max_height),(to.0,from.1+max_height)));
                document = composition(document, to, (to.0,from.1+max_height), _t_arrow,false , false);
            }
            "dependency" | "include"=>{
                document = dependency(document, from, (from.0,from.1+max_height), _f_arrow, false, false);
                document = document.add(broken_line((from.0,from.1+max_height),(to.0,from.1+max_height)));
                document = dependency(document, to, (to.0,from.1+max_height), _t_arrow,false , false);
            }
            _=>{}
        }
        
        document
}
fn dependency_arrow(from:(i32,i32), to:(i32,i32), horizontal:bool)-> Path{
        let rel_to = (to.0-from.0,to.1-from.1);
        let extends_arrow_path;
        if horizontal{
                let extends_arrow_data = Data::new()
                        .move_to((from.0, from.1)) 
                        .line_by(rel_to)

                        .line_by((rel_to.0 *(-1), -10))
                        .line_by((rel_to.0 , 10))

                        .line_by((rel_to.0*(-1), 10))
                        .line_by((rel_to.0 , -10))
                        .close();

                extends_arrow_path = Path::new()
                        .set("fill", "none")    //Füllung
                        .set("stroke", "black") //Rand farbe
                        .set("stroke-width", 2) //Rand breite
                        .set("d", extends_arrow_data);
        }else {
                let extends_arrow_data = Data::new()
                        .move_to((from.0, from.1)) 
                        .line_by(rel_to)

                        .line_by((-ARROW, rel_to.1 *(-1)))
                        .line_by(( ARROW, rel_to.1))

                        .line_by((ARROW, rel_to.1*(-1)))
                        .line_by((-ARROW, rel_to.1))
                        .close();

                extends_arrow_path = Path::new()
                        .set("fill", "none")    //Füllung
                        .set("stroke", "black") //Rand farbe
                        .set("stroke-width", 2) //Rand breite
                        .set("d", extends_arrow_data);
        }

        extends_arrow_path
}
fn extends_arrow(from:(i32,i32), to:(i32,i32), horizontal:bool)->Path{
        let rel_to = (to.0-from.0,to.1-from.1);
        let extends_arrow_path;
        if horizontal {
                let extends_arrow_data = Data::new()
                        .move_to(from) 
                        .line_by((0, ARROW))                    //10 nach oben
                        .line_by((rel_to.0, rel_to.1 -ARROW))   //zur Spitze
                        .line_by((rel_to.0 * (-1), -ARROW))     //zum linken flügel 
                        .close();                               //zurück zum Pfeil

                extends_arrow_path = Path::new()
                        .set("fill", "white")    //Füllung
                        .set("stroke", "black") //Rand farbe
                        .set("stroke-width", 2) //Rand breite
                        .set("d", extends_arrow_data);

                
        } else {
               let extends_arrow_data = Data::new()
                        .move_to(from) 
                        .line_by((ARROW, 0))                    //10 nach links
                        .line_by((rel_to.0 -ARROW, rel_to.1 ))   //zur Spitze
                        .line_by((-ARROW, rel_to.1 * (-1)))     //zum rechten flügel 
                        .close();                               //zurück zum Pfeil

                extends_arrow_path = Path::new()
                        .set("fill", "white")    //Füllung
                        .set("stroke", "black") //Rand farbe
                        .set("stroke-width", 2) //Rand breite
                        .set("d", extends_arrow_data); 
        }
        
        extends_arrow_path
        
}
fn aggregation_arrow(from:(i32,i32), to:(i32,i32), horizontal:bool)->Path{
        let rel_to : (i32,i32);
        let tmp_from :i32;
        let aggregation_arrow_path;
        if horizontal {
                //Berchnung der werte für move_to, weil die Pfeile länger sind
                if to.0-from.0 >0 {
                        tmp_from= from.0-10;
                        rel_to = (to.0-from.0,to.1-from.1);
                }else {
                        tmp_from= from.0+10;
                        rel_to = (to.0-from.0,to.1-from.1);
                }
        
                let aggregation_arrow_data = Data::new()
                        .move_to((tmp_from,from.1))
                        .line_by((rel_to.0 , ARROW/2))
                        .line_by((rel_to.0 ,-ARROW/2))
                        .line_by((rel_to.0  * (-1), -ARROW/2))
                        .line_by((rel_to.0  * (-1), ARROW/2))
                        .close();

                aggregation_arrow_path = Path::new()
                        .set("fill", "white")    //Füllung
                        .set("stroke", "black") //Rand farbe
                        .set("stroke-width", 2) //Rand breite
                        .set("d", aggregation_arrow_data);
        }else {
                //Berchnung der werte für move_to, weil die Pfeile länger sind
                if to.1-from.1 >0 {
                        tmp_from= from.1-ARROW;
                        rel_to = (to.0-from.0,to.1-from.1);
                }else {
                        tmp_from= from.1+ARROW;
                        rel_to = (to.0-from.0,to.1-from.1);
                }
        
                let aggregation_arrow_data = Data::new()
                        .move_to((from.0,tmp_from))
                        .line_by(( ARROW/2, rel_to.1))
                        .line_by((-ARROW/2, rel_to.1))
                        .line_by((-ARROW/2, rel_to.1  * (-1)))
                        .line_by((ARROW/2, rel_to.1  * (-1)))
                        .close();

                aggregation_arrow_path = Path::new()
                        .set("fill", "white")    //Füllung
                        .set("stroke", "black") //Rand farbe
                        .set("stroke-width", 2) //Rand breite
                        .set("d", aggregation_arrow_data);
        }

        aggregation_arrow_path
}
fn composition_arrow(from:(i32,i32), to:(i32,i32), horizontal:bool)->Path{
        let rel_to : (i32,i32);
        let tmp_from :i32;
        let composition_arrow_path;
        if horizontal {
                //Berchnung der werte für move_to, weil die Pfeile länger sind
                if to.0-from.0 >0 {
                        tmp_from= from.0-10;
                        rel_to = (to.0-from.0,to.1-from.1);
                }else {
                        tmp_from= from.0+10;
                        rel_to = (to.0-from.0,to.1-from.1);
                }
        
                let composition_arrow_data = Data::new()
                        .move_to((tmp_from,from.1))
                        .line_by((rel_to.0 , ARROW/2))
                        .line_by((rel_to.0 ,-ARROW/2))
                        .line_by((rel_to.0  * (-1), -ARROW/2))
                        .line_by((rel_to.0  * (-1), ARROW/2))
                        .close();

                composition_arrow_path = Path::new()
                        .set("fill", "black")    //Füllung
                        .set("stroke", "black") //Rand farbe
                        .set("stroke-width", 2) //Rand breite
                        .set("d", composition_arrow_data);
        }else {
                //Berchnung der werte für move_to, weil die Pfeile länger sind
                if to.1-from.1 >0 {
                        tmp_from= from.1-ARROW;
                        rel_to = (to.0-from.0,to.1-from.1);
                }else {
                        tmp_from= from.1+ARROW;
                        rel_to = (to.0-from.0,to.1-from.1);
                }
        
                let composition_arrow_data = Data::new()
                        .move_to((from.0,tmp_from))
                        .line_by(( ARROW/2, rel_to.1))
                        .line_by((-ARROW/2, rel_to.1))
                        .line_by((-ARROW/2, rel_to.1  * (-1)))
                        .line_by((ARROW/2, rel_to.1  * (-1)))
                        .close();

                composition_arrow_path = Path::new()
                        .set("fill", "black")    //Füllung
                        .set("stroke", "black") //Rand farbe
                        .set("stroke-width", 2) //Rand breite
                        .set("d", composition_arrow_data);
        }

        composition_arrow_path
}
pub fn write(mut document:Document, from:(i32,i32), _string:String, size:i32)->Document{
        let text = Text::new()
        .set("x",from.0)
        .set("y",from.1)
        .set("fill","black")
        .set("font-size",size)
        .add(svg::node::Text::new(_string));

        document = document.add(text);
        document
}
pub fn class_enum_to_string(name:&parsing::parse_class::Beziehungstyp)->String{
       
        let mut string:String = String::new();
        match *name{
            parsing::parse_class::Beziehungstyp::EXTENDS => {
                string = "extends".to_string();
            }
            parsing::parse_class::Beziehungstyp::IMPLEMENTS=>{
                string = "implements".to_string();
            }
            parsing::parse_class::Beziehungstyp::ASSOCIATION=>{     
               string = "association".to_string();
            }
            parsing::parse_class::Beziehungstyp::AGGREGATION=>{
               string = "aggregation".to_string();
            }
            parsing::parse_class::Beziehungstyp::COMPOSITION=>{
                string = "composition".to_string();
            }
            parsing::parse_class::Beziehungstyp::DEPENDENCY=>{
               string = "dependency".to_string(); 
            }
            parsing::parse_class::Beziehungstyp::UNDEFINED=>{}
        }
        string
}
pub fn usecase_enum_to_string(name:&parsing::parse_usecase::Beziehungstyp)->String{

        let mut string:String = String::new();
        match *name{
            parsing::parse_usecase::Beziehungstyp::EXTENDS => {
                string = "extends".to_string();
            }
            parsing::parse_usecase::Beziehungstyp::INCLUDE=>{
                string = "include".to_string();
            }
            parsing::parse_usecase::Beziehungstyp::ASSOCIATION=>{     
               string = "association".to_string();
            }
            parsing::parse_usecase::Beziehungstyp::EXTEND=>{
               string = "extend".to_string();
            }
            parsing::parse_usecase::Beziehungstyp::GENERALIZATION=>{
                string = "generaliation".to_string();
            }
            parsing::parse_usecase::Beziehungstyp::UNDEFINED=>{}
        }
        string
}
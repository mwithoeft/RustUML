use std::string::String;
use svg::Document;
use parsing;
use svglib;

const WIDTH: i32 = 1680;
const HEIGHT: i32 = 720;
const TITEL_MIN: i32 = 30;
const FONT_SIZE_TITLE: i32 = 20;
const FONT_SIZE: i32 = 10;

const SUB_HEIGHT: i32 = HEIGHT - (HEIGHT / 9);
const SUB_WIDTH: i32 = (WIDTH / 16) * 10;
const MIN_RAND_LINKS: i32 = 5;
const MIN_RAND_OBEN: i32 = 10;
const ACTOR_L :i32 = (WIDTH - SUB_WIDTH) / 4;
const ACTOR_R :i32 = 3 * ACTOR_L + SUB_WIDTH;
const ACTOR_O :i32 = HEIGHT / 4;
const ACTOR_U :i32 = (HEIGHT / 4) * 3;
const USECASEHOEHE : i32 = 50;
const USECASE_MIN_OBEN: i32 = 120;
const USECASE_MIN_RECHTS: i32 = 90;
const ACTOR_LINE:i32 = 30;
const ARROW_SPACE:i32 = 15;

struct Usecase{
    id:i32,
    center: (i32,i32),
    breite: i32,
    hoehe: i32,
    rel_point_loru: ((i32,i32),(i32,i32),(i32,i32),(i32,i32)),
    name: String,
    ext_name: String,
    row :i32,
    column : i32
}

fn build_usecase(id: i32, name: String, breite:i32, hoehe: i32)->Usecase{
    Usecase {
        id,
        center: (0,0),
        breite,
        hoehe,
        rel_point_loru:((0,0),(0,0),(0,0),(0,0)),
        name,
        ext_name: String::new(),
        row: 0,
        column: 0,
    }
}
struct Actor{
    id:i32,
    rel_point: (i32,i32),
    name: String,
    
}
fn build_actor(id: i32, rel_point: (i32,i32), name:String)->Actor{
    Actor {
        id,
        rel_point,
        name, 
    }
}

pub fn build_usecase_diagramm(typen: &mut Vec<parsing::parse_usecase::Typ>, beziehungen : &mut Vec<parsing::parse_usecase::Beziehung>) -> Document{
    let mut document = Document::new()
            .set("viewBox", (WIDTH-WIDTH, HEIGHT - HEIGHT , WIDTH, HEIGHT)) // Bild größe
            .set("id","zoom");
    let mut usecase: Vec<Usecase>= Vec::new(); 
    let mut usecase_id: i32 = 0;
    let mut actor: Vec<Actor>= Vec::new(); 
    let mut actor_id: i32 = 0;
    let mut sub :i32 = 0; // Kontrollvariable damit es nur ein subjekt gibt
    //let mut ex_points: Vec<Ex> = Vec::new();

    for t in typen{
        match t._elementtyp{
            parsing::parse_usecase::TypEnum::SUBJECT =>{
                //Subject wird gezeichnet
                if sub==0{
                    sub=1;
                    let mut x: i32;
                    let mut y: i32;
                    x = WIDTH / 2 - SUB_WIDTH / 2;
                    y = HEIGHT / 2 - SUB_HEIGHT / 2;
                    document = svglib::rectangle(document, x, y, SUB_WIDTH, SUB_HEIGHT);
                    document = svglib::write(document, (x + MIN_RAND_LINKS, y + FONT_SIZE_TITLE), t._elementname.to_string(), FONT_SIZE_TITLE);
                } 
            }
            parsing::parse_usecase::TypEnum::ACTOR =>{
                //Es dürfen nur 4 Actore vorhanden sein
                if actor_id < 5 {
                    actor_id = actor_id + 1;
                    let mut c: (i32,i32) = (0,0);
                    if actor_id <= 2 {
                        c.0 = ACTOR_L as i32;
                    }else {
                        c.0 = ACTOR_R as i32;
                    }
                    if actor_id % 2 == 0 {
                        c.1 = ACTOR_U as i32;
                    }else {
                        c.1 = ACTOR_O as i32;
                    }
                    let mut point:(i32,i32)=(c.0 + ACTOR_LINE + 5 ,c.1 - ACTOR_LINE / 2);
                    if actor_id > 2{
                        point = (point.0 - 10 - 2 * ACTOR_LINE, point.1);
                    }
                    actor.push(build_actor(actor_id, point, t._elementname.to_string()));
                    document = svglib::actor(document, c, t._elementname.to_string(), FONT_SIZE);
                }
            }
            parsing::parse_usecase::TypEnum::USECASE =>{
                //Usecase Strucktur wird erstellt
                usecase_id = usecase_id + 1;
                usecase.push(build_usecase(usecase_id, t._elementname.to_string(), t._elementname.chars().count() as i32 * FONT_SIZE  + 10, USECASEHOEHE));
               /* for u in &mut usecase {
                    if usecase_id == u.id{
                        u.name = t._elementname.to_string();
                        u.breite = t._elementname.chars().count() as i32 * FONT_SIZE  + 10;
                        u.hoehe = USECASEHOEHE;
                    }
                }*/
            }
            parsing::parse_usecase::TypEnum::EXTPOINT =>{
                //Extpoints werden in die usecase strucktur aufgenommen
                for u in &mut usecase{
                    if u.name == t._behaelter.to_string() {
                        u.ext_name = t._elementname.to_string();
                    } else {   
                    } 
                }
            }
            parsing::parse_usecase::TypEnum::UNDEFINED =>{}
        }
    }

    let mut x = WIDTH / 2 - SUB_WIDTH / 2;
    let mut y = HEIGHT / 2 - SUB_HEIGHT / 2 + TITEL_MIN + USECASE_MIN_OBEN / 2 + USECASEHOEHE / 2;
    let mut res_flag = 0; //c
    let mut row = 1;
    let mut column = 0;

    //Usecase wird gezeichnet
    for u in &mut usecase{
        // Wenn das case noch in die reihe passt
        if x < (WIDTH / 2 + SUB_WIDTH / 2) - USECASE_MIN_RECHTS - u.breite {
            x = x + u.breite / 2 + USECASE_MIN_RECHTS;
        }else {
            x = WIDTH / 2 - SUB_WIDTH / 2 + u.breite / 2 + USECASE_MIN_RECHTS; 
            res_flag = 1;
            column = 0;
        }
        //Wenn eine Reihe voll ist wird das spalten reset falg gesetzt
        if res_flag != 0 {
            res_flag = 0;
            row = row +1;
            y = y + USECASE_MIN_OBEN + USECASEHOEHE / 2;
        }else {
            
        }
        u.center = (x as i32,y as i32);
        u.rel_point_loru = ((u.center.0 - u.breite / 2 ,u.center.1),
                            (u.center.0,u.center.1 - u.hoehe / 2),
                            (u.center.0  + u.breite / 2, u.center.1),
                            (u.center.0, u.center.1 + u.hoehe / 2));
        u.row = row;
        column = column +1;
        u.column = column; 
        if u.ext_name.is_empty() {
            document = svglib::usecase(document, u.center, u.breite / 2, u.hoehe / 2, u.name.to_string(), FONT_SIZE);
            //draw_hollow_ellipse_mut(&mut image, u.center, u.breite as i32 / 2 , u.hoehe as i32 /2 , _black);
            //draw_text_mut(&mut image, _black, x - (u.name.chars().count() as u32 * FONT_SIZE as u32 /2), y ,scale, &font, & u.name);
        }else {
            /*
            let mut breite: f32 = u.breite as f32;
            let mut _hoehe: f32 = u.hoehe as f32 * 2.0;
            if breite < u.ext_name.chars().count() as f32 * FONT_SIZE as f32 {
                breite = u.ext_name.chars().count() as f32 * FONT_SIZE as f32 
            }*/
            document = svglib::extpoint(document, u.center, u.breite / 2 , u.hoehe / 2 , u.name.to_string(), u.ext_name.to_string(), FONT_SIZE);
            //draw_hollow_ellipse_mut(&mut image, u.center, breite as i32 / 2 , hoehe as i32 /2 , _black);
            //draw_line_segment_mut(&mut image, (u.center.0 as f32 - breite/2.0 , u.center.1 as f32), (u.center.0 as f32 + breite/2.0 , u.center.1 as f32), _black);
            //draw_text_mut(&mut image, _black, x - (u.name.chars().count() as u32 * FONT_SIZE as u32 /2), y -10 ,scale, &font, & u.name);
            //draw_text_mut(&mut image, _black, x - (u.ext_name.chars().count() as u32 * FONT_SIZE as u32 /2), y +10 ,scale, &font, & u.ext_name);
        }
        x = x + u.breite / 2;
    }
    //Beziehungen werden gezeichnet
    let mut max_height = USECASE_MIN_OBEN * (4/3);
    for b in beziehungen{
        let mut from: (i32,i32) = (1,1);
        let mut to: (i32,i32) = (1,1);
        let mut temp_point_from: ((i32,i32),(i32,i32),(i32,i32),(i32,i32)) = ((1,1),(1,1),(1,1),(1,1));
        let mut temp_point_to: ((i32,i32),(i32,i32),(i32,i32),(i32,i32)) = ((1,1),(1,1),(1,1),(1,1));
        let mut temp_from_row:i32 = -1;
        let mut temp_to_row:i32 = -1;
        let mut temp_from_column:i32 = -1;
        let mut temp_to_column:i32 = -1;
        let mut temp_from_id:i32 = -1;
        let mut temp_to_id:i32 = -1;
        let mut name = b._von_element_name.pop();
        let mut horizontal:bool = true;
        let mut tausch:bool = false;
        let mut rel_finish:bool = false;

        for u in &usecase {
            //stimmt der erste name der beziehung mit einem usecase überein
            // Some wird benötigt, weil name aus dem Vector gepopt wird.
            if name == Some(u.name.to_string()){
                temp_from_row = u.row;
                temp_point_from = u.rel_point_loru;
                temp_from_column = u.column
               
            }
            //stimmt der zweite name der beziehung mit einem usecas überein
            if b._zu_element_name == u.name.to_string(){
                temp_to_row = u.row;
                temp_point_to = u.rel_point_loru;
                temp_to_column = u.column
            }
            //wenn beide namen der beziehung gefunden wurden
            if temp_from_row != -1 && temp_to_row != -1{
                //lieg der erste usecase über dem zweiten
                if temp_from_row < temp_to_row{
                    from = temp_point_from.3;
                    to = temp_point_to.1;  
                    horizontal = false;
                }
                //lieg der erste usecase unter dem zweiten  
                else if temp_from_row > temp_to_row {
                    from = temp_point_from.1;
                    to = temp_point_to.3;
                    horizontal = false;
                }
                //sind sie nebeneinander
                else {
                    //wen sie in einer reihe sind aber nicht nebeneinander
                    if !(temp_from_column + 1 == temp_to_column || temp_from_column - 1 == temp_to_column) {
                        from = temp_point_from.3;
                        to = temp_point_to.3;
                        max_height = max_height - ARROW_SPACE- ARROW_SPACE;
                        document = svglib::around_the_corner_arrow(document, from, to, tausch, !tausch, svglib::usecase_enum_to_string(&b._beziehungstyp), max_height);
                        rel_finish = true;
                        max_height = max_height + ARROW_SPACE;
                        break;
                    }
                    //from links, to rechts
                    else if temp_from_column < temp_to_column {
                        from = temp_point_from.2;
                        to = temp_point_to.0;
                        //tausch = false;
                        horizontal = true;
                    }else if temp_from_column > temp_to_column {
                        //from und to koordinaten und ob sie Pfeile haben müssen getauscht werden, weil sonst der Pfeil falsch dargestellt wird
                        from = temp_point_to.2;
                        to = temp_point_from.0;
                        tausch = true;
                        horizontal = true;
                    }
                }
            }
            
        }
        //Wenn nicht beide teile der beziehung in usecase gefunden wurde
        if temp_from_row == -1 || temp_to_row == -1{

            for a in &actor {
                //stimmt der name des ersten beziehungselement mit einem Actor überein
                if name == Some(a.name.to_string()){
                    if a.id == 1 || a.id == 3 {
                        temp_from_row = 1;
                    } else {
                        temp_from_row = 3;
                    }
                    temp_from_id = a.id;
                    from = a.rel_point;
                }
                //stimmt der name des zweiten beziehungselement mit einem Actor überein
                if b._zu_element_name == a.name.to_string(){
                    if a.id == 1 || a.id == 3 {
                        temp_to_row = 1;
                    } else {
                        temp_to_row = 3;
                    }
                    temp_to_id = a.id;
                    to = a.rel_point;
                }
                //wurde für beide beziehungstypen ein element gefunden?
                if temp_from_row != -1 && temp_to_row != -1 && !rel_finish{
                            
                    //sind es beide Actoure
                    if temp_from_id != -1 && temp_to_id != -1{
                        //Wenn sie gegenüber liegen
                        if to.0 == from.0{
                            //tausch = true;
                            horizontal = false;
                        }
                        else if temp_from_row == temp_to_row || temp_from_row != temp_to_row{
                            max_height = max_height - ARROW_SPACE;
                            document = svglib::around_the_corner_arrow(document, from, to, tausch, !tausch, svglib::usecase_enum_to_string(&b._beziehungstyp) , max_height);
                            rel_finish = true;
                            break;
                        }//Wenn sie in einer spalte liegen
                    }else {
                        //Wenn from ein actor ist 
                        if temp_to_id == -1 && temp_from_id != -1 {
                            //Wenn from über usecase liegt
                            if temp_from_row < temp_to_row {
                                to = temp_point_to.3;
                                
                            }
                            // Wenn from unter usecase liegt
                            else if temp_from_row > temp_to_row {
                                to = from;
                                from = temp_point_to.3;
                                tausch = true;
                                horizontal = false;
                            }
                            //Wenn beiede auf gleicher höhe sind
                            else{
                                //Wenn der Actor rechts ist
                                if temp_from_id > 2 {
                                    //Wenn der Actor neben dem Usecase ist
                                    if temp_to_row == 1 {
                                        to = temp_point_to.2;
                                        
                                    }
                                    //Wenn sie nicht nebeneinander sind
                                    else{
                                        to = temp_point_to.3;
                                        max_height = max_height - ARROW_SPACE;
                                        document = svglib::around_the_corner_arrow(document, from, to, tausch, !tausch, svglib::usecase_enum_to_string(&b._beziehungstyp) , max_height);
                                        rel_finish = true;
                                        break;
                                    }
                                }
                                //Wenn der Actor links ist
                                else {
                                    //Wenn der Actor neben dem Usecase ist
                                    if temp_to_column == 1 {
                                        to = temp_point_to.0;
                                        
                                    }
                                    //Wenn sie nicht nebeneinander sind
                                    else{
                                        to = temp_point_to.3;
                                        max_height = max_height - ARROW_SPACE;
                                        document = svglib::around_the_corner_arrow(document, from, to, tausch, !tausch, svglib::usecase_enum_to_string(&b._beziehungstyp) , max_height);
                                        rel_finish = true;
                                        break;
                                    }
                                }
                                //horizontal = true;
                            }
                        }
                        //Wenn to ein actor ist
                        else if temp_from_id == -1 && temp_to_id != -1 {

                            if temp_from_row > temp_to_row {
                                to = temp_point_from.3;
                                
                            }else if temp_from_row < temp_to_row {
                                to = temp_point_from.1;
                                
                            }else{
                                if temp_from_id >2 {
                                    to = temp_point_from.2;
                                   
                                }else {
                                    to = temp_point_from.0;
                                }
                            }
                        }
                    }
                }
            }
            
        }

        if !rel_finish{
            match b._beziehungstyp {
                parsing::parse_usecase::Beziehungstyp::ASSOCIATION =>{
                    document = svglib::association(document, from, to, tausch, !tausch, horizontal);
                }
                parsing::parse_usecase::Beziehungstyp::GENERALIZATION =>{
                }
                parsing::parse_usecase::Beziehungstyp::INCLUDE =>{
                    document = svglib::dependency(document, from, to, tausch, !tausch, horizontal);
                }
                parsing::parse_usecase::Beziehungstyp::EXTEND =>{
                    document = svglib::extends(document, from, to, tausch, !tausch, horizontal);
                }
                parsing::parse_usecase::Beziehungstyp::EXTENDS =>{
                    document = svglib::extends(document, from, to, tausch, !tausch, horizontal);
                }
                parsing::parse_usecase::Beziehungstyp::UNDEFINED =>{
                }
            }   
        }
        //reset
        /*
        temp_from_row = -1; 
        temp_to_row = -1; 
        temp_from_id = -1; 
        temp_to_id = -1; 
        temp_from_column = -1; 
        temp_to_column = -1;
        horizontal = true;
        from = (1,1);
        to = (1,1);
        temp_point_from = ((1,1),(1,1),(1,1),(1,1));
        temp_point_to = ((1,1),(1,1),(1,1),(1,1));
        tausch = false;
        rel_finish = false;
        */
    }
    document
}
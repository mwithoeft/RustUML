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
const MIN_RAND_OBEN: i32 = 5;
const R: i32 = 40;
const ACTOR_L :i32 = (WIDTH - SUB_WIDTH) / 4;
const ACTOR_R :i32 = 3 * ACTOR_L + SUB_WIDTH;
const ACTOR_O :i32 = HEIGHT / 4;
const ACTOR_U :i32 = (HEIGHT / 4) * 3;
const ACTOR_ABSTAND :i32 = 350;
const ACTOR_SPANNWEITE : i32 = 100;
const ACTOR_BEINE : i32 = 30;
const ACTOR_BEINE_WINKEL : i32 = 30;
const USECASEHOEHE : i32 = 50;
const USECASE_MIN_OBEN: i32 = 120;
const USECASE_MIN_RECHTS: i32 = 90;

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

fn buildUsecase(id: i32, name: String, breite:i32, hoehe: i32)->Usecase{
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
    breite: u32,
    hoehe: u32,
    rel_point: (i32,i32),
    name: String,
    
}
fn buildActor(id: i32, rel_point: (i32,i32), name:String)->Actor{
    Actor {
        id,
        breite: 1,
        hoehe: 0,
        rel_point,
        name, 
    }
}
struct Ex{
    name:String,
    ex_point:String
}
fn buildEx(name: String, ex_point: String)->Ex{
    Ex {
        name,
        ex_point,
    }
}

pub fn build_usecase_diagramm(mut typen: &mut Vec<parsing::parse_usecase::Typ>, mut beziehungen : &mut Vec<parsing::parse_usecase::Beziehung>) -> Document{
    let mut document = Document::new()
            .set("viewBox", (WIDTH-WIDTH, HEIGHT - HEIGHT , WIDTH, HEIGHT)) // Bild größe
            .set("id","zoom");
    let mut usecase: Vec<Usecase>= Vec::new(); 
    let mut usecase_id: i32 = 0;
    let mut actor: Vec<Actor>= Vec::new(); 
    let mut actor_id: i32 = 0;
    let mut sub :i32 = 0; // Kontrollvariable damit es nur ein subjekt gibt
    let mut ex_points: Vec<Ex> = Vec::new();

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
                    document = svglib::write(document, (x + MIN_RAND_LINKS, y + MIN_RAND_OBEN), t._elementname.to_string(), FONT_SIZE);
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
                    document = svglib::actor(document, c, t._elementname.to_string(), FONT_SIZE);
                }
            }
            parsing::parse_usecase::TypEnum::USECASE =>{
                //Usecase Strucktur wird erstellt
                usecase_id = usecase_id + 1;
                usecase.push(buildUsecase(usecase_id, t._elementname.to_string(), t._elementname.chars().count() as i32 * FONT_SIZE  + 10, USECASEHOEHE));
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
    let mut res_flag = 0;
    let mut row = 1;
    let mut column = 0;

    //Usecase wird gezeichnet
    for u in &mut usecase{
        if x < (WIDTH / 2 + SUB_WIDTH / 2) - USECASE_MIN_RECHTS - u.breite {
            x = x + u.breite / 2 + USECASE_MIN_RECHTS;
        }else {
            x = WIDTH / 2 - SUB_WIDTH / 2 + u.breite / 2 + USECASE_MIN_RECHTS; 
            res_flag = 1;
            column = 0;
        }
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
        u.column = u.column; //?
        //
        if u.ext_name.is_empty() {
            document = svglib::usecase(document, u.center, u.name.to_string(), FONT_SIZE);
            //draw_hollow_ellipse_mut(&mut image, u.center, u.breite as i32 / 2 , u.hoehe as i32 /2 , _black);
            //draw_text_mut(&mut image, _black, x - (u.name.chars().count() as u32 * FONT_SIZE as u32 /2), y ,scale, &font, & u.name);
        }else {
            let mut breite: f32 = u.breite as f32;
            let mut hoehe: f32 = u.hoehe as f32 * 2.0;
            if breite < u.ext_name.chars().count() as f32 * FONT_SIZE as f32 {
                breite = u.ext_name.chars().count() as f32 * FONT_SIZE as f32 
            }
            document = svglib::extpoint(document, u.center, u.breite, u.hoehe, u.name.to_string(), u.ext_name.to_string(), FONT_SIZE);
            //draw_hollow_ellipse_mut(&mut image, u.center, breite as i32 / 2 , hoehe as i32 /2 , _black);
            //draw_line_segment_mut(&mut image, (u.center.0 as f32 - breite/2.0 , u.center.1 as f32), (u.center.0 as f32 + breite/2.0 , u.center.1 as f32), _black);
            //draw_text_mut(&mut image, _black, x - (u.name.chars().count() as u32 * FONT_SIZE as u32 /2), y -10 ,scale, &font, & u.name);
            //draw_text_mut(&mut image, _black, x - (u.ext_name.chars().count() as u32 * FONT_SIZE as u32 /2), y +10 ,scale, &font, & u.ext_name);
        }
        x = x + u.breite / 2;
    }
    //Beziehungen werden gezeichnet
    for b in beziehungen{
        let mut from: (i32,i32) = (1,1);
        let mut to: (i32,i32) = (1,1);
        let mut temp_point_from: ((i32,i32),(i32,i32),(i32,i32),(i32,i32)) = ((1,1),(1,1),(1,1),(1,1));
        let mut temp_point_to: ((i32,i32),(i32,i32),(i32,i32),(i32,i32)) = ((1,1),(1,1),(1,1),(1,1));
        let mut temp_from_row:i32 = -1; //reset
        let mut temp_to_row:i32 = -1; //reset
        let mut temp_from_id:i32 = -1; //reset
        let mut temp_to_id:i32 = -1; //reset
        let mut temp_from_column:i32 = -1; 
        let mut temp_to_column:i32 = -1;
        let mut name = b._von_element_name.pop();
       

        for u in &usecase {
            //stimmt der erste name der beziehung mit einem usecas überein
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
            //wurden beide namen der beziehung gefunden
            if !(temp_from_row == -1 && temp_to_row == -1){
                //lieg der erste usecase über dem zweiten
                if temp_from_row < temp_to_row{
                    from = temp_point_from.3;
                    to = temp_point_to.1;
                //lieg der erste usecase unter dem zweiten    
                }else if temp_from_row > temp_to_row {
                    from = temp_point_from.1;
                    to = temp_point_to.3;
                }else {//sind sie nebeneinander
                    if temp_from_column < temp_to_column {
                        from = temp_point_from.0;
                        to = temp_point_to.2;
                    }else {
                        from = temp_point_from.2;
                        to = temp_point_to.0;
                    }
                }
            }
            
        }
        //Wenn nicht beide teile der beziehung gefunden wurde
        if temp_from_row == -1 || temp_to_row == -1{

            for a in &actor {
                //stimmt der name des ersten beziehungselement mit einem Actor überein
                if name == Some(a.name.to_string()){
                   
                    if a.id == 1 || a.id == 3 {
                        temp_from_id = a.id;
                        temp_from_row = 1;
                       
                        
                    } else {
                        
                        temp_from_id = a.id;
                        temp_from_row = 2;
                    }
                    
                    from = a.rel_point;
                    
            
                }
                //stimmt der name des zweiten beziehungselement mit einem Actor überein
                if b._zu_element_name == a.name.to_string(){
                    if a.id == 1 || a.id == 3 {
                        temp_to_id = a.id;
                        temp_to_row = 1;
                    } else {
                        temp_from_id = a.id;
                        temp_from_row = 2;
                    }
                    to = a.rel_point;
                }
                //wurde für beide beziehungstypen ein element gefunden?
                if !(temp_from_row == -1 && temp_to_row == -1){
                            
                    //sind es beide Actoure
                    if temp_from_id != -1 && temp_to_id != -1{
                        //Punkte sind schon gesetzt    
                        
                    }else if temp_to_row != -1 && temp_from_row != -1{
                            print!("2:\n{},{}\n",from.0,from.1);
                            print!("{},{}\n",to.0,to.1);
                            print!("{}\n",a.name);
                            print!("2:\n{},{}\n",temp_to_id,temp_from_id);
                        if temp_to_id != -1 && temp_to_id == -1{
                            print!("test");
                            if temp_from_row > temp_to_row {
                                from = temp_point_from.1;
                            }else if temp_from_row < temp_to_row {
                                from = temp_point_from.3;
                            }else{
                                if temp_to_id >2 {
                                    from = temp_point_from.2;
                                }else {
                                    from = temp_point_from.0;
                                }
                            }
                        }
                        else if temp_from_id == -1 && temp_from_id != -1 {
                            print!("3:\n{},{}\n",from.0,from.1);
                            print!("{},{}\n",to.0,to.1);
                            print!("{}\n",a.name);
                            print!("2:\n{},{}\n",temp_to_id,temp_from_id);
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

                        if temp_from_row < temp_to_row{
                            
                            from = temp_point_from.3;
                            to = temp_point_to.1;
                            
                        }else if temp_from_row > temp_to_row {
                            from = temp_point_from.1;
                            to = temp_point_to.3;
                        }
                    }
                
                }
            }
            
        }
        //reset
            temp_from_row = -1; //reset
            temp_to_row = -1; //reset
            temp_from_id = -1; //reset
            temp_to_id = -1; //reset
            temp_from_column = -1; 
            temp_to_column = -1;

        match b._beziehungstyp {
            parsing::parse_usecase::Beziehungstyp::ASSOCIATION =>{
                document = svglib::association(document, from, to, false, false, true);
                //draw_line_segment_mut(&mut image, from, to, _black);
            }
            parsing::parse_usecase::Beziehungstyp::GENERALIZATION =>{

            }
            parsing::parse_usecase::Beziehungstyp::INCLUDE =>{
                //draw_line_segment_mut(&mut image, from, to, _black);
                //draw_line_segment_mut(&mut image, (to.0 -10.0 , to.1 +10.0) , to, _black);
                //draw_line_segment_mut(&mut image, (to.0 -10.0 , to.1 -10.0) , to, _black);
            }
            parsing::parse_usecase::Beziehungstyp::EXTEND =>{
                document = svglib::extends(document, from, to, false, true, true);
                //draw_line_segment_mut(&mut image, from, to, _black);
                //draw_line_segment_mut(&mut image, (to.0 -10.0 , to.1 +10.0) , to, _black);
                //draw_line_segment_mut(&mut image, (to.0 -10.0 , to.1 -10.0) , to, _black);
                //draw_line_segment_mut(&mut image, (to.0 -10.0 , to.1 -10.0) , (to.0 -10.0 , to.1 +10.0), _black);
            }
            parsing::parse_usecase::Beziehungstyp::EXTENDS =>{
                document = svglib::extends(document, from, to, false, true, true);
                //draw_line_segment_mut(&mut image, from, to, _black);
                //draw_line_segment_mut(&mut image, (to.0 -10.0 , to.1 +10.0) , to, _black);
                //draw_line_segment_mut(&mut image, (to.0 -10.0 , to.1 -10.0) , to, _black);
                //draw_line_segment_mut(&mut image, (to.0 -10.0 , to.1 -10.0) , (to.0 -10.0 , to.1 +10.0), _black);
            }
            parsing::parse_usecase::Beziehungstyp::UNDEFINED =>{
            }
        }
    }
    document
}
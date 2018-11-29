use image::{ Rgb, RgbImage};
use std::string::String;
use imageproc::rect::Rect;
use rusttype::{FontCollection, Scale};
use imageproc::drawing::{
    draw_hollow_rect_mut,
    draw_hollow_ellipse_mut,
    draw_filled_rect_mut,
    draw_line_segment_mut,
    draw_text_mut,
    draw_hollow_circle_mut
    
};

use parsing;


const WIDTH: u32 = 1680;
const HEIGHT: u32 = 720;
const TITEL_MIN: u32 = 30;
const FONT_SIZE_TITLE: f32 = 20.0;
const FONT_SIZE: f32 = 10.0;

const SUB_HEIGHT: u32 = HEIGHT - (HEIGHT / 9);
const SUB_WIDTH: u32 = (WIDTH / 16) * 10;
const MIN_RAND_LINKS: u32 = 5;
const MIN_RAND_OBEN: u32 = 5;
const R: i32 = 40;
const ACTOR_L :u32 = (WIDTH - SUB_WIDTH) / 4;
const ACTOR_R :u32 = 3 * ACTOR_L + SUB_WIDTH;
const ACTOR_O :u32 = 80;
const ACTOR_ABSTAND :u32 = 350;
const ACTOR_U :u32 = ACTOR_O + ACTOR_ABSTAND ;
const ACTOR_SPANNWEITE : f32 = 100.0;
const ACTOR_BEINE : f32 = 30.0;
const ACTOR_BEINE_WINKEL : f32 = 30.0;
const USECASEHOEHE : u32 = 50;
const USECASE_MIN_OBEN: u32 = 120;
const USECASE_MIN_RECHTS: u32 = 90;



struct Usecase{
    id:i32,
    center: (i32,i32),
    breite: u32,
    hoehe: u32,
    rel_point_loru: ((f32,f32),(f32,f32),(f32,f32),(f32,f32)),
    name: String,
    ext_name: String,
    row :i32,
    column : i32
}

fn buildUsecase(id: i32)->Usecase{
    Usecase {
        id,
        center: (0,0),
        breite: 1,
        hoehe: 0,
        rel_point_loru:((0.0,0.0),(0.0,0.0),(0.0,0.0),(0.0,0.0)),
        name: String::new(),
        ext_name: String::new(),
        row: 0,
        column: 0,
    }



}

struct Actor{
    id:i32,
    breite: u32,
    hoehe: u32,
    rel_point: (f32,f32),
    name: String,
    
}

fn buildActor(id: i32, rel_point: (f32,f32), name:String)->Actor{
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
pub fn build_usecase_diagramm(mut typen: &mut Vec<parsing::parse_usecase::Typ>, mut beziehungen : &mut Vec<parsing::parse_usecase::Beziehung>){
    let mut image = creat_png(); // Das bild
    let mut usecase: Vec<Usecase>= Vec::new(); 
    let mut usecase_id: i32 = 0;
    let mut actor: Vec<Actor>= Vec::new(); 
    let mut actor_id: i32 = 0;
    let mut sub :i32 = 0; // Kontrollvariable damit es nur ein subjekt gibt
    let mut ex_points: Vec<Ex> = Vec::new();

    // Zum malen und schreiben
    let _black = Rgb ([0u8, 0u8, 0u8]);
    let title_scale = Scale { x: FONT_SIZE_TITLE*2.0 , y: FONT_SIZE_TITLE};
    let scale = Scale { x: FONT_SIZE * 2.0 , y: FONT_SIZE};
    let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();


    for t in typen{
        
        match t._elementtyp{
            parsing::parse_usecase::TypEnum::SUBJECT =>{
                if sub==0{
                    sub=1;
                    let mut x: u32;
                    let mut y: u32;
                    x = WIDTH / 2 - SUB_WIDTH / 2;
                    y = HEIGHT / 2 - SUB_HEIGHT / 2;
                    draw_hollow_rect_mut( &mut image, Rect::at(x as i32, y as i32).of_size(SUB_WIDTH, SUB_HEIGHT), _black);
                    draw_hollow_rect_mut( &mut image, Rect::at(x as i32, y as i32).of_size(SUB_WIDTH, TITEL_MIN), _black);
                    draw_text_mut(&mut image, _black, x + MIN_RAND_LINKS, y + MIN_RAND_OBEN, title_scale, &font, &t._elementname);
                } 
            }
            parsing::parse_usecase::TypEnum::ACTOR =>{
                
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
                    draw_hollow_circle_mut(&mut image, c , R, _black);
                    let mut x:(f32,f32) = (0.0,0.0);
                    let mut y:(f32,f32) = (0.0,0.0);
                    x.0 = c.0 as f32;
                    x.1 = (c.1 + R) as f32;
                    y.0 = x.0;
                    y.1 = x.1 + ACTOR_SPANNWEITE;
                    draw_line_segment_mut(&mut image,x,y,_black);
                    draw_line_segment_mut(&mut image,(x.0 - ACTOR_SPANNWEITE / 2.0  ,x.1 + ACTOR_SPANNWEITE / 2.0),(x.0 + ACTOR_SPANNWEITE / 2.0,x.1 + ACTOR_SPANNWEITE / 2.0),_black);
                    actor.push(buildActor(actor_id, (x.0 + ACTOR_SPANNWEITE / 2.0,x.1 + ACTOR_SPANNWEITE / 2.0), t._elementname.to_string()));
                    x.0 = y.0;
                    x.1 = y.1;
                    y.0 = y.0 + ACTOR_BEINE_WINKEL;
                    y.1 = y.1 + ACTOR_BEINE;
                    draw_line_segment_mut(&mut image,x,y,_black);
                    y.0 = x.0 - ACTOR_BEINE_WINKEL;
                    y.1 = x.1 + ACTOR_BEINE;
                    draw_line_segment_mut(&mut image,x,y,_black);
                    let mut x1:u32 = x.0 as u32;
                    let mut y1:u32 = x.1 as u32; 
                    x1 = x1-(t._elementname.chars().count() as u32* FONT_SIZE_TITLE as u32)/2;
                    y1 = y1 + ACTOR_BEINE as u32;
                    draw_text_mut(&mut image, _black, x1 ,y1 , title_scale, &font, &t._elementname);


                }
            }
            parsing::parse_usecase::TypEnum::USECASE =>{
                usecase_id = usecase_id + 1;
                usecase.push(buildUsecase(usecase_id));
                for u in &mut usecase {
                    if usecase_id == u.id{
                        u.name = t._elementname.to_string();
                        u.breite = t._elementname.chars().count() as u32* FONT_SIZE as u32 +10;
                        u.hoehe = USECASEHOEHE;

                    }
                }
                
            }
            parsing::parse_usecase::TypEnum::EXTPOINT =>{
                ex_points.push(buildEx(t._behaelter.to_string(),t._elementname.to_string()));
            }
            
            parsing::parse_usecase::TypEnum::UNDEFINED =>{

            }
    
        }
    }

    
    
    for u in &mut usecase{
        for e in &mut ex_points{
            if u.name == e.name {
                u.ext_name = e.ex_point.to_string();
            }else {   
            } 
        }
    }

    let mut x = WIDTH / 2 - SUB_WIDTH / 2;
    let mut y = HEIGHT / 2 - SUB_HEIGHT / 2 + TITEL_MIN + USECASE_MIN_OBEN / 2 + USECASEHOEHE / 2;
    let mut res_flag = 0;
    let mut row = 1;
    let mut column = 0;

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
        u.rel_point_loru = ((u.center.0 as f32 - u.breite as f32 / 2.0 ,u.center.1 as f32),
                            (u.center.0 as f32,u.center.1 as f32 - u.hoehe  as f32 / 2.0),
                            (u.center.0 as f32 + u.breite as f32 / 2.0 ,u.center.1 as f32),
                            (u.center.0 as f32,u.center.1 as f32 + u.hoehe as f32 / 2.0));
        u.row = row;
        column = column +1;
        u.column = u.column;
        if u.ext_name.is_empty() {
            draw_hollow_ellipse_mut(&mut image, u.center, u.breite as i32 / 2 , u.hoehe as i32 /2 , _black);
            draw_text_mut(&mut image, _black, x - (u.name.chars().count() as u32 * FONT_SIZE as u32 /2), y ,scale, &font, & u.name);
        }else {
            let mut breite: f32 = u.breite as f32;
            let mut hoehe: f32 = u.hoehe as f32 * 2.0;
            if breite < u.ext_name.chars().count() as f32 * FONT_SIZE as f32 {
                breite = u.ext_name.chars().count() as f32 * FONT_SIZE as f32 
            }
            draw_hollow_ellipse_mut(&mut image, u.center, breite as i32 / 2 , hoehe as i32 /2 , _black);
            draw_line_segment_mut(&mut image, (u.center.0 as f32 - breite/2.0 , u.center.1 as f32), (u.center.0 as f32 + breite/2.0 , u.center.1 as f32), _black);
            draw_text_mut(&mut image, _black, x - (u.name.chars().count() as u32 * FONT_SIZE as u32 /2), y -10 ,scale, &font, & u.name);
            draw_text_mut(&mut image, _black, x - (u.ext_name.chars().count() as u32 * FONT_SIZE as u32 /2), y +10 ,scale, &font, & u.ext_name);
        }
        x = x + u.breite / 2;
    }

    for b in beziehungen{
        let mut from: (f32,f32) = (1.0,1.0);
        let mut to: (f32,f32) = (1.0,1.0);
        let mut temp_point_from: ((f32,f32),(f32,f32),(f32,f32),(f32,f32)) = ((1.0,1.0),(1.0,1.0),(1.0,1.0),(1.0,1.0));
        let mut temp_point_to: ((f32,f32),(f32,f32),(f32,f32),(f32,f32)) = ((1.0,1.0),(1.0,1.0),(1.0,1.0),(1.0,1.0));
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
                
                draw_line_segment_mut(&mut image, from, to, _black);
            }
            parsing::parse_usecase::Beziehungstyp::GENERALIZATION =>{

            }
            parsing::parse_usecase::Beziehungstyp::INCLUDE =>{
                draw_line_segment_mut(&mut image, from, to, _black);
                draw_line_segment_mut(&mut image, (to.0 -10.0 , to.1 +10.0) , to, _black);
                draw_line_segment_mut(&mut image, (to.0 -10.0 , to.1 -10.0) , to, _black);
            }
            parsing::parse_usecase::Beziehungstyp::EXTEND =>{
                
                draw_line_segment_mut(&mut image, from, to, _black);
                draw_line_segment_mut(&mut image, (to.0 -10.0 , to.1 +10.0) , to, _black);
                draw_line_segment_mut(&mut image, (to.0 -10.0 , to.1 -10.0) , to, _black);
                draw_line_segment_mut(&mut image, (to.0 -10.0 , to.1 -10.0) , (to.0 -10.0 , to.1 +10.0), _black);
            }
            parsing::parse_usecase::Beziehungstyp::EXTENDS =>{
                
                draw_line_segment_mut(&mut image, from, to, _black);
                draw_line_segment_mut(&mut image, (to.0 -10.0 , to.1 +10.0) , to, _black);
                draw_line_segment_mut(&mut image, (to.0 -10.0 , to.1 -10.0) , to, _black);
                draw_line_segment_mut(&mut image, (to.0 -10.0 , to.1 -10.0) , (to.0 -10.0 , to.1 +10.0), _black);
            }
            parsing::parse_usecase::Beziehungstyp::UNDEFINED =>{

            }
        }
    }
    




    image.save("usecase.png").unwrap();
    print!("PNG erstellt");
}




fn creat_png() ->RgbImage{
    let white = Rgb ([255u8, 255u8, 255u8]);
    let mut image = RgbImage::new(WIDTH, HEIGHT);
    draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(WIDTH, HEIGHT), white);  
    image
}
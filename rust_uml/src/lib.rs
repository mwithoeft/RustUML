#![recursion_limit="1024"]
#[macro_use]
extern crate yew;
extern crate stdweb;
extern crate regex;
extern crate rusttype;
extern crate svg;

use yew::prelude::*;
use stdweb::*;
use std::boxed::Box;

use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;
use svg::node::element::Circle;

mod parsing;

enum FormOption {
    CLASS, USECASE, ACTION, SEQUENCE, STATE, COMPONENT, PACKAGE, DEPLOYMENT, OBJECT
}

pub struct Model {
    text_area: String,
    select: FormOption,
}


pub enum Msg {
    GotInput(String),
    Clicked,
    SelectType,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            text_area: "".into(),
            select: FormOption::CLASS,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotInput(new_value) => {
                self.text_area = new_value;
            }
            Msg::Clicked => {
                let mut klassen: Vec<parsing::parse_class::Klasse> = Vec::new();
                let mut beziehungen_class: Vec<parsing::parse_class::Beziehung> = Vec::new();

                let mut typen: Vec<parsing::parse_usecase::Typ> = Vec::new();
                let mut beziehungen_usecase: Vec<parsing::parse_usecase::Beziehung> = Vec::new();
                let mut vektor : Vec<&'static str> = edit_string(&self.text_area);

                match self.select {
	                FormOption::CLASS => {
                        js!(console.log("Parse Class!"));  
                        //DATEIEN NICHT MEHR VORHANDEN - NUR ZUR KENNZEICHNUNG //parsing::parse_class::parse(&mut vektor, &mut klassen, &mut beziehungen_class);
                        //DATEIEN NICHT MEHR VORHANDEN - NUR ZUR KENNZEICHNUNG //build_class_diagram::build_klassendiagramm(&mut klassen, &mut beziehungen_class);
                        let mut svg : String = test_svg();
                        svg = edit_svg(&svg);
                        js!(document.getElementById("bild").style.backgroundImage = @{svg});
                    }

	                FormOption::USECASE => {
                        js!(console.log("Parse Usecase!")); 
                        
						//DATEIEN NICHT MEHR VORHANDEN - NUR ZUR KENNZEICHNUNG //parsing::parse_usecase::parse(&mut vektor, &mut typen, &mut beziehungen_usecase);
                        //DATEIEN NICHT MEHR VORHANDEN - NUR ZUR KENNZEICHNUNG //build_usecase_diagram::build_usecase_diagramm(&mut typen, &mut beziehungen_usecase);
                    }
        
                    FormOption::ACTION => { js!(console.log("Parse Action!"));  }
                    FormOption::SEQUENCE => { js!(console.log("Parse Sequence!"));  }
                    FormOption::STATE => { js!(console.log("Parse State!"));  }
                    FormOption::COMPONENT => { js!(console.log("Parse Component!"));  }
                    FormOption::PACKAGE => { js!(console.log("Parse Package!"));  }
                    FormOption::DEPLOYMENT => { js!(console.log("Parse Deployment!"));  }
                    FormOption::OBJECT => { js!(console.log("Parse Object!"));  }
                }
            }

            Msg::SelectType => {
                let s : String = get_type();
                if s == "class" {self.select = FormOption::CLASS;}
                else if s == "usecase" {self.select = FormOption::USECASE;}
                else if s == "action" {self.select = FormOption::ACTION;}
                else if s == "sequence" {self.select = FormOption::SEQUENCE;}
                else if s == "state" {self.select = FormOption::STATE;}
                else if s == "component" {self.select = FormOption::COMPONENT;}
                else if s == "package" {self.select = FormOption::PACKAGE;}
                else if s == "deployment" {self.select = FormOption::DEPLOYMENT;}
                else if s == "object" {self.select = FormOption::OBJECT;}
                js!(console.log("TypeSelected: " + @{s}));             
            }

        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <section>
            <div>
                <textarea rows=5,
                    value=&self.text_area,
                    oninput=|e| Msg::GotInput(e.value),
                    placeholder="Hier können Sie ihre Spezifikation eintragen",>
                </textarea>
                <div id="bild",>
                </div>
                <div>
                    <select id="selectBox", onchange=|_| Msg::SelectType,>
                        <option value="class",>{"Klassendiagramm"}</option>
                        <option value="usecase",>{"Anwendungsfalldiagramm"}</option>
                        <option value="action",>{"Aktivitätsdiagramm"}</option>
                        <option value="sequence",>{"Sequenzdiagramm"}</option>
                        <option value="state",>{"Zustandsdiagramm"}</option>
                        <option value="component",>{"Komponentendiagramm"}</option>
                        <option value="package",>{"Paketdiagramm"}</option>
                        <option value="deployment",>{"Verteilungsdiagramm"}</option>
                        <option value="object",>{"Objektdiagramm"}</option>
                    </select>
                    <button onclick=|_| Msg::Clicked,>{ "Jetzt Parsen!" }</button>
                </div>
            </div>
            </section>
        }
    }
}

fn get_type() -> String {
    (js!{
        var selectBox = document.getElementById("selectBox");
        var selectedValue = selectBox.options[selectBox.selectedIndex].value;
        return selectedValue}).into_string().unwrap()
}

fn edit_string(t: &str) -> Vec<&'static str> {
    let mut s = String::from(t);
    s = s.replace(" ", "");
    s = s.replace("\"\r\n", "\"|");
    s = s.replace("\r\n", "");

    let split = s.split("|");
    let mut v: Vec<&'static str> = Vec::new();

    for s in split {
        let p: String = s.to_owned();
        let s_slice: &'static str = Box::leak(p.into_boxed_str());

        v.push(s_slice);
    }
    v

}

fn edit_svg(s: &str) -> String {
    let mut t : String = String::from(s);
    t = t.replace("%", "%25");
    t = t.replace(" ", "%20");
    t = t.replace("!", "%21");
    t = t.replace("\"", "%22");
    t = t.replace("#", "%23");
    t = t.replace("$", "%24");
    t = t.replace("&", "%26");
    t = t.replace("'", "%27");
    t = t.replace("(", "%28");
    t = t.replace(")", "%29");
    t = t.replace("*", "%2A");
    t = t.replace("+", "%2B");
    t = t.replace(",", "%2C");
    t = t.replace("-", "%2D");
    t = t.replace(".", "%2E");
    t = t.replace("/", "%2F");

    
    t = t.replace(":", "%3A");
    t = t.replace(";", "%3B");
    t = t.replace("<", "%3C");
    t = t.replace("=", "%3D");
    t = t.replace(">", "%3E");
    t = t.replace("?", "%3F");
    t = t.replace("@", "%40");

    t = t.replace("[", "%5B");
    t = t.replace("\\", "%5C");
    t = t.replace("]", "%5D");
    t = t.replace("^", "%5D");
    t = t.replace("_", "%5F");
    t = t.replace("`", "%60");

    t = t.replace("{", "%7B");
    t = t.replace("|", "%7C");
    t = t.replace("}", "%7D");
    t = t.replace("~", "%7E");
    t = t.replace("\n", "");

    let mut a : String = String::from("url(\"data:image/svg+xml,");
    a.push_str(&t);
    a.push_str("\")");

    a
}

fn test_svg() -> String {

    let data = Data::new()
        .move_to((10, 10))
        .line_by((0, 50))
        .line_by((50, 0))
        .line_by((0, -50))
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "red")
        .set("stroke-width", 3)
        .set("d", data);

    let cir = Circle::new()
        .set("fill", "yellow")
        .set("stroke", "green")
        .set("stroke-width", 3)
        .set("cx", 20)
        .set("cy", 20)
        .set("r", 10);



    let document = Document::new()
        .set("viewBox", (0, 0, 70, 70))
        .add(path)
        .add(cir);
	

    document.to_string()
	

}
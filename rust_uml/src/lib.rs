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

mod build_class_svg;
mod parsing;
mod svglib;

enum FormOption {
    CLASS, USECASE, ACTION, SEQUENCE, STATE, COMPONENT, PACKAGE, DEPLOYMENT, OBJECT
}

pub struct Model {
    text_area: String,
    select: FormOption,
}


pub enum Msg {
    GotInput(String),
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

                let mut klassen: Vec<parsing::parse_class::Klasse> = Vec::new();
                let mut beziehungen_class: Vec<parsing::parse_class::Beziehung> = Vec::new();

                let mut typen: Vec<parsing::parse_usecase::Typ> = Vec::new();
                let mut beziehungen_usecase: Vec<parsing::parse_usecase::Beziehung> = Vec::new();
                let mut vektor : Vec<&'static str> = edit_string(&self.text_area);

                match self.select {
	                FormOption::CLASS => {
                        js!(console.log("Parse Class!"));  
                        parsing::parse_class::parse(&mut vektor, &mut klassen, &mut beziehungen_class);

                        let mut svg : String = build_class_svg::build_klassendiagramm(&mut klassen, &mut beziehungen_class).to_string();

                        js!{
                            document.getElementById("bild").innerHTML = @{svg};
                            svgPanZoom("#zoom", {
                                panEnabled: true, 
                                controlIconsEnabled: false, 
                                zoomEnabled: true, 
                                dblClickZoomEnabled: true, 
                                mouseWheelZoomEnabled: true, 
                                preventMouseEventsDefault: true, 
                                zoomScaleSensitivity: 0.2, 
                                minZoom: 0.5, 
                                maxZoom: 10,
                                fit: true, 
                                contain: true,
                                center: false, 
                                refreshRate: "auto"
                            });
                        }
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
            }

        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div id="mantel",>
                <div id="bild",></div>
                <div id="textfeld",>
                    <textarea rows=5,
                        value=&self.text_area,
                        oninput=|e| Msg::GotInput(e.value),
                        placeholder="Hier können Sie ihre Spezifikation eintragen",>
                    </textarea>
                </div>
                <div id="buttons",>
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
                </div>
            </div>
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
    s = s.replace("\"\n", "\"|");
    s = s.replace("\n", "");

    let split = s.split("|");
    let mut v: Vec<&'static str> = Vec::new();

    for s in split {
        let p: String = s.to_owned();
        let s_slice: &'static str = Box::leak(p.into_boxed_str());

        v.push(s_slice);
    }
    v

}

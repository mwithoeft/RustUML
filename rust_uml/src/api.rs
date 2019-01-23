pub use parsing;
use api_yew;
use api_htmlfile;

pub struct Api {
    _eingabe: Eingaben,
    _ausgabe: Ausgaben,
    _class_klassen: Vec<parsing::parse_class::Klasse>,
    _class_beziehungen: Vec<parsing::parse_class::Beziehung>,
    _usecase_typen: Vec<parsing::parse_usecase::Typ>,
    _usecase_beziehungen: Vec<parsing::parse_usecase::Beziehung>
}

impl Api {
    pub fn start(&self) {
        self.read();
    }

    fn read(&self) {
        match self._eingabe {
            Eingaben::WEBTEXT => {
                api_yew::run_yew(false);
            }
            Eingaben::DOCSIFY => {
                api_yew::run_yew(true);
            }
            Eingaben::HTMLFILE => {
                api_htmlfile::run_yew();
            }
            Eingaben::TEXTFILE => {}
            Eingaben::VOICE => {}
        }
    }
    fn parse(&self) {

    }
    fn write (&self) {
        match self._ausgabe {
            Ausgaben::SVGWEB => {}
            Ausgaben::PNGFILE => {}
        }
    }
}

pub fn build_api(_eingabe: Eingaben, _ausgabe: Ausgaben) -> Api {
    Api {
        _eingabe,
        _ausgabe,
        _class_klassen: Vec::new(),
        _class_beziehungen: Vec::new(),
        _usecase_typen: Vec::new(),
        _usecase_beziehungen: Vec::new()
    }
}

pub enum Eingaben {
    WEBTEXT,
    DOCSIFY,
    TEXTFILE,
    VOICE,
    HTMLFILE
}

pub enum Ausgaben {
    SVGWEB,
    PNGFILE
}
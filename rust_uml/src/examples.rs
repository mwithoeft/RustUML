/*Beispieleingaben*/
pub static CLASS_EXAMPLE: &str = "type \"classdiagram:Firma\"

classname \"Kunde\"
attribute \"+, kundenNr, String\"

classname \"Einkauf\"
attribute \"+, datum, Date\"
attribute \"+, bestellID, String\"
method \"+, bestellen, boolean\"
method \"+, artikelHinzufuegen, void, artikel:Artikel\"

classname \"Lagerbestand\"
keywords \"interface\"
method \"#, istAuslieferbar, boolean\"
method \"#, anzahl, integer, Object\"

classname \"Artikel\"
attribute \"-, bezeichnung, String\"
attribute \"-, preis, integer\"
method \"#, istAuslieferbar, boolean\"
method \"#, anzahl, integer, Artikel\"

classname \"Kategorie\"
attribute \"-, bezeichnung, String\"

classname \"Person\"
property \"abstract\"
attribute \"-, name, String\"
attribute \"-, vorname, String\"

classname \"Hand\"
attribute \"-, anzahlFinger, String\"
method \"-, fingerBewegen, void\"

relationship \"association, Kunde, false, 1, Einkauf, false, 0..*, -, -, -\"
relationship \"extends, Kunde, false, *, Person, true, *, -, -, -\"
relationship \"aggregation, Einkauf, true, 0..*, Artikel, false, 1..*, beinhaltet, anzahl, ware\"
relationship \"implements, Artikel, false, -, Lagerbestand, true, -, hat, Anzahl, Ware\"
relationship \"aggregation, Kategorie, true, 1, Artikel, false, 1..*, zählt, Genre, Ware\"
relationship \"composition, Person, true, 1, Hand, false, 0..2, -, -, -\"
relationship \"dependency, Kunde, false, -, Lagerbestand, true, -, -, -,-\"
";


pub static USECASE_EXAMPLE: &str = "type \"usecase diagram:CD\"

elementtype \"subject, Multimediasystem\"

elementtype \"actor, Benutzer1\"
elementtype \"actor, Benutzer2\"
elementtype \"usecase, CD-brennen\"
elementtype \"usecase, CD-erstellen\"
elementtype \"usecase, CD-beschriften\"
elementtype \"usecase, Booklet-erstellen\"
elementtype \"extpoint, Beschriftung, CD-erstellen\"

relationship \"association, Benutzer1, -, CD-erstellen, -\"
relationship \"association, Benutzer1, -, CD-brennen, -\"
relationship \"association, Benutzer1, -, Booklet-erstellen, -\"
relationship \"association, Benutzer2, -, CD-beschriften, -\"
relationship \"include, CD-erstellen, -, CD-brennen, -\"
relationship \"include, CD-erstellen, -, Booklet-erstellen, -\"
relationship \"extend, CD-beschriften, -, CD-erstellen, -\"
";

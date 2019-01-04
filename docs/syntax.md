# Syntax der UML Diagramme

## Allgemeines zur Syntax

> Im Folgenden soll die verwendete Syntax zur Darstellung und zum Parsen der UML-Diagramme erläutert werden.
> Diese Syntax muss verwendet werden, damit die Erstellung bzw. Generierung eines UML-Diagramms aus der Eingabe heraus erfolgreich verläuft.
> Zuerst soll im Allgemeinen dargestellt werden, wie der Diagrammtyp in der Syntax festzulegen ist. Danach wird auf die einzelnen UML-Diagramme im Detail eingegangen, da sich die Syntax der verschiedenen Diagrammtypen unterscheidet.
> Alle Keywörter beginnen immer mit einem **kleinen** Buchstaben! Die **Rechtschreibung** ist unbedingt zu beachten, damit die Eingabe korrekt verarbeitet werden kann! Nach der Eingabe einer Zeile folgt ein **Zeilenumbruch** als Trennung der einzelnen Eingaben.  

## Festlegung des Diagrammtyps und des Namens

<!-- tabs:start -->

#### ** Website **

> Am Anfang einer jeden Diagrammdefinition steht die festlegung des Diagrammtyps. Soll beispielsweise in Klassendiagramm definiert werden erfolgt dies mit Hilfe der Zeile `type "classdiagram"`

So lassen sich alle Diagrammtypen deklarieren:
- Klassendiagramm: `type "classdiagram"`
- Anwendungsfalldiagramm: `type "usecasediagram"`
- Akitivitätsdiagramm: `type "actiondiagram"`
- Sequenzdiagramm: `type "sequencediagram"`
- Zustandsdiagramm: `type "statediagram"`
- Komponentendiagramm: `type "componentdiagram"`
- Paketdiagramm: `type "packagediagram"`
- Verteilungsdiagramm: `type "deploymentdiagram"`
- Objektdiagramm: `type "objectdiagram"`

Zusätzlich <u>kann</u> dem Diagramm selbst noch ein Name gegeben werden, welche über einen Doppelpunkt `:` angehängt werden kann.

*Beispiel:* `type "classdiagram:Firma"`

#### ** HTML-Standalone **

> Die Festlegung des Klassendiagramms in der HTML-Standalone-Anwendung erfolgt mit Hilfe des **HTML-Universalattributs** `data-type="TYP"`.  

- Klassendiagramm: `"class"`
- Anwendungsfalldiagramm: `"usecase"`
- Akitivitätsdiagramm: `"action"`
- Sequenzdiagramm: `"sequence"`
- Zustandsdiagramm: `"state"`
- Komponentendiagramm: `"component"`
- Paketdiagramm: `"package"`
- Verteilungsdiagramm: `"deployment"`
- Objektdiagramm: `"object"`

*Beispiele:*  
`data-type="class"`  
`data-type="deployment"`

> Weitere Informationen zum Umgang mit der HTML-Standalone-Anwendung finden Sie unter **HTML-Anwendung**.

#### ** Markdown-Standalone **
> Aktuell steht die Markdown-Version nicht zur Verfügung.


<!-- tabs:end -->


## Klassendiagramm

> Im Folgenden werden die einzelnen Möglichkeiten zur Eingabe eines Klassendiagramms erläutert. Dabei steht zuerst immer eine abstrakte Darstellung der Eingabe, gefolgt von einem Beispiel. Danach werden alle weiteren möglichen Schlüsselwörter aufgelistet.

**Klassenname:**  
`classname "Name der Klasse"`  
*Beispiel:* `classname "Ball"`  

**Klasseneingenschaft:**  
`property "Klasseneingenschaft"`  
*Beispiel:* `property "abstract"`  

**Schlüsselwörter (im UML-Diagramm):**  
`keywords "Schlüsselwörter"`  
*Beispiele:*  
`keywords "interface, gui"`  
`keywords "main"`  

**Attribute:**  
`attribute "Modifikator, Attributentyp1(optional), Attributentyp2(optional), Name des Attributes, Datentyp, Wert(optional)"`  
*Beispiele:*  
`attribute "+, size, int, 5"`  
`attribute "-, static, final, length, float"`  
*Modifikatoren:* `#`, `-`, `+`, `~`  
*Attributstypen:* `final`, `static`  
*Datentypen:* `int`, `float`, `String`, ...

**Methoden:**  
> Soll der Konstruktor definiert werden, so ist der Methodenname der Name der Klasse! 

`method "Modifikator, Methodentyp1(optional), Methodentyp2(optional), Name, Datentyp der Rückgabe, parameter1:Datentyp, parameter2:Datentyp, ..."`  
*Beispiele:*   
`method "-, length, int, xwert:int, ywert:int"`  
`method "#, final, static, height, double, xwert:int, ywert:int"`  
*Modifikatoren:* `#`, `-`, `+`, `~`  
*Methodentypen:* `final`, `static`  
*Datentypen:* `int`, `float`, `String`, ...

> Nun kann eine neue Klasse definiert werden, indem eine neue Zeile wieder mit dem Schlüsselword `classname` beginnt.
> Nach dem Definieren der einzelnen Klassen können nun noch die Beziehung der einzelnen Klassen untereinander definiert werden:  

**Beziehungen:**  
`relationship "Beziehungstyp, 'von Klasse' Name, Pfeil auf 'von Klasse' (true/false), Multiplizität 'von Klasse', 'zu Klasse' Name, Pfeil auf 'zu Klasse' (true/false)  Multiplizität 'zu Klasse'"`  
*Beispiele:*  
`relationship "extends, Vater, false, -, Kind, true, -"`  
`relationship "association, Gebäude, true, 1, Raum, false, 0..*"`  
*Beziehungstypen:* `extends`, `implements`, `association`, `aggregation`, `composition`, `dependency`

> Soll *keine Multiplizität* angegeben werden schreibt man an der Stelle einfach ein `-`.

## Anwendungsfalldiagramm

> Im Folgenden werden die einzelnen Möglichkeiten zur Eingabe eines Anwendungsfalldiagramms erläutert. Dabei steht zuerst immer eine abstrakte Darstellung der Eingabe, gefolgt von einem Beispiel. Danach werden alle weiteren möglichen Schlüsselwörter aufgelistet. **Leerzeichen** werden beim Parsen **komplett entfernt**. Möchte man ein Leerzeichen zwischen zwei Worten haben trennt man diese Worte mit einem `-`(Bindestrich!). Es müssen zudem erst alle Elementtypen angegeben werden, bevor die Beziehungen geschrieben werden.

**Elementtypen:**  
`elementtype "Elementtyp, Elementname , Behälter(Optional)"`  
*Beispiel:* `elementtype "actor, sender"`  
*Schlüsselwörter der Typen:* `subject`, `actor`, `usecase`, `extpoint`

**Beziehungen:**  
`relationship "Beziehungstyp, 'von Element' Name, 'von Element' Multiplizizät, 'von Element' Name2(Optional),..., 'zu Element' Name, 'zu Element' Multiplizität, Inhalt der Notiz/Bedingung(optional)"`   
*Beziehungstypen:* `association`, `generalization`, `include`, `extend`, `extends`  
> Soll *keine Multiplizität* angegeben werden schreibt man an der Stelle einfach ein `-`.  
> **Unterschied von extend und extends:** `extends` wird geschrieben wenn ein Akteur von einem anderen erbt. Extended ein Usecase einen anderen schreibt man `extend`! Bei der Eingabe ist auf diesen Unterschied zu achten, weil er sehr bedeutsam ist!

> Es können nur mehrere 'von Elemente' angegeben werden, wenn es sich um eine Generalisierung handelt! Hierbei ist wiederum keine Angabe der Multiplizität möglich. Beziehungen, bei denen eine Angabe der Multiplizität möglich ist, können nicht mit mehreren 'von Elementen' ausgestattet werden, da dies nur für eine Generalisierung möglich ist!  

*Beispiel Assoziation/Andere:* `relationship "association, sender, *, empfänger, -"`  
*Beispiel Generalisierung:* `relationship "generalization, sender1, sender2, hauptsender"`

## Aktivitätsdiagramm

> Im Folgenden werden die einzelnen Möglichkeiten zur Eingabe eines Aktivitätsdiagramms erläutert. Dabei steht zuerst immer eine abstrakte Darstellung der Eingabe, gefolgt von einem Beispiel. Danach werden alle weiteren möglichen Schlüsselwörter aufgelistet.

**Paketsname**:  
`package "name"`  
*Beispiel:* `package "Spaghetti kochen"`  

**Elementtypen:**  
`elementtype "Elementtyp, name"`  
*Beispiel:* `elementtype "state, Spaghetti roh"`  
*Schlüsselwörter der Typen:* `state`, `receivesignal`, `sendsignal`, `timesignal`, `dataobject`

**Beziehungen:**  
`relationship "erstes Element, zweites Element"`  
*Beispiele:*  
`relationship "startpoint, Anrede"`  
`relationship "Anrede, Land"`  
`relationship "Land, endpoint"`  
`relationship "startpoint, endpoint"`  
*Schlüsselwörter:* `startpoint`, `endpoint`	

## Sequenzdiagramm

> Im Folgenden werden die einzelnen Möglichkeiten zur Eingabe eines Sequenzdiagramms erläutert. Dabei steht zuerst immer eine abstrakte Darstellung der Eingabe, gefolgt von einem Beispiel. Danach werden alle weiteren möglichen Schlüsselwörter aufgelistet.

**Tokens**:  
`objectToken "Objektname, Klassenname"`  
*Beispiel:* `objectToken "Mann, Mensch"`  

**Beziehungen:**  
`relationship "Beziehungstyp, erstes Token , zweites Token, name"`  
*Beispiel:* `relationship "synchronic, Patient, Doktor, Rezept"`  
*Beziehungstypen*: `synchronous`, `asynchronous`, `return`, `undetermined`  

**Löschen:**  
`delete "Löscher, Gelöschter"`  
*Beispiel:* `delete "Kunde, PIN"`  

## Zustandsdiagramm

> Im Folgenden werden die einzelnen Möglichkeiten zur Eingabe eines Zustandsdiagramms erläutert. Dabei steht zuerst immer eine abstrakte Darstellung der Eingabe, gefolgt von einem Beispiel. Danach werden alle weiteren möglichen Schlüsselwörter aufgelistet.

**Zustände:**  
`state "name"`  
*Beispiel:* `state "Punkte addiert"`  

**Events:**  
`event "Zustand 1, Zustand 2, name, condition(Optional)"`  
*Beispiele:*  
`event "Note veröffentlicht, Studienschein ausgestellt, Note ermittelt"`  
`event "Note veröffentlicht, endpoint, Note ermittelt"`  
*Schlüsselwörter:* `startpoint`, `endpoint`	 

**Choices:**  
`choice "condition, if state,else state"`  
*Beispiel:* `choice "Punkte > 100, Klausur bestanden, Klausur nicht bestanden"`  

## Komponentendiagramm

> Im Folgenden werden die einzelnen Möglichkeiten zur Eingabe eines Komponentendiagramms erläutert. Dabei steht zuerst immer eine abstrakte Darstellung der Eingabe, gefolgt von einem Beispiel. Danach werden alle weiteren möglichen Schlüsselwörter aufgelistet.

**Komponenten:**  
`component "Komponentenname, Elternkomponente(Optional), Interfacename(Optional)"`  
*Beispiele:*  
`component "Webdienst, komponente"` *Komponente ist hier ein Interface!*  
`component "Mailserver, Webdienst, komponente"` *Komponente ist hier ein Interface, aber Webdienst die Elternkomponente!*  
`component "Testserver, Webdienst"` *Webdienst ist hier die Elternkomponente und kein Interface!*
> Der Interface name darf nicht der Name einer schon vorhandenen Komponente sein, weil die erzeugte Komponente diese sonst als Elternkomponente ansieht!

**Ports:**  
`port "Porttyp, Portname, Kommentar(Optional)"`  
*Beispiel:* `port "required, PaymentAuthorization"`  
*Schlüsselwörter der Porttypen:* `required`, `provided`	

**Beziehungen:**  
`relationship "Beziehungstyp, 'von Komponente/Port', 'von Komponente/Port' Multiplizizät, 'zu Komponente/Port', ' zu Komponente/Port' Multiplizizät, Kommentar(Optional)"`  
*Beispiele:*  
`relationship "dependency, Webdienst, 1, PaymentAutorization, 1..*, Validierung"`  
`relationship "composition, Testserver, -, MailserverSC, -"`  
*Schlüsselwörter der Beziehungstypen:* `dependency`, `generalization`, `partassemly`, `composition`
> Soll *keine Multiplizität* angegeben werden schreibt man an der Stelle einfach ein `-`. Möchte man sich mit der Schnittstelle einer Komponente vervinden schreibt man hinter den Namen noch `SC`.

## Paketdiagramm

> Im Folgenden werden die einzelnen Möglichkeiten zur Eingabe eines Paketdiagramms erläutert. Dabei steht zuerst immer eine abstrakte Darstellung der Eingabe, gefolgt von einem Beispiel. Danach werden alle weiteren möglichen Schlüsselwörter aufgelistet.

**Pakete:**  
`package "Paketname, Elternpaket, Paketeigenschaft, Paketattribut1(Optional), Paketattribut2(Optional), ..."`  
*Beispiel:* `package "ServiceProvider, -, interface, ServiceFactorie, ServiceType"`  
> Gibt es *kein Elternpaket* oder *keine Paketeigenschaft* schreibt man an der Stelle einfach ein `-`.

**Elemente:**  
`element "Modifikator, Elementname, Kommentar(Optional)"`  
*Beispiele:*  
`element "+, Katalog"`  
`element "-, Account"`  
*Modifikatoren:* `-`, `+`,   

**Beziehungen:**  
`relationship "Beziehungstyp, 'von Paket' Name, Pfeil auf 'von Paket' (true/false), Multiplizität 'von Paket', 'zu Paket' Name, Pfeil auf 'zu Paket' (true/false)  Multiplizität 'zu Paket', Kommentar(Optional)"`  
*Beispiele:*  
`relationship "access, WebApplication, false, -, Presentation, true, -"`  
`relationship "import, WebApplication, false, 1, Domain, false, 1..*"`  
*Beziehungstypen:* `required`, `import`, `access`, `merge`, `dependency`, `composition`  

> Bei *merge* gibt es die Ausnahme, dass es mehrere 'von Pakete' geben darf. Hier wird das letzte angegebene Paket als 'zu Paket' betrachtet. Die Syntax ändert sich dabei nicht, das heißt es werden pro Paket in der Reihenfolge erwartet: `'Paket' Name, Pfeil auf 'Paket' (true/false), Multiplizität 'Paket'`  

*Beispiel:* `relationship "merge, Kernel, false, -, Profiles, false, -, Constructs, true, -"`  
> Soll *keine Multiplizität* angegeben werden schreibt man an der Stelle einfach ein `-`.


## Verteilungsdiagramm

> Im Folgenden werden die einzelnen Möglichkeiten zur Eingabe eines Verteilungsdiagramms erläutert. Dabei steht zuerst immer eine abstrakte Darstellung der Eingabe, gefolgt von einem Beispiel. Danach werden alle weiteren möglichen Schlüsselwörter aufgelistet.

**System:**  
`system "Systemname, Systemattribut1(Optional), SystemAttribut2(Optional), ..."`
*Beispiel:* `system "Deployment of Components"` 

**Knoten:**  
`node "Knotentyp, Name des Knotens, Elternknoten, Knoteneigenschaft(Optional)"`  
*Beispiel:* `node "device, :Server, -"`
*Schlüsselwörter der Knotentypen:* `device`, `execution`  
> Es gibt zwei Arten von Knoten: Die "Geräte"-Knoten und die ausführenden Umgebungsknoten.  
> Gibt es *keinen Elternknoten* schreibt man an der Stelle einfach ein `-`.

**Attribute:**  
`attribute "Inhalt"`  
*Beispiel:* `attribute "session-type: Stateful"`
> Das definierte Attribut bezieht sich immer auf den zuletzt definierten Knoten!

**Beziehungen:**  
`relationship "Beziehungstyp, 'von Knoten' Name, Pfeil auf 'von Knoten' (true/false), Multiplizität 'von Knoten', 'zu Knoten' Name, Pfeil auf 'zu Knoten' (true/false)  Multiplizität 'zu Knoten', Kommentar(Optional)"`  
*Beispiele:*  
`relationship "composition, Apache, false, -, ApplicationServer, true, -"`  
`relationship "deploy, portfolio.jar, false, -, ApacheServer, true, -"`  
*Beziehungstypen:* `association`, `activity`, `generalization`, `aggregation`, `dependency`, `composition`  

> Soll *keine Multiplizität* angegeben werden schreibt man an der Stelle einfach ein `-`.
> Bei *generalization* gibt es die Ausnahme, dass es mehrere 'von Knoten' geben darf. Hier wird der letzte angegebene Knoten als 'zu Knoten' betrachtet. Die Syntax ändert sich dabei nicht, das heißt es werden pro Paket in der Reihenfolge erwartet: `'Knoten' Name, Pfeil auf 'Knoten' (true/false), Multiplizität 'Knoten'`  

*Beispiel:* `relationship "generalization, ApacheServer, false, -, WAMPServer, false, -, WebServer, true, -"`  

**Notizen:**  
`note "Bezugsknoten, Inhalt"`  
*Beispiel:* `note ":Server, Needs maintainance every now and again"`
> Hier ist aufzupassen, dass **NODE** und **NOTE** nicht verwechselt werden!


## Objektdiagramm

> Im Folgenden werden die einzelnen Möglichkeiten zur Eingabe eines Objektdiagramms erläutert. Dabei steht zuerst immer eine abstrakte Darstellung der Eingabe, gefolgt von einem Beispiel. Danach werden alle weiteren möglichen Schlüsselwörter aufgelistet.

**Objektname:**  
`objectname "Name des Objekts, Name der Klasse"`  
*Beispiel:* `objectname "Blauer Ball, Ball"`  

**Objekt-Stereotyp:**  
`objectstereotype "Stereotyp des Objekts"`  
*Beispiel:* `objectstereotype "TestKlasse"`  

**Attribute:**  
`attribute "Modifikator(optional), Vererbung?(y/n), Attributentyp1(optional),Attributentyp2(optional), Name des Attributs, Datentyp, Wert"`  
*Beispiele:*  
`attribute "+, y, size, int, 5"`  
`attribute "#, n, static, final, size, int, 10"`  
`attribute "-, y, final, size, double, 1.3"`  
*Attributstypen:* `final`, `static`  
*Modifikatoren:* `#`, `-`, `+`, `~`  

> Nun kann ein neues Objekt definiert werden, indem eine neue Zeile wieder mit dem Schlüsselword `objectname` beginnt.
> Nach dem Definieren der einzelnen Objekte können nun noch die Beziehung der einzelnen Objekte untereinander definiert werden:  

**Beziehungen:**  
`relationship "Beziehungstyp, von Objekt 1, nach Objekt 2"`  
*Beispiele:*  
`relationship "composition, Haus, Wand"`  
`relationship "association, Gebäude, Raum"`  
*Beziehungstypen:* `association`, `aggregation`, `composition`  

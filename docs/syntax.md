# Syntax der UML Diagramme

> Im Folgenden soll wird die verwendete Syntax zur Darstellung und zum Parsen der UML-Diagramme erläutert.
> Diese Syntax muss verwendet werden, damit die Erstellung bzw. Generierung eines UML-Diagramms aus der Eingabe heraus erfolgreich verläuft.
> Zuerst soll Allgemein Dargestellt werden, wie der Diagrammtyp in der Syntax festzulegen ist. Danach wird auf die einzelnen UML-Diagramme im Detail eingegangen, da sich die Syntax der verschiedenen Diagrammtypen unterscheidet.

## Festlegung des Diagrammtyps

> Als erstes erwartet der Parser die Festlegung des Diagrammtyps. Folgende Diagrammtypen stehen zur Verfügung:  
> Als erstes wird der deutsche Diagrammtyp aufgelistet, dahinter steht das Keywort, das für die Syntax entscheidend ist! Die alle Keywörter beginnen immer mit einem **kleinen** Buchstaben! Die **Rechtschreibung** ist unbedingt zu beachten, damit die Eingabe korrekt verarbeitet werden kann!  

- Klassendiagramm: `"class diagram"`
- Anwendungsfalldiagramm: `"application diagram"`
- Akitivitätsdiagramm: `"action diagram"`
- Sequenzdiagramm: `"sequence diagram"`
- Zustandsdiagramm: `"state diagram"`
- Komponentendiagramm: `"component diagram"`
- Paketdiagramm: `"package diagram"`
- Verteilungsdiagramm: `"distribution diagram"`
- Objektdiagramm: `"object diagram"`  

> Zu wissen, wie die Diagrammtypen benannt werden reicht jedoch nicht. **Zuerst** schaut sich der Parser danach um, ob der Typ des Diagramm festlegt wird. Dies geschieht mit Hilfe des Schlüsselworts `type`.
> Die Erste Zeile einer Eingabe könnte also wie im folgenden Beispiel aussehen:  

`type "class diagram"`  

> Das reicht um den Diagrammtypen festzulegen. Dabei dürfen die **Anführungszeichen** nicht vergessen werden! Nach der Eingabe einer Zeile folgt ein **Zeilenumbruch** als Trennung der einzelnen Eingaben.

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
*Beispiel:* `keywords "interface, gui"`  
*Weitere Schlüsselwörter:* `"interface"`, `"gui"`, `"main"`, ...

**Attribute:**  
`attribute "Modifikator, Attributentyp1(optional), Attributentyp2(optional), Name des Attributes, Datentyp, Wert(optional)"`  
*Beispiele:*  
`attribute "+, size, int, 5"`  
`attribute "-, static, final, length, float"`  
*Modifikatoren:* `#`, `-`, `+`, `~`  
*Attributstypen:* `final`, `static`  
*Datentypen:* `int`, `float`, `String`, ...

**Methoden:**  
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
`relationship "Beziehungstyp, von Klasse 1, nach Klasse 2, Multiplizität Klasse 1, Multiplizität Klasse 2"`  
*Beispiele:*  
`relationship "extends, Vater, Kind, -, -"`  
`relationship "association, Gebäude, Raum, 1, 0..*"`  
*Beziehungstypen:* `extends`, `implements`, `association`, `aggregation`, `composition`

> Soll *keine Multiplizität* angegeben werden schreibt man an der Stelle einfach ein `-`.

## Anwedungsfalldiagramm

> Im Folgenden werden die einzelnen Möglichkeiten zur Eingabe eines Anwendungsfalldiagramms erläutert. Dabei steht zuerst immer eine abstrakte Darstellung der Eingabe, gefolgt von einem Beispiel. Danach werden alle weiteren möglichen Schlüsselwörter aufgelistet.

**Elementtypen:**  
`elementtype "Elementtyp, Elementname , Behälter(Optional)"`  
*Beispiel*: `elementtype "actor, sender"`  
*Schlüsselwörter der Typen:* `subject`, `actor`, `usecase`, `extpoint`

**Beziehungen:**  
`relationship "Beziehungstyp, erstes Element, zweites Element, Inhalt der Notiz(optional)"`  
*Beispiel*: `relationship "association, sender, empfänger"`  
*Beziehungstypen:* `association`, `generalization`, `include`, `extend`

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
*Beispiel*: `objectToken "Mann, Mensch"`  

**Beziehungen:**  
`relationship "Beziehungstyp, erstes Token , zweites Token, name"`  
*Beispiel:* `relationship "synchronic, Patient, Doktor, Rezept"`  
*Beziehungstypen*: `synchronic`, `asynchronous`, `return`, `undetermined`  

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
## Paketdiagramm
## Verteilungsdiagramm

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

# Reguläre Ausdrücke zum Parsen der Eingabe

## Allgemeines zu den Ausdrücken

> Die regulären Ausdrücke wurden im Programm verwendet, um die Eingabe des Ntuzers auf Korrektheit zu überprüfen. Dabei prüfen diese vor allem die formale Struktur der Eingabe. Also: steht am Anfang der Zeile ein Schlüsselwort? Und: Sind die Parameter innerhalb der Anführungszeichen definiert? Im Folgenden sind die einzelnen regulären Ausdrücke spezifiziert. Dabei stehen innerhalb der Klammern in den regulären Ausdrücken immer die Paramater (bei regulären Ausdrücken auch Gruppen genannt), die dann nach einer erfolgreichen Prüfung der Eingabe verarbeitet werden können. Für die Verarbeitung der einzelnen Parameter einer Zeile werden dann keine regulären Ausdrücke verwendet, da hier eine Prüfung auf die Anzahl der Parameter und eventuell notwendige Schlüsselwörter sinnvoller erscheint.

## Diagrammtyp

> Zuerst wird die Eingabe auf die verschiedenen Diagrammtypen mit Hilfe von regulären Ausdrücken überprüft. Dabei wird unterschieden, ob die Eingabe mit eigens definiertem Diagrammnamen erfolgt ist oder nicht.

Über eine definierte Gruppe im Ausdruck kann dann der eingegebene Name ermittelt werden, wie hier am Beispiel eines Klassendiagramms:
`^type\"classdiagram:(.+?)\"`. Zuvor wurde bereits mit Hilfe von diesem Ausdruck `(^type\"classdiagram\")|(^type\"classdiagram:.+?\")` geprüft, ob es sich um ein Klassendiagramm handelt. Die Ermittlung anderer Diagrammtypen erfolgt dabei äquivalent. Es wurde nur der Diagrammtyp im RegEx angepasst.

## Klassendiagramm

**Klassenname:**  
`^classname\"([^\"]+?)\"$`  
**Klasseneigenschaft:**  
`^property\"([^\"]+?)\"$`  
**Schlüsselwörter:**  
`^keywords\"([^\"]+?)\"$`  
**Attribute:**  
`^attribute\"([^\"]+?)\"$`  
**Methoden:**  
`^method\"([^\"]+?)\"$`  
**Beziehungen:**  
`^relationship\"([^\"]+?)\"$`

> Wie hier zu sehen, unterscheiden sich die regulären Ausdrücke kaum. Das liegt daran, dass im Grunde für alle Eingaben die gleiche Syntax gilt, sich jedoch immer das Schlüsselwort zum richtigen Parsen unterscheidet. Der reguläre Ausdrück prüft also immer erst, ob die Zeile mit dem richtigen Schlüsselwort beginnt (Whitespaces sind hier auf Grund von vorheriger Verarbeitung kein Problem) und dann ob es eine Eingabe zwischen den Anführungszeichen gibt. Dabei wird auch direkt überprüft, ob zwischen den Anführungszeichen *überhaupt* etwas steht, um eine leere Eingabe schnell zu identifizieren. Die Klammern sorgen hier dafür, dass nach einer erfolgreichen Überprüfung schnell auf die Daten innerhalb der Anführungszeichen (die Parameter) zugegriffen werden kann. Eine weitere Prüfung der Parameter findet dann in einzelnen Funktionen statt, die die Eingabe in eine Struktur einlesen sollen, wenn diese korrekt ist.

## Anwendungsfalldiagramm

> Auch beim Anwendungsfalldiagramm wurden die regulären Ausdrücke angepasst. Auch hier musste jeweils nur das Schlüsselwort verändert werden.

**Elementtyp:**  
`^elementtype\"([^\"]+?)\"$`  
**Beziehungen:**  
`^relationship\"([^\"]+?)\"$`  

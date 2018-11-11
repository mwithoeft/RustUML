# Definition der Schnittstellen

## Allgemeines zu den Schnittstellen

> Schon während des Parsens wird eine Datenstruktur als Schnittstelle benötigt. In diese Datenstruktur können direkt die ausgewerteten Informationen eingespeichert werden, sodass diese nicht verloren gehen.
> Jetzt könnte sich die Frage stellen, warum denn diese Datenstrukur. Eine Schnittstelle darstellt. Die Antwort ist sehr simpel. Die Datenstruktur dient nicht nur als Speicher während dem Einlesen der Informationen.
> Mit ihr wird nach dem Parsen weitergearbeitet, um aus der aufgebauten Struktur ein visuelles Diagramm erstellen zu können. Sie wird also sowohl von dem Teil des Programms genutzt, das für das Parsen zuständig ist, als auch von dem Teil, der die Erzeugung der finalen Bilddatei regelt.
> Jetzt wurde bisher immer von **der einen** Schnittstelle und **der einen** Datenstruktur gesprochen, es gibt jedoch **mehrere**. Einen für jeden Diagrammtyp.
> Die Diagramme sind so vielfältig und unterschiedlich, dass für alle Typen des Diagramms eine Struktur bestehen muss. Die Struktur wird zusammen mit der Information, um welchen Diagrammtyp es sich handelt, übergeben. Diese soll im Folgenden mit Hilfe von Code-Snippets dargestellt und erläutert werden.
> **Eine Struktur kann und wird mehrfach erzeugt werden. Gleiche Strukturen befinden sich in einer generischen Liste.**
## Klassendiagramm

Um eine sinnvolle Struktur für das Klassendiagramm zu gewährleisten, wurden in Rust mehrere Strukturen erstellt:

#### Klasse als Hauptstruktur
```rust
pub struct Klasse {
    _name: String,
    _property: String,
    _keywords: String,
    _attribute: Vec<Attribut>,
    _methoden: Vec<Methode>
}
```

##### Attribut als Substruktur einer Klasse
```rust
struct Attribut {
    _modifikator: char,
    _final: bool,
    _static: bool,
    _name: String,
    _datentyp: String,
    _wert: String
}
```

##### Methode als Substruktur einer Klasse
```rust
struct Methode {
    _modifikator: char,
    _final: bool,
    _static: bool,
    _name: String,
    _returntyp: String,
    _parameter: Vec<String>
}
```

## Beziehungen
> Nun gilt es noch, die Beziehungen vernünftigt darzustellen. Dazu gibt es eine extra Struktur. Natürlich können auch Beziehungen mehrfach erzeugt werden und in die dafür vorgesehene generische Liste eingefügt werden.

#### Beziehung als Hauptstruktur
```rust
pub struct Beziehung {
    _beziehungstyp: Beziehungstyp,
    _von_klasse_name: String,
    _von_klasse_pfeil: bool,
    _von_klasse_mult: String,
    _zu_klasse_name: String,
    _zu_klasse_pfeil: bool,
    _zu_klasse_mult: String
}
```

##### Der Beziehungstyp als ENUM, in der die verschiedene Beziehungstypen definiert werden
```rust
enum Beziehungstyp {
    EXTENDS,
    IMPLEMENTS,
    ASSOCIATION,
    DEPENDENCY,
    AGGREGATION,
    COMPOSITION
}
```


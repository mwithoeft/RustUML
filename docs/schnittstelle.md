# Definition der Schnittstellen

## Allgemeines zu den Schnittstellen

> Schon während des Parsens wird eine Datenstruktur als Schnittstelle benötigt. In diese Datenstruktur können direkt die ausgewerteten Informationen eingespeichert werden, sodass diese nicht verloren gehen.
> Jetzt könnte sich die Frage stellen, warum denn diese Datenstrukur. Eine Schnittstelle darstellt. Die Antwort ist sehr simpel. Die Datenstruktur dient nicht nur als Speicher während dem Einlesen der Informationen.
> Mit ihr wird nach dem Parsen weitergearbeitet, um aus der aufgebauten Struktur ein visuelles Diagramm erstellen zu können. Sie wird also sowohl von dem Teil des Programms genutzt, das für das Parsen zuständig ist, als auch von dem Teil, der die Erzeugung der finalen Bilddatei regelt.
> Jetzt wurde bisher immer von **der einen** Schnittstelle und **der einen** Datenstruktur gesprochen, es gibt jedoch **mehrere**. Einen für jeden Diagrammtyp.
> Die Diagramme sind so vielfältig und unterschiedlich, dass für alle Typen des Diagramms eine Struktur bestehen muss. Die Struktur wird zusammen mit der Information, um welchen Diagrammtyp es sich handelt, übergeben. Diese soll im Folgenden mit Hilfe von Code-Snippets dargestellt und erläutert werden.

## Klassendiagramm


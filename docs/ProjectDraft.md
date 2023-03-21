Aus meiner Sicht müssten wir zunächst ein paar Dinge klären:

- Welche Art der Datenbank bevorzugen wir (ich würde Postgresql vorschlagen)
- Wie sieht das Backend aus (aus meiner Sicht macht Python Django Sinn)
- Wie implementieren wir das Frontend (Ein Framework wie Vue und anschließend TypeScript oder JavaScript)

Mein Vorschlag:

Wenn wir auf Django zurückgreifen, brauchen wir uns über die SQL-Implementierung kaum gedanken machen. Django nutzt Modelle, die über einfache `migrations` in der Datenbank übernommen werden. Mit relativ einfacher Python Syntax können wir dann auf den Inhalt der Datenbank zugreifen. Ein Modell wäre in etwa eine Tabelle in deinem Datanbank-Diagramm. Die Funktionalität implementieren wir im Python-Teil.
Die Datenbank selbst ist relativ einfach. Sie muss einfach nur erreichbar sein, ansonsten kümmert sich Django komplett um die Datenbank.
Die Kommunikation mit dem Backend läuft über die Django-REST-API. So können wir über einfache Requests auf den Inhalt der Datenbank zugreifen und im Frontend visualisieren. Über die REST-API haben wir auch eine Schnittstelle, die evtl. auch von außen erreichbar ist und die Kommunikation mit weiterer Software in Zukunft ermöglicht.

Soll jeder Endanwender eine Plattform für sich nutzen, oder soll eine gemeinsame Datenbank geschaffen werden? Also brauchen wir einen Login, da alles nicht irgendwo auf einem Heimserver läuft, sondern im WWW?

Für die Entwicklung würde ich auf Docker (bzw. Docker-Compose) zurückgreifen, dawir so einfach sicherstellen können, dass wir die gleichen Voraussetzungen haben und nicht durch Systemeinstellungen, etc. uns Probleme einhandeln.

Was denkst du dazu?

Generell wäre ich für Englisch als Sprache im Quellcode, das macht es uns und evtl anderen, die mithelfen möchten/können oder einfach nur die Sachen selbst nutzen möchten, das Leben einfacher.

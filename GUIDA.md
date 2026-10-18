# Confini ISTAT: guida all'uso

*[English version in the README](README.md#use)*

Il plugin per [RustGIS](https://rustgis.app) aggiunge a RustGIS i confini amministrativi italiani pubblicati da ISTAT, serviti dall'[API onData](https://www.confini-amministrativi.it/).

## Installazione

Serve RustGIS 0.2.1 o successivo. Scarica il file `.rgplugin` dalla pagina [Releases](https://github.com/pigreco/rustgis-istat-boundaries/releases) e installalo da **Plugins › Plugin Manager**. Il plugin chiede i permessi `edit` (aggiungere livelli) e `network` (far scaricare i dati a RustGIS).

## Pulsanti

I pulsanti sono nella scheda **Plugins**.

- **Aggiungi Ripartizioni**: le 5 ripartizioni geografiche (Nord-ovest, Nord-est, Centro, Sud, Isole).
- **Aggiungi Regioni**: le 20 regioni.
- **Aggiungi Province**: le province e le città metropolitane.
- **Aggiungi Comuni**: tutti i comuni d'Italia (file grande, qualche secondo in più).
- **URL confini**: mostra nella barra di stato l'indirizzo dei dati delle regioni.

I pulsanti usano i confini al 1° gennaio 2026. RustGIS scarica il GeoPackage e aggiunge i livelli al progetto, ma non salva il file su disco: per averlo, seleziona il livello e usa **Publish › Output › Export Features**, che ti fa scegliere cartella e formato.

## Altre date e altre aree: lo strumento

Lo strumento **URL confini ISTAT** (Geoprocessing, gruppo *Confini ISTAT*) ha un form con data di riferimento (dal 1991 al 2026), livello, regione, provincia e formato. Costruisce l'indirizzo dei dati e un comando `curl` per scaricarli. Lo strumento non può aggiungere livelli: scarica il file e aprilo con **Add Data**, oppure usa l'indirizzo con **Add Data › From URL**.

## Da script

I comandi accettano parametri, per esempio i comuni della Basilicata al 2020:

```
plugin:istat-boundaries/download-comuni={"date": "20200101", "region": "17"}
```

Parametri: `date` (AAAAMMGG), `level` (`regioni`, `province`, `comuni`, `ripartizioni`), `region` (`cod_reg`), `province` (`cod_uts`), `format` (`gpkg`, `zip`, `csv`, `kml`, `kmz`).

## Codici

- Regioni, `cod_reg`: 1 = Piemonte … 20 = Sardegna.
- Province, `cod_uts`: per esempio 201 = Torino.
- Senza zeri iniziali (`1`, non `01`). I menu del form elencano le unità del 2026: per date più vecchie i codici possono essere diversi.

## Dati e licenza

Dati ISTAT, serviti da [confini-amministrativi.it](https://www.confini-amministrativi.it) (onData, Alessio Cimarelli), con licenza [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/deed.it): quando li riusi, cita ISTAT e la licenza.

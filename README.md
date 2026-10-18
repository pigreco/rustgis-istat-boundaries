# ISTAT Boundaries (RustGIS plugin)

Download the Italian administrative boundaries published by ISTAT, served by the
[onData API](https://www.confini-amministrativi.it/), into [RustGIS](https://rustgis.app).
Inspired by the QGIS plugin [ondata/istat_boundaries_downloader](https://github.com/ondata/istat_boundaries_downloader).

Data URL pattern: `https://www.confini-amministrativi.it/api/v2/it/<date>/<level>.<format>`

## Requirements

RustGIS 0.2.1 or later. Permissions: `edit` and `network` (the plugin asks RustGIS to download
the file with `layer.addUrl`; the plugin itself has no network access).

## Install

Plugins › Plugin Manager › install the `.rgplugin` from the releases, or build it (below).

## Use

- **Download Regioni / Province / Comuni / Ripartizioni**: ribbon buttons (Plugins tab). Each
  downloads the 2026 boundaries of that level as GeoPackage and adds them as layers.
- **Boundaries URL** (button): shows the URL of the 2026 regions in the status bar.
- **ISTAT Boundaries URL** (tool, with a form): reference date, level, region, province (one menu
  per geographic area), format. It prints the URL and a `curl` command. Tools cannot call
  `layer.addUrl`, so with this form you download the file yourself and open it with Add Data.

The commands also accept parameters from a script, for example another date or a region:
`{"date"?: YYYYMMDD, "level"?: regioni|province|comuni|ripartizioni, "region"?: cod_reg,
"province"?: cod_uts, "format"?: gpkg|zip|csv|kml|kmz}`.

Codes: regions are `cod_reg` (1 = Piemonte … 20 = Sardegna) and provinces `cod_uts`
(201 = Torino), both without leading zeros. The province menus list the 2026 units; for older
dates the codes can differ.

## Build

```sh
rustup target add wasm32-unknown-unknown
python scripts/package.py      # -> dist/istat-boundaries-<version>.rgplugin
cargo test
```

## License

MIT. Data: ISTAT, via confini-amministrativi.it (check their terms for reuse).

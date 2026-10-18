# Changelog

Format based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.3.2] - 2026-10-18

### Changed
- The download buttons are now **Add Ripartizioni / Regioni / Province / Comuni** (*Aggiungi …* in Italian): RustGIS downloads the file and adds the layers, but saves no file on disk.
- Ribbon order: Ripartizioni, Regioni, Province, Comuni, then Boundaries URL.
- The README's **Use** section is now a full user guide, with an Italian version in `GUIDA.md`; the buttons' help text and the plugin description link to it.

## [0.3.1] - 2026-10-18

### Added
- Credits for onData and Alessio Cimarelli in the README and in the plugin description (English and Italian).

## [0.3.0] - 2026-10-18

### Added
- Italian translation of the interface and of the error messages (`i18n` table in `plugin.json`).
- Screenshot of the form in the README.
- Data licence (CC BY 4.0) in the README.

### Changed
- The "no filter" menu entry is now `all (no filter)` (shown as `tutte (nessun filtro)` in Italian); both are accepted.
- Author and copyright: Salvatore Fiandaca. `homepage` points to this repository.

## [0.2.1]

### Added
- Tool "ISTAT Boundaries URL": builds the onData API address and a `curl` command from a reference date (24 dates), level, region, province and format.
- Buttons to download regions, provinces, comuni and geographic areas (`layer.addUrl`, reference date 2026-01-01).
- Button that shows the regions URL in the status bar.
- Check that a province belongs to the chosen region (date 2026-01-01).

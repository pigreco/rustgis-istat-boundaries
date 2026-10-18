# Changelog

Format based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

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

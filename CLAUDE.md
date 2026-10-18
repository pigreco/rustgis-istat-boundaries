# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Cos'è

Plugin [RustGIS](https://rustgis.app) (modulo WebAssembly in sandbox, senza rete né processi) che costruisce gli URL dell'API onData `https://www.confini-amministrativi.it/api/v2/it/<AAAAMMGG>/<percorso>.<formato>` e, tramite i pulsanti, fa scaricare i confini ISTAT a RustGIS. `NOTES.md` contiene lo stato del lavoro, i vincoli di RustGIS emersi e i prossimi passi: leggilo all'inizio.

## Comandi

- Test: `cargo test` (un test solo: `cargo test urls_follow_the_api_paths`). Verificano solo la costruzione di URL e `curl`, non il download.
- Pacchetto: `python3 scripts/package.py` → `dist/istat-boundaries-<versione>.rgplugin` (zip con `plugin.json`, `plugin.wasm`, `README.md`). Serve il target `wasm32-unknown-unknown`. Lo script funziona con Python 3.8 (quello di WSL); `scripts/build.py` di `rustgis-plugins` richiede 3.11 e qui non parte.
- Non c'è un linter configurato. Il progetto non è un repository git.

## Architettura

Tutta la logica è in `src/lib.rs`; `plugin.json` è il manifesto e deve restare coerente con il codice.

- `handle` smista per `(Kind, id)`. Ogni `id` nel codice deve corrispondere a un comando o strumento di `plugin.json`, altrimenti restituisce errore "has no '...'".
- Gli **strumenti** (con form) possono solo restituire messaggi (URL e `curl`): non possono fare richieste a RustGIS, quindi non scaricano. I **comandi** (pulsanti) possono chiamare `host::run("layer.addUrl", ...)`, ma non hanno un form: `params` nel manifesto è solo una descrizione, i parametri arrivano solo da script.
- `build_url` è l'unica funzione che costruisce gli URL; la usano comandi, strumento e `download`. `download` imposta il livello fisso del pulsante se lo script non ne passa uno.
- Le voci dei menu del form arrivano come testo (es. "17 Basilicata"); `code` estrae al massimo 3 cifre iniziali, senza zeri iniziali (l'API vuole `1`, non `01`), e ignora voci senza numero ("tutte").
- Dati statici in cima a `lib.rs`: `DATES` (24 date esatte, non tutte `AAAA0101`), `UTS` (province come coppie `(cod_uts, cod_reg)`, elenco 2026) e `FORMATS`. La coerenza provincia/regione è controllata solo per la data 20260101.

## Vincoli da ricordare

- Un `choice` ammette al massimo 64 voci: per questo le province sono in 5 menu, uno per ripartizione. Altri limiti del manifesto: 64 comandi, 64 strumenti, 32 parametri per strumento.
- Cambiando versione vanno allineati `Cargo.toml`, `plugin.json` e il nome del pacchetto in `dist`.
- Il plugin si installa a mano da Plugin Manager di RustGIS (`plugin.install` è riservato all'utente); il server MCP di RustGIS non lo vede.
- Nessuna azione pubblica (repository, PR, issue verso `opengeos`) senza conferma esplicita dell'utente.

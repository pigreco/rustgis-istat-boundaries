//! ISTAT Boundaries: builds onData API addresses for ISTAT administrative boundaries
//! (the same ones as the QGIS plugin ondata/istat_boundaries_downloader). A plugin has no network access, so the download itself is the user's.

use rustgis_plugin::{Kind, Request, ToolOutput, Value, host, json};

/// The manifest, for the `i18n` table that words the error messages in the interface language.
const MANIFEST: &str = include_str!("../plugin.json");
const BASE: &str = "https://www.confini-amministrativi.it/api/v2/it/";
/// The reference dates the onData API serves.
const DATES: [&str; 24] = ["20260101", "20250101", "20240101", "20230101", "20220101", "20210101", "20200101", "20190101", "20180101", "20170101", "20160101", "20150101", "20140101", "20130101", "20120101", "20111009", "20100101", "20060101", "20050101", "20040101", "20030101", "20020101", "20011021", "19911020"];
/// (cod_uts, cod_reg) of the 2026 provinces and metropolitan cities, to check a province
/// against the region chosen with it.
const UTS: [(u16, u8); 110] = [(6, 1), (5, 1), (96, 1), (4, 1), (3, 1), (201, 1), (103, 1), (2, 1), (7, 2), (16, 3), (17, 3), (13, 3), (19, 3), (97, 3), (98, 3), (20, 3), (215, 3), (108, 3), (18, 3), (14, 3), (12, 3), (21, 4), (22, 4), (25, 5), (28, 5), (29, 5), (26, 5), (227, 5), (23, 5), (24, 5), (31, 6), (93, 6), (32, 6), (30, 6), (210, 7), (8, 7), (11, 7), (9, 7), (237, 8), (38, 8), (40, 8), (36, 8), (34, 8), (33, 8), (39, 8), (35, 8), (99, 8), (51, 9), (248, 9), (53, 9), (49, 9), (46, 9), (45, 9), (50, 9), (47, 9), (100, 9), (52, 9), (54, 10), (55, 10), (42, 11), (44, 11), (109, 11), (43, 11), (41, 11), (60, 12), (59, 12), (57, 12), (258, 12), (56, 12), (69, 13), (66, 13), (68, 13), (67, 13), (70, 14), (94, 14), (64, 15), (62, 15), (61, 15), (263, 15), (65, 15), (272, 16), (110, 16), (74, 16), (71, 16), (75, 16), (73, 16), (77, 17), (76, 17), (79, 18), (78, 18), (101, 18), (280, 18), (102, 18), (84, 19), (85, 19), (287, 19), (86, 19), (283, 19), (282, 19), (88, 19), (89, 19), (81, 19), (318, 20), (113, 20), (117, 20), (114, 20), (116, 20), (115, 20), (312, 20), (119, 20)];

const FORMATS: [&str; 5] = ["gpkg", "zip", "csv", "kml", "kmz"];

fn handle(req: Request) -> Result<Value, String> {
    match (req.kind, req.id.as_str()) {
        (Kind::Command, "boundaries-url") => {
            let url = build_url(&req)?;
            host::notify(&url);
            Ok(json!({"url": url, "curl": curl(&url)}))
        }
        (Kind::Command, "download-boundaries") => download(req, "regioni"),
        (Kind::Command, "download-province") => download(req, "province"),
        (Kind::Command, "download-comuni") => download(req, "comuni"),
        (Kind::Command, "download-ripartizioni") => download(req, "ripartizioni"),
        (Kind::Tool, "boundaries-url") => {
            let url = build_url(&req)?;
            Ok(ToolOutput::new().message(url.clone()).message(curl(&url)).into())
        }
        (_, other) => Err(req.tr(MANIFEST, "ISTAT Boundaries has no '{other}'", &[("other", other)])),
    }
}

rustgis_plugin::export!(handle);

/// Has RustGIS download the file and add it as layers (`layer.addUrl`, needs `network` and `edit`).
/// The ribbon button sets the level; a script can still pass its own.
fn download(mut req: Request, level: &str) -> Result<Value, String> {
    if !req.params.is_object() {
        req.params = json!({});
    }
    if req.params.get("level").is_none() {
        req.params["level"] = json!(level);
    }
    let url = build_url(&req)?;
    let name = url.strip_prefix(BASE).unwrap_or(&url).rsplit_once('.').map_or(url.as_str(), |(p, _)| p).replace('/', " ");
    host::run("layer.addUrl", json!({"url": url, "name": name, "zoom": true}))
}

/// A `curl` command that saves `url` in the current folder, as `confini_<date>_<path>.<format>`.
fn curl(url: &str) -> String {
    let name = url.strip_prefix(BASE).unwrap_or(url).replace('/', "_");
    format!("curl -L -o confini_{name} \"{url}\"")
}

/// A code: digits only (so it can't alter the address), up to 3, read from the start of the
/// text, so a menu entry such as "17 Basilicata" gives 17. Entries with no number ("all (no filter)", or "tutte" in Italian)
/// mean no filter. The API wants it without leading zeros (`1`, not `01`).
fn code<'a>(req: &'a Request, name: &str) -> Result<Option<&'a str>, String> {
    let Some(text) = req.str(name).map(str::trim).filter(|s| !s.is_empty()) else { return Ok(None) };
    if text.starts_with("all (") || text.starts_with("tutte") {
        return Ok(None);
    }
    let end = text.find(|c: char| !c.is_ascii_digit()).unwrap_or(text.len());
    let (digits, label) = text.split_at(end);
    if (1..=3).contains(&digits.len()) && (label.is_empty() || label.starts_with(' ')) {
        Ok(Some(digits.trim_start_matches('0').max("0")))
    } else {
        Err(req.tr(MANIFEST, "'{name}': '{text}' is not a code (up to 3 digits)", &[("name", name), ("text", text)]))
    }
}

/// `{BASE}{date}/{path}.{format}`, with the same paths as the onData API.
fn build_url(req: &Request) -> Result<String, String> {
    let date = req.str("date").map(str::trim).filter(|s| !s.is_empty()).unwrap_or(DATES[0]);
    if !DATES.contains(&date) {
        return Err(req.tr(MANIFEST, "'{date}' is not a reference date; use one of {dates}", &[("date", date), ("dates", &DATES.join(", "))]));
    }
    let format = req.str("format").unwrap_or("gpkg");
    if !FORMATS.contains(&format) {
        return Err(req.tr(MANIFEST, "format '{format}': use one of {formats}", &[("format", format), ("formats", &FORMATS.join(", "))]));
    }
    let region = code(req, "region")?;
    // One menu per geographic area (a form menu holds at most 64 entries); a command passes `province`.
    let mut provinces = ["province", "province_nw", "province_ne", "province_c", "province_s", "province_i"]
        .into_iter()
        .filter_map(|n| code(req, n).transpose());
    let province = provinces.next().transpose()?;
    if provinces.next().is_some() {
        return Err(req.tr(MANIFEST, "choose a province in one area menu only", &[]));
    }
    drop(provinces);
    if let (Some(r), Some(p), true) = (region, province, date == DATES[0]) {
        let (r, p): (u8, u16) = (r.parse().unwrap_or(0), p.parse().unwrap_or(0));
        if UTS.iter().any(|&(c, _)| c == p) && !UTS.contains(&(p, r)) {
            return Err(req.tr(MANIFEST, "province {p} is not in region {r}: choose a matching province or set the region to 'all'", &[("p", &p.to_string()), ("r", &r.to_string())]));
        }
    }
    let comuni = req.bool("comuni") == Some(true);
    let uts = "unita-territoriali-sovracomunali";
    let path = match req.str("level").unwrap_or("regioni") {
        "regioni" => match region {
            Some(r) => format!("regioni/{r}"),
            None => "regioni".to_string(),
        },
        "ripartizioni" | "ripartizioni-geografiche" => "ripartizioni-geografiche".to_string(),
        "province" => match region {
            Some(r) => format!("regioni/{r}/{uts}"),
            None => uts.to_string(),
        },
        "comuni" => match (province, region) {
            (Some(p), _) => format!("{uts}/{p}/comuni"),
            (None, Some(r)) => format!("regioni/{r}/comuni"),
            (None, None) => "comuni".to_string(),
        },
        other => return Err(req.tr(MANIFEST, "level '{other}': use regioni, province, comuni or ripartizioni", &[("other", other)])),
    };
    // A command can also ask for one province, or its comuni.
    let path = match (req.str("level"), province, comuni) {
        (Some("province"), Some(p), false) => format!("{uts}/{p}"),
        (Some("province"), Some(p), true) => format!("{uts}/{p}/comuni"),
        _ => path,
    };
    Ok(format!("{BASE}{date}/{path}.{format}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curl_names_the_file_after_the_path() {
        let url = format!("{BASE}20260101/unita-territoriali-sovracomunali/201/comuni.gpkg");
        assert_eq!(
            curl(&url),
            format!("curl -L -o confini_20260101_unita-territoriali-sovracomunali_201_comuni.gpkg \"{url}\"")
        );
    }

    fn url(params: Value) -> Result<String, String> {
        url_in("en", params)
    }

    fn url_in(lang: &str, params: Value) -> Result<String, String> {
        let req = Request {
            kind: Kind::Tool,
            id: "boundaries-url".into(),
            params,
            layers: Default::default(),
            files: Default::default(),
            lang: lang.into(),
        };
        build_url(&req)
    }

    #[test]
    fn errors_follow_the_interface_language() {
        let bad = json!({"date": "20260102"});
        assert!(url_in("en", bad.clone()).unwrap_err().contains("is not a reference date"));
        assert!(url_in("it", bad).unwrap_err().contains("non è una data di riferimento"));
        assert_eq!(url(json!({"region": "all (no filter)"})).unwrap(), url(json!({"region": "tutte (nessun filtro)"})).unwrap());
    }

    #[test]
    fn urls_follow_the_api_paths() {
        let u = |tail: &str| format!("{BASE}20260101/{tail}");
        assert_eq!(url(json!({})).unwrap(), u("regioni.gpkg"));
        assert_eq!(url(json!({"level": "regioni", "region": "17"})).unwrap(), u("regioni/17.gpkg"));
        assert_eq!(url(json!({"level": "province", "region": "17"})).unwrap(), u("regioni/17/unita-territoriali-sovracomunali.gpkg"));
        assert_eq!(url(json!({"level": "comuni", "region": "17"})).unwrap(), u("regioni/17/comuni.gpkg"));
        assert_eq!(url(json!({"level": "comuni", "province": "201"})).unwrap(), u("unita-territoriali-sovracomunali/201/comuni.gpkg"));
        assert_eq!(url(json!({"level": "comuni", "region": "01", "format": "zip"})).unwrap(), u("regioni/1/comuni.zip"));
        assert_eq!(url(json!({"level": "ripartizioni"})).unwrap(), u("ripartizioni-geografiche.gpkg"));
        assert_eq!(url(json!({"level": "comuni", "region": "17 Basilicata"})).unwrap(), u("regioni/17/comuni.gpkg"));
        assert_eq!(url(json!({"level": "comuni", "region": "tutte (nessun filtro)"})).unwrap(), u("comuni.gpkg"));
        assert!(url(json!({"region": "1/../x"})).is_err());
        assert_eq!(url(json!({"level": "comuni", "province": "201 Torino (TO) - Piemonte"})).unwrap(), u("unita-territoriali-sovracomunali/201/comuni.gpkg"));
        assert_eq!(url(json!({"level": "comuni", "province_s": "tutte (nessun filtro)", "province_nw": "201 Torino (TO) - Piemonte"})).unwrap(), u("unita-territoriali-sovracomunali/201/comuni.gpkg"));
        assert!(url(json!({"province_s": "2 Vercelli (VC) - Piemonte", "province_nw": "201 Torino (TO) - Piemonte"})).is_err());
        assert!(url(json!({"level": "comuni", "region": "17 Basilicata", "province": "201 Torino (TO) - Piemonte"})).is_err());
        assert!(url(json!({"level": "comuni", "region": "1 Piemonte", "province": "201 Torino (TO) - Piemonte"})).is_ok());
        assert!(url(json!({"date": "20260102"})).is_err());
    }
}

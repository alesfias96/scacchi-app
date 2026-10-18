//! Il tema dell'interfaccia.

/// L'aspetto dell'app: colori, tipografia e atmosfera.
///
/// Nel JSON diventa il nome in minuscolo (`"studio"`, `"auto"`, ...).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS), ts(export))]
pub enum AppTheme {
    /// Segue il sistema operativo: Giorno con il tema chiaro, Sera con il tema scuro.
    Auto,
    /// Scuro e raccolto, con dettagli ambra.
    Sera,
    /// Chiaro e arioso, con dettagli verdi.
    Giorno,
    /// Barra laterale blu inchiostro e dettagli arancioni (predefinito).
    #[default]
    Studio,
}

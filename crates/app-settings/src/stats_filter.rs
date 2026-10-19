//! I filtri della sezione «Progressi»: periodo e avversario.

/// Il periodo delle statistiche.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS), ts(export))]
pub enum StatsPeriod {
    /// Gli ultimi 30 giorni.
    Month,
    /// Gli ultimi 90 giorni.
    Quarter,
    /// Da sempre (predefinito).
    #[default]
    All,
}

impl StatsPeriod {
    /// Quanti giorni indietro guardare; `None` = da sempre.
    #[must_use]
    pub fn days(self) -> Option<u32> {
        match self {
            Self::Month => Some(30),
            Self::Quarter => Some(90),
            Self::All => None,
        }
    }
}

/// Contro chi si sono giocate le partite da contare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS), ts(export))]
pub enum StatsOpponent {
    /// Tutte le partite (predefinito).
    #[default]
    All,
    /// Solo contro il computer.
    Computer,
    /// Solo a due giocatori.
    TwoPlayers,
}

/// I filtri scelti nella sezione «Progressi» (salvati con le impostazioni).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
#[cfg_attr(feature = "bindings", derive(ts_rs::TS), ts(export))]
pub struct StatsFilter {
    /// Il periodo.
    pub period: StatsPeriod,
    /// L'avversario.
    pub opponent: StatsOpponent,
}

#[cfg(test)]
mod tests {
    use super::{StatsFilter, StatsOpponent, StatsPeriod};

    #[test]
    fn by_default_counts_everything() {
        let filter = StatsFilter::default();

        assert_eq!(filter.period, StatsPeriod::All);
        assert_eq!(filter.opponent, StatsOpponent::All);
        assert_eq!(StatsPeriod::Month.days(), Some(30));
        assert_eq!(StatsPeriod::All.days(), None);
    }
}

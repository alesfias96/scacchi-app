//! Una scheda della base di conoscenza, com'è scritta nel suo file TOML.

use crate::topic::{Topic, Trigger};

/// Per chi è un piano della scheda.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlanFor {
    /// Per il Bianco (es. nelle aperture).
    White,
    /// Per il Nero.
    Black,
    /// Per chi "ha" la cosa riconosciuta (es. chi ha il pedone isolato).
    Owner,
    /// Per l'altro giocatore (es. chi gioca contro il pedone isolato).
    Opponent,
    /// Per tutti e due.
    Both,
}

/// Un piano: per chi è e i suoi passi.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CardPlan {
    /// Per chi è il piano.
    #[serde(rename = "for")]
    pub for_side: PlanFor,
    /// Come chiamare chi segue il piano (es. "Chi ha l'isolano"); se manca
    /// si usa "il Bianco" / "il Nero".
    #[serde(default)]
    pub label: Option<String>,
    /// I passi del piano, in ordine.
    pub steps: Vec<String>,
}

/// Il contenuto di un file della base di conoscenza.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CardContent {
    /// Il titolo (es. "Il pedone isolato di donna").
    pub title: String,
    /// Quando vale la scheda (es. `["structure:isolated-queen-pawn"]`).
    pub when: Vec<String>,
    /// Una o due frasi che dicono di cosa si tratta.
    pub summary: String,
    /// Le idee principali.
    #[serde(default)]
    pub ideas: Vec<String>,
    /// I piani tipici.
    #[serde(default)]
    pub plans: Vec<CardPlan>,
    /// Gli errori tipici da evitare.
    #[serde(default)]
    pub mistakes: Vec<String>,
    /// Quando conviene seguire l'idea della scheda (soprattutto per i piani).
    #[serde(default)]
    pub use_when: Vec<String>,
    /// Quando invece non conviene.
    #[serde(default)]
    pub avoid_when: Vec<String>,
    /// I segnali che il piano sta funzionando.
    #[serde(default)]
    pub signs: Vec<String>,
}

/// Una scheda pronta da usare.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeCard {
    /// Il nome del file, con la cartella (es. `"structures/isolated-queen-pawn"`).
    pub id: String,
    /// L'argomento (dalla cartella).
    pub topic: Topic,
    /// Quando vale, già letto.
    pub triggers: Vec<Trigger>,
    /// Il contenuto.
    pub content: CardContent,
}

/// Il motivo per cui una scheda non è valida (lo controllano i test).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CardError {
    /// Il file non è un TOML valido o ha campi sconosciuti.
    #[error("la scheda {id} non si legge: {message}")]
    Unreadable {
        /// Il nome della scheda.
        id: String,
        /// Il problema trovato.
        message: String,
    },
    /// La cartella non corrisponde a nessun argomento.
    #[error("la scheda {0} non sta in una cartella conosciuta")]
    UnknownFolder(String),
    /// Un "quando vale" non è scritto bene o è di un altro argomento.
    #[error("la scheda {id} ha un \"quando vale\" non valido: {trigger}")]
    BadTrigger {
        /// Il nome della scheda.
        id: String,
        /// Il "quando vale" sbagliato.
        trigger: String,
    },
}

impl KnowledgeCard {
    /// Legge una scheda.
    ///
    /// - Riceve: il nome (`cartella/file`) e il contenuto del file.
    ///
    /// # Errors
    ///
    /// Un [`CardError`] se il file non si legge, sta nella cartella sbagliata
    /// o ha un "quando vale" non valido (deve essere dello stesso argomento della cartella).
    pub fn parse(id: &str, text: &str) -> Result<Self, CardError> {
        let content: CardContent = toml::from_str(text).map_err(|error| CardError::Unreadable {
            id: id.to_owned(),
            message: error.message().to_owned(),
        })?;
        let folder = id.split('/').next().unwrap_or_default();
        let topic = Topic::ALL
            .into_iter()
            .find(|topic| topic.folder() == folder)
            .ok_or_else(|| CardError::UnknownFolder(id.to_owned()))?;
        let triggers = content
            .when
            .iter()
            .map(|text| {
                Trigger::parse(text)
                    .filter(|trigger| trigger.topic == topic)
                    .ok_or_else(|| CardError::BadTrigger {
                        id: id.to_owned(),
                        trigger: text.clone(),
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            id: id.to_owned(),
            topic,
            triggers,
            content,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{CardError, KnowledgeCard, PlanFor};
    use crate::topic::Topic;

    const CARD: &str = r#"
title = "Prova"
when = ["structure:test"]
summary = "Una scheda di prova."
ideas = ["Prima idea"]

[[plans]]
for = "owner"
label = "Chi ha la struttura"
steps = ["Primo passo"]
"#;

    #[test]
    fn reads_a_card() {
        let card = KnowledgeCard::parse("structures/test", CARD).expect("scheda valida");

        assert_eq!(card.topic, Topic::Structure);
        assert_eq!(card.triggers[0].key, "test");
        assert_eq!(card.content.plans[0].for_side, PlanFor::Owner);
    }

    #[test]
    fn a_trigger_must_match_the_folder() {
        assert!(matches!(
            KnowledgeCard::parse("endgames/test", CARD),
            Err(CardError::BadTrigger { .. })
        ));
    }

    #[test]
    fn unknown_fields_are_errors() {
        let text = format!("{CARD}\ncolor = \"blu\"\n");

        assert!(matches!(
            KnowledgeCard::parse("structures/test", &text),
            Err(CardError::Unreadable { .. })
        ));
    }
}

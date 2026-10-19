//! La revisione di una partita, pronta per l'interfaccia.

use analysis::{Fact, GamePhase};
use chess_core::{LegalMove, PositionView, Side};
use engine::Evaluation;

use crate::move_quality::MoveQuality;

/// Una mossa della partita con il suo giudizio.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct MoveReview {
    /// La mezza mossa (1 = la prima).
    pub ply: u32,
    /// Il numero di mossa come nei libri.
    pub move_number: u32,
    /// Chi l'ha giocata.
    pub side: Side,
    /// La mossa in SAN (es. `"Nf3"`).
    pub san: String,
    /// La mossa giocata, con le sue case.
    pub played: LegalMove,
    /// Il giudizio; `None` se mancano le valutazioni del motore.
    pub quality: Option<MoveQuality>,
    /// I punti di probabilità di vittoria persi con questa mossa.
    pub chance_lost: Option<f64>,
    /// La valutazione prima della mossa (per il Bianco).
    pub evaluation_before: Option<Evaluation>,
    /// La valutazione dopo la mossa (per il Bianco).
    pub evaluation_after: Option<Evaluation>,
    /// La mossa migliore secondo il motore, con le sue case.
    pub best: Option<LegalMove>,
    /// La mossa migliore in SAN.
    pub best_san: Option<String>,
    /// La fase di gioco della posizione prima della mossa.
    pub phase: GamePhase,
}

/// Il riepilogo di un giocatore.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct PlayerSummary {
    /// La precisione media delle sue mosse, da 0 a 100.
    pub accuracy: Option<u32>,
    /// Quante imprecisioni.
    pub inaccuracies: u32,
    /// Quanti errori.
    pub mistakes: u32,
    /// Quanti errori gravi.
    pub blunders: u32,
    /// La fase di gioco in cui ha perso di più, se ha sbagliato.
    pub worst_phase: Option<GamePhase>,
}

/// Un momento critico: una mossa che ha cambiato la partita, spiegata.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct CriticalMoment {
    /// La mezza mossa.
    pub ply: u32,
    /// Il giudizio.
    pub quality: MoveQuality,
    /// Il giudizio a parole (es. "Errore grave: la probabilità di vincere del
    /// Bianco scende dal 62% al 18%").
    pub text: String,
    /// La mossa giocata, in SAN.
    pub played_san: String,
    /// La mossa migliore, in SAN.
    pub best_san: Option<String>,
    /// Cosa la mossa giocata concedeva all'avversario (dai rilevatori).
    pub conceded: Vec<Fact>,
    /// Cosa otteneva la mossa migliore (dai rilevatori).
    pub best_gains: Vec<Fact>,
    /// Il motivo tattico del momento (quello sfuggito o concesso), per allenarsi
    /// con i puzzle dello stesso tema.
    pub motif: Option<tutor::MotifView>,
}

/// La revisione completa di una partita.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "bindings",
    derive(serde::Serialize, ts_rs::TS),
    serde(rename_all = "camelCase"),
    ts(export)
)]
pub struct GameReview {
    /// La posizione di partenza (FEN).
    pub start_fen: String,
    /// Le posizioni della partita (una in più delle mosse), per la scacchiera.
    pub positions: Vec<PositionView>,
    /// Le mosse, con il loro giudizio.
    pub moves: Vec<MoveReview>,
    /// La probabilità di vittoria del Bianco in ogni posizione (una in più
    /// delle mosse): è il grafico della partita.
    pub white_chances: Vec<f64>,
    /// Il riepilogo del Bianco.
    pub white: PlayerSummary,
    /// Il riepilogo del Nero.
    pub black: PlayerSummary,
    /// I momenti critici, in ordine di partita.
    pub critical: Vec<CriticalMoment>,
}

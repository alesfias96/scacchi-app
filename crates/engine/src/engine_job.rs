//! Le richieste che si possono fare a un motore.

use tokio::sync::oneshot;

use crate::analysis_update::AnalysisUpdate;
use crate::engine_error::EngineError;
use crate::search_limits::SearchLimits;
use crate::search_position::SearchPosition;

/// Una richiesta per il motore. Una nuova richiesta interrompe quella in corso.
#[derive(Debug)]
pub enum EngineJob {
    /// Analizza la posizione senza fermarti, inviando un aggiornamento a ogni
    /// profondità completata, finché non arriva un'altra richiesta.
    Analyze(SearchPosition),
    /// Trova la mossa da giocare nella posizione.
    FindMove(MoveRequest),
    /// Valuta la posizione per poco tempo e consegna il giudizio finale.
    Evaluate(EvaluateRequest),
    /// Smetti di pensare.
    Stop,
}

/// La richiesta di una mossa da giocare.
#[derive(Debug)]
pub struct MoveRequest {
    /// La posizione in cui muovere.
    pub position: SearchPosition,
    /// Per quanto tempo pensare.
    pub limits: SearchLimits,
    /// Le impostazioni del livello di gioco: (nome, valore).
    pub options: Vec<(String, String)>,
    /// Dove consegnare la mossa trovata (in notazione UCI), o l'errore.
    pub reply: oneshot::Sender<Result<String, EngineError>>,
}

/// Una richiesta di valutazione breve (ad esempio per il tutor).
#[derive(Debug)]
pub struct EvaluateRequest {
    /// La posizione da valutare.
    pub position: SearchPosition,
    /// Per quanto tempo pensare.
    pub limits: SearchLimits,
    /// Dove consegnare l'ultimo giudizio (valutazione e variante), o l'errore.
    pub reply: oneshot::Sender<Result<AnalysisUpdate, EngineError>>,
}

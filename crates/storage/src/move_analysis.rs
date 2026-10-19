//! I dati dell'analisi delle mosse: salvataggio e lettura.

use sqlx::Row;

use crate::database::Database;
use crate::storage_error::StorageError;

/// Scrive i dati dell'analisi di una mossa.
const UPDATE_MOVE: &str = "
    UPDATE moves SET
        eval_before_cp = ?, eval_before_mate = ?, eval_after_cp = ?, eval_after_mate = ?,
        best_move_uci = ?, centipawn_loss = ?, game_phase = ?, tactical_themes = ?, missed_threats = ?
    WHERE game_id = ? AND ply = ?";
/// Segna la partita come analizzata.
const MARK_ANALYZED: &str = "UPDATE games SET analyzed_at = ? WHERE id = ?";
/// Quando la partita è stata analizzata.
const SELECT_ANALYZED_AT: &str = "SELECT analyzed_at FROM games WHERE id = ?";
/// Legge i dati dell'analisi di tutte le mosse.
const SELECT_MOVES: &str = "
    SELECT ply, eval_before_cp, eval_before_mate, eval_after_cp, eval_after_mate,
           best_move_uci, centipawn_loss, game_phase, tactical_themes, missed_threats
    FROM moves WHERE game_id = ? ORDER BY ply";

/// I dati dell'analisi di una mossa, come si salvano.
///
/// Le valutazioni sono dal punto di vista del Bianco: in centipedoni (`_cp`)
/// oppure "matto in N" (`_mate`); al più una delle due è presente.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MoveAnalysisRecord {
    /// La mezza mossa (1 = la prima).
    pub ply: u32,
    /// La valutazione prima della mossa, in centipedoni.
    pub eval_before_cp: Option<i32>,
    /// La valutazione prima della mossa, come matto in N.
    pub eval_before_mate: Option<i32>,
    /// La valutazione dopo la mossa, in centipedoni.
    pub eval_after_cp: Option<i32>,
    /// La valutazione dopo la mossa, come matto in N.
    pub eval_after_mate: Option<i32>,
    /// La mossa migliore (UCI).
    pub best_move_uci: Option<String>,
    /// I centipedoni persi.
    pub centipawn_loss: Option<i32>,
    /// La fase di gioco ('opening' / 'middlegame' / 'endgame').
    pub game_phase: Option<String>,
    /// Le tattiche e le minacce a favore di chi muove.
    pub tactical_themes: Vec<String>,
    /// Ciò che la mossa ha concesso all'avversario.
    pub missed_threats: Vec<String>,
}

/// Un elenco di testi come JSON (vuoto = nessun valore).
fn to_json(values: &[String]) -> Option<String> {
    (!values.is_empty()).then(|| serde_json::to_string(values).unwrap_or_default())
}

/// Un elenco di testi letto dal JSON (se non si legge: vuoto).
fn from_json(text: Option<String>) -> Vec<String> {
    text.and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default()
}

impl Database {
    /// Salva l'analisi di una partita e la segna come analizzata, tutto insieme.
    ///
    /// # Errors
    ///
    /// [`StorageError::Database`] se la scrittura non riesce.
    pub async fn save_analysis(
        &self,
        game_id: &str,
        records: &[MoveAnalysisRecord],
        analyzed_at_ms: i64,
    ) -> Result<(), StorageError> {
        let mut transaction = self.pool.begin().await?;
        for record in records {
            sqlx::query(UPDATE_MOVE)
                .bind(record.eval_before_cp)
                .bind(record.eval_before_mate)
                .bind(record.eval_after_cp)
                .bind(record.eval_after_mate)
                .bind(&record.best_move_uci)
                .bind(record.centipawn_loss)
                .bind(&record.game_phase)
                .bind(to_json(&record.tactical_themes))
                .bind(to_json(&record.missed_threats))
                .bind(game_id)
                .bind(i64::from(record.ply))
                .execute(&mut *transaction)
                .await?;
        }
        sqlx::query(MARK_ANALYZED)
            .bind(analyzed_at_ms)
            .bind(game_id)
            .execute(&mut *transaction)
            .await?;
        transaction.commit().await?;
        Ok(())
    }

    /// L'analisi salvata di una partita: `None` se non è mai stata analizzata.
    ///
    /// # Errors
    ///
    /// [`StorageError::Database`] se la lettura non riesce.
    pub async fn load_analysis(
        &self,
        game_id: &str,
    ) -> Result<Option<Vec<MoveAnalysisRecord>>, StorageError> {
        let analyzed: Option<Option<i64>> = sqlx::query_scalar(SELECT_ANALYZED_AT)
            .bind(game_id)
            .fetch_optional(&self.pool)
            .await?;
        if analyzed.flatten().is_none() {
            return Ok(None);
        }
        let rows = sqlx::query(SELECT_MOVES)
            .bind(game_id)
            .fetch_all(&self.pool)
            .await?;
        rows.iter()
            .map(|row| {
                Ok(MoveAnalysisRecord {
                    ply: row.try_get("ply")?,
                    eval_before_cp: row.try_get("eval_before_cp")?,
                    eval_before_mate: row.try_get("eval_before_mate")?,
                    eval_after_cp: row.try_get("eval_after_cp")?,
                    eval_after_mate: row.try_get("eval_after_mate")?,
                    best_move_uci: row.try_get("best_move_uci")?,
                    centipawn_loss: row.try_get("centipawn_loss")?,
                    game_phase: row.try_get("game_phase")?,
                    tactical_themes: from_json(row.try_get("tactical_themes")?),
                    missed_threats: from_json(row.try_get("missed_threats")?),
                })
            })
            .collect::<Result<Vec<_>, StorageError>>()
            .map(Some)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use game_session::GameSession;

    use super::MoveAnalysisRecord;
    use crate::database::Database;

    #[tokio::test]
    async fn saves_and_reads_the_analysis_of_a_game() {
        let database = Database::open_in_memory().await.expect("database aperto");
        let now = Instant::now();
        let mut session = GameSession::default();
        session.play("e2e4", now).expect("mossa legale");
        database
            .save(&session.snapshot(now))
            .await
            .expect("salvataggio");
        assert_eq!(
            database.load_analysis(session.id()).await.expect("lettura"),
            None
        );

        let record = MoveAnalysisRecord {
            ply: 1,
            eval_before_cp: Some(20),
            eval_after_cp: Some(35),
            best_move_uci: Some("e2e4".to_owned()),
            centipawn_loss: Some(0),
            game_phase: Some("opening".to_owned()),
            tactical_themes: vec!["Il pedone è passato".to_owned()],
            ..MoveAnalysisRecord::default()
        };
        database
            .save_analysis(session.id(), std::slice::from_ref(&record), 1_000)
            .await
            .expect("salvataggio dell'analisi");

        assert_eq!(
            database.load_analysis(session.id()).await.expect("lettura"),
            Some(vec![record])
        );
        let games = database
            .list(&crate::GameFilter::default())
            .await
            .expect("elenco");
        assert!(games[0].analyzed);
    }
}

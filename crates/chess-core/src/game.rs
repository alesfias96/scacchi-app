//! La partita: sequenza di mosse, posizioni, navigazione e fine partita.

use shakmaty::fen::Fen;
use shakmaty::san::SanPlus;
use shakmaty::uci::UciMove;
use shakmaty::zobrist::Zobrist64;
use shakmaty::{Chess, EnPassantMode, KnownOutcome, Move, Outcome, Position};

use crate::game_error::GameError;
use crate::game_result::{GameEndReason, GameResult};
use crate::game_view::GameView;
use crate::legal_move::LegalMove;
use crate::move_record::MoveRecord;
use crate::position_view::PositionView;
use crate::setup::parse_fen;
use crate::side::Side;

/// Numero di mezze mosse senza catture né mosse di pedone che fa scattare
/// la regola delle 50 mosse (50 mosse per parte = 100 mezze mosse).
const FIFTY_MOVE_RULE_HALFMOVES: u32 = 100;

/// Quante volte deve presentarsi la stessa posizione per la patta per ripetizione.
const REPETITIONS_FOR_DRAW: usize = 3;

/// Una partita a scacchi.
///
/// Tiene tutte le posizioni attraversate (così si può tornare indietro senza
/// ricalcolare nulla) e un "cursore" che indica quale posizione è mostrata.
/// Se si gioca una mossa mentre il cursore è indietro, il seguito della
/// partita viene sostituito dalla nuova mossa.
#[derive(Debug, Clone)]
pub struct Game {
    /// `positions[0]` è la posizione di partenza; `positions[i]` quella dopo `i` mosse.
    positions: Vec<Chess>,
    /// Impronta di ogni posizione (stesso indice di `positions`), per riconoscere le ripetizioni.
    hashes: Vec<Zobrist64>,
    /// Le mosse giocate.
    moves: Vec<MoveRecord>,
    /// Quante mosse sono state giocate fino alla posizione mostrata.
    cursor: usize,
}

impl Default for Game {
    /// Una partita nuova, dalla posizione iniziale.
    fn default() -> Self {
        Self::from_position(Chess::default())
    }
}

impl Game {
    /// Crea una partita nuova, dalla posizione iniziale.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Crea una partita che parte dalla posizione indicata.
    #[must_use]
    pub fn from_position(start: Chess) -> Self {
        let hash = start.zobrist_hash(EnPassantMode::Legal);
        Self {
            positions: vec![start],
            hashes: vec![hash],
            moves: Vec::new(),
            cursor: 0,
        }
    }

    /// Crea una partita che parte dalla posizione descritta da una FEN.
    ///
    /// # Errors
    ///
    /// [`GameError::InvalidPosition`] se la FEN non è valida.
    pub fn from_fen(fen: &str) -> Result<Self, GameError> {
        Ok(Self::from_position(parse_fen(fen)?))
    }

    /// Le mosse giocate, dalla prima all'ultima.
    #[must_use]
    pub fn moves(&self) -> &[MoveRecord] {
        &self.moves
    }

    /// La posizione di partenza in notazione FEN.
    #[must_use]
    pub fn starting_fen(&self) -> String {
        Fen::from_position(self.start(), EnPassantMode::Legal).to_string()
    }

    /// Gioca una mossa scritta in notazione UCI (es. `"e2e4"`, `"e7e8q"`)
    /// nella posizione mostrata. Se il cursore non è sull'ultima mossa,
    /// il seguito della partita viene sostituito.
    ///
    /// # Errors
    ///
    /// - [`GameError::GameOver`] se la partita è finita nella posizione mostrata;
    /// - [`GameError::IllegalMove`] se la mossa non è scritta bene o non è legale.
    pub fn play_uci(&mut self, uci: &str) -> Result<(), GameError> {
        if self.result().is_some() {
            return Err(GameError::GameOver);
        }
        let parsed = uci
            .parse::<UciMove>()
            .map_err(|_| GameError::IllegalMove(uci.to_owned()))?;
        let chess_move = parsed
            .to_move(self.current())
            .map_err(|_| GameError::IllegalMove(uci.to_owned()))?;
        self.play_move(chess_move);
        Ok(())
    }

    /// Gioca una mossa già verificata come legale nella posizione mostrata.
    ///
    /// Usata da [`Game::play_uci`] e dalla lettura dei PGN.
    pub(crate) fn play_move(&mut self, chess_move: Move) {
        self.truncate_after_cursor();

        let before = self.current().clone();
        let mut after = before.clone();
        let capture = chess_move.is_capture();
        // Calcola la notazione SAN (con + o # finale) e intanto gioca la mossa.
        let san = SanPlus::from_move_and_play_unchecked(&mut after, chess_move).to_string();
        let uci = UciMove::from_standard(chess_move);
        let (from, to) = match uci {
            UciMove::Normal { from, to, .. } => (from.to_string(), to.to_string()),
            UciMove::Put { to, .. } => (String::new(), to.to_string()),
            UciMove::Null => (String::new(), String::new()),
        };

        self.moves.push(MoveRecord {
            ply: self.moves.len() + 1,
            move_number: before.fullmoves().get(),
            side: Side::from(before.turn()),
            san,
            uci: uci.to_string(),
            from,
            to,
            capture,
        });
        self.hashes.push(after.zobrist_hash(EnPassantMode::Legal));
        self.positions.push(after);
        self.cursor = self.moves.len();
    }

    /// Ritira l'ultima mossa della partita e mostra la nuova ultima posizione.
    /// Se non ci sono mosse non fa nulla.
    pub fn undo(&mut self) {
        if self.moves.pop().is_some() {
            self.positions.pop();
            self.hashes.pop();
        }
        self.cursor = self.moves.len();
    }

    /// Mostra la posizione dopo `ply` mosse (0 = posizione di partenza).
    /// Un numero oltre la fine della partita mostra l'ultima posizione.
    pub fn go_to(&mut self, ply: usize) {
        self.cursor = ply.min(self.moves.len());
    }

    /// Il risultato nella posizione mostrata, se la partita lì è finita.
    #[must_use]
    pub fn result(&self) -> Option<GameResult> {
        self.result_at(self.cursor)
    }

    /// Il risultato alla fine della partita (ultima mossa), se è finita.
    #[must_use]
    pub fn final_result(&self) -> Option<GameResult> {
        self.result_at(self.moves.len())
    }

    /// La descrizione completa della partita per l'interfaccia.
    #[must_use]
    pub fn view(&self) -> GameView {
        let last_move = self
            .cursor
            .checked_sub(1)
            .and_then(|index| self.moves.get(index))
            .map(|record| LegalMove {
                from: record.from.clone(),
                to: record.to.clone(),
                uci: record.uci.clone(),
            });

        GameView {
            position: PositionView::from_position(self.current()),
            moves: self.moves.clone(),
            cursor: self.cursor,
            last_move,
            result: self.result(),
            starting_fen: self.starting_fen(),
        }
    }

    /// La posizione mostrata (quella del cursore).
    pub(crate) fn current(&self) -> &Chess {
        &self.positions[self.cursor]
    }

    /// La posizione di partenza.
    fn start(&self) -> &Chess {
        &self.positions[0]
    }

    /// Elimina le mosse dopo il cursore, prima di giocarne una nuova.
    fn truncate_after_cursor(&mut self) {
        self.moves.truncate(self.cursor);
        self.positions.truncate(self.cursor + 1);
        self.hashes.truncate(self.cursor + 1);
    }

    /// Il risultato dopo `ply` mosse, se la partita lì è finita.
    fn result_at(&self, ply: usize) -> Option<GameResult> {
        let position = self.positions.get(ply)?;

        match position.outcome() {
            Outcome::Known(KnownOutcome::Decisive { winner }) => Some(GameResult::decisive(
                Side::from(winner),
                GameEndReason::Checkmate,
            )),
            Outcome::Known(KnownOutcome::Draw) => {
                let reason = if position.is_stalemate() {
                    GameEndReason::Stalemate
                } else {
                    GameEndReason::InsufficientMaterial
                };
                Some(GameResult::draw(reason))
            }
            Outcome::Unknown => {
                let hash = self.hashes.get(ply)?;
                let repetitions = self.hashes[..=ply]
                    .iter()
                    .filter(|earlier| *earlier == hash)
                    .count();
                if repetitions >= REPETITIONS_FOR_DRAW {
                    Some(GameResult::draw(GameEndReason::ThreefoldRepetition))
                } else if position.halfmoves() >= FIFTY_MOVE_RULE_HALFMOVES {
                    Some(GameResult::draw(GameEndReason::FiftyMoveRule))
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Game;
    use crate::game_error::GameError;
    use crate::game_result::GameEndReason;
    use crate::side::Side;

    /// Gioca una serie di mosse UCI, fermandosi al primo errore.
    fn play_all(game: &mut Game, moves: &[&str]) {
        for uci in moves {
            game.play_uci(uci).expect("mossa legale");
        }
    }

    fn last_san(game: &Game) -> String {
        game.moves().last().expect("almeno una mossa").san.clone()
    }

    #[test]
    fn a_new_game_starts_from_the_initial_position() {
        let view = Game::new().view();

        assert_eq!(view.moves, []);
        assert_eq!(view.cursor, 0);
        assert_eq!(view.position.legal_moves.len(), 20);
        assert_eq!(view.result, None);
        assert_eq!(view.last_move, None);
    }

    #[test]
    fn records_a_move_with_its_notation() {
        let mut game = Game::new();
        game.play_uci("e2e4").expect("mossa legale");

        let view = game.view();
        let record = &view.moves[0];
        assert_eq!(record.san, "e4");
        assert_eq!(record.side, Side::White);
        assert_eq!(record.move_number, 1);
        assert_eq!(record.ply, 1);
        assert_eq!(view.cursor, 1);
        assert_eq!(view.position.turn, Side::Black);
        assert_eq!(view.last_move.expect("ultima mossa").uci, "e2e4");
    }

    #[test]
    fn rejects_an_illegal_move() {
        let mut game = Game::new();

        assert_eq!(
            game.play_uci("e2e5"),
            Err(GameError::IllegalMove("e2e5".to_owned()))
        );
        assert_eq!(game.moves(), []);
    }

    #[test]
    fn castles_when_allowed() {
        let mut game = Game::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").expect("FEN valida");
        game.play_uci("e1g1").expect("arrocco corto legale");

        assert_eq!(last_san(&game), "O-O");
    }

    #[test]
    fn cannot_castle_after_the_king_has_moved() {
        let mut game = Game::from_fen("4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1").expect("FEN valida");
        play_all(&mut game, &["e1f1", "e8e7", "f1e1", "e7e8"]);

        assert!(matches!(
            game.play_uci("e1g1"),
            Err(GameError::IllegalMove(_))
        ));
    }

    #[test]
    fn captures_en_passant() {
        let mut game = Game::from_fen("4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 1").expect("FEN valida");
        game.play_uci("e5d6").expect("presa en passant legale");

        assert_eq!(last_san(&game), "exd6");
        assert_eq!(game.view().position.board_fen, "4k3/8/3P4/8/8/8/8/4K3");
        assert!(game.moves()[0].capture);
    }

    #[test]
    fn marks_which_moves_are_captures() {
        let mut game = Game::new();
        play_all(&mut game, &["e2e4", "d7d5", "e4d5"]);

        let captures: Vec<bool> = game.moves().iter().map(|record| record.capture).collect();
        assert_eq!(captures, [false, false, true]);
    }

    #[test]
    fn promotes_to_the_chosen_piece() {
        let mut game = Game::from_fen("8/4P3/8/7k/8/8/8/4K3 w - - 0 1").expect("FEN valida");
        game.play_uci("e7e8n").expect("promozione legale");

        assert_eq!(last_san(&game), "e8=N");
    }

    #[test]
    fn recognises_checkmate() {
        let mut game = Game::new();
        play_all(&mut game, &["f2f3", "e7e5", "g2g4", "d8h4"]);

        let result = game.result().expect("partita finita");
        assert_eq!(result.winner, Some(Side::Black));
        assert_eq!(result.reason, GameEndReason::Checkmate);
        assert_eq!(result.score, "0-1");
        assert_eq!(last_san(&game), "Qh4#");
    }

    #[test]
    fn no_moves_after_the_game_is_over() {
        let mut game = Game::new();
        play_all(&mut game, &["f2f3", "e7e5", "g2g4", "d8h4"]);

        assert_eq!(game.play_uci("a2a3"), Err(GameError::GameOver));
    }

    #[test]
    fn recognises_stalemate() {
        let game = Game::from_fen("7k/5Q2/6K1/8/8/8/8/8 b - - 0 1").expect("FEN valida");

        let result = game.result().expect("partita finita");
        assert_eq!(result.winner, None);
        assert_eq!(result.reason, GameEndReason::Stalemate);
    }

    #[test]
    fn recognises_insufficient_material() {
        let game = Game::from_fen("8/8/8/8/8/8/8/K6k w - - 0 1").expect("FEN valida");

        assert_eq!(
            game.result().expect("partita finita").reason,
            GameEndReason::InsufficientMaterial
        );
    }

    #[test]
    fn recognises_threefold_repetition() {
        let mut game = Game::new();
        play_all(
            &mut game,
            &[
                "g1f3", "g8f6", "f3g1", "f6g8", "g1f3", "g8f6", "f3g1", "f6g8",
            ],
        );

        assert_eq!(
            game.result().expect("partita finita").reason,
            GameEndReason::ThreefoldRepetition
        );
    }

    #[test]
    fn recognises_the_fifty_move_rule() {
        let mut game = Game::from_fen("4k3/8/8/8/8/8/8/R3K3 w - - 99 80").expect("FEN valida");
        game.play_uci("a1a2").expect("mossa legale");

        assert_eq!(
            game.result().expect("partita finita").reason,
            GameEndReason::FiftyMoveRule
        );
    }

    #[test]
    fn going_back_shows_an_earlier_position() {
        let mut game = Game::new();
        play_all(&mut game, &["e2e4", "e7e5"]);

        game.go_to(1);

        let view = game.view();
        assert_eq!(view.cursor, 1);
        assert_eq!(view.position.turn, Side::Black);
        assert_eq!(view.moves.len(), 2);
    }

    #[test]
    fn a_move_from_an_earlier_position_replaces_the_rest() {
        let mut game = Game::new();
        play_all(&mut game, &["e2e4", "e7e5"]);

        game.go_to(1);
        game.play_uci("c7c5").expect("mossa legale");

        assert_eq!(game.moves().len(), 2);
        assert_eq!(last_san(&game), "c5");
    }

    #[test]
    fn undo_takes_back_the_last_move() {
        let mut game = Game::new();
        play_all(&mut game, &["e2e4", "e7e5"]);

        game.undo();

        assert_eq!(game.moves().len(), 1);
        assert_eq!(game.view().cursor, 1);
    }

    #[test]
    fn numbers_moves_correctly_when_black_starts() {
        let mut game = Game::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1")
            .expect("FEN valida");
        game.play_uci("e7e5").expect("mossa legale");

        let record = &game.moves()[0];
        assert_eq!(record.side, Side::Black);
        assert_eq!(record.move_number, 1);
    }
}

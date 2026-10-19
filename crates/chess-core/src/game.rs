//! La partita: l'albero delle mosse (linea principale e varianti), le
//! posizioni, la navigazione e la fine partita.

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
use crate::move_tree::{MoveNode, MoveTree};
use crate::position_view::PositionView;
use crate::setup::parse_fen;
use crate::side::Side;

/// Numero di mezze mosse senza catture né mosse di pedone che fa scattare
/// la regola delle 50 mosse (50 mosse per parte = 100 mezze mosse).
const FIFTY_MOVE_RULE_HALFMOVES: u32 = 100;

/// Quante volte deve presentarsi la stessa posizione per la patta per ripetizione.
const REPETITIONS_FOR_DRAW: usize = 3;

/// Il numero che identifica una mossa nell'albero.
pub type NodeId = usize;

/// Una mossa dell'albero, con la posizione che produce.
#[derive(Debug, Clone)]
struct Node {
    /// La mossa, con la sua notazione.
    record: MoveRecord,
    /// La posizione dopo la mossa.
    position: Chess,
    /// L'impronta della posizione, per riconoscere le ripetizioni.
    hash: Zobrist64,
    /// La mossa precedente (`None` se parte dalla posizione iniziale).
    parent: Option<NodeId>,
    /// Le mosse che la seguono: prima la continuazione principale, poi le varianti.
    children: Vec<NodeId>,
}

/// Una partita a scacchi.
///
/// Le mosse formano un **albero**: da ogni posizione la prima continuazione
/// è la linea principale, le altre sono varianti. Se si gioca una mossa da
/// una posizione passata, il seguito non si perde: nasce una variante (se la
/// stessa mossa c'era già, la si segue).
///
/// In ogni momento si guarda una **linea**: il cammino dalla partenza fino in
/// fondo a una variante (o alla linea principale). Le posizioni e le mosse
/// della linea sono tenute pronte (così si scorre senza ricalcolare nulla),
/// insieme a un "cursore" che indica quale posizione è mostrata.
#[derive(Debug, Clone)]
pub struct Game {
    /// La posizione di partenza e la sua impronta.
    start: Chess,
    start_hash: Zobrist64,
    /// Tutte le mosse mai giocate (quelle tolte restano qui, ma non sono più raggiungibili).
    nodes: Vec<Node>,
    /// Le prime mosse dalla posizione di partenza.
    roots: Vec<NodeId>,
    /// La linea guardata: le mosse, dalla prima all'ultima.
    line: Vec<NodeId>,
    /// La linea guardata: `positions[0]` è la partenza, `positions[i]` quella dopo `i` mosse.
    positions: Vec<Chess>,
    /// Le impronte delle posizioni della linea (stesso indice di `positions`).
    hashes: Vec<Zobrist64>,
    /// Le mosse della linea guardata.
    moves: Vec<MoveRecord>,
    /// Quante mosse della linea sono state giocate fino alla posizione mostrata.
    cursor: usize,
    /// Il risultato deciso fuori dalla scacchiera (tempo scaduto o abbandono),
    /// valido alla fine della linea principale; `None` se non c'è.
    adjudication: Option<GameResult>,
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
            positions: vec![start.clone()],
            hashes: vec![hash],
            start,
            start_hash: hash,
            nodes: Vec::new(),
            roots: Vec::new(),
            line: Vec::new(),
            moves: Vec::new(),
            cursor: 0,
            adjudication: None,
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

    /// Le mosse della linea guardata, dalla prima all'ultima.
    #[must_use]
    pub fn moves(&self) -> &[MoveRecord] {
        &self.moves
    }

    /// Le mosse della linea principale (la partita "vera"), dalla prima all'ultima.
    #[must_use]
    pub fn mainline(&self) -> Vec<MoveRecord> {
        self.mainline_ids()
            .into_iter()
            .map(|id| self.nodes[id].record.clone())
            .collect()
    }

    /// Quante mosse della linea guardata sono state giocate fino alla posizione mostrata.
    #[must_use]
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// `true` se è mostrata l'ultima posizione della linea guardata.
    #[must_use]
    pub fn is_at_end(&self) -> bool {
        self.cursor == self.moves.len()
    }

    /// `true` se è mostrata l'ultima posizione della linea principale.
    #[must_use]
    pub fn is_at_mainline_end(&self) -> bool {
        let mainline = self.mainline_ids();
        self.current_node() == mainline.last().copied()
    }

    /// Chi deve muovere nella posizione mostrata.
    #[must_use]
    pub fn turn(&self) -> Side {
        Side::from(self.current().turn())
    }

    /// Chi deve muovere alla fine della linea principale.
    #[must_use]
    pub fn last_turn(&self) -> Side {
        Side::from(self.mainline_end().turn())
    }

    /// La posizione mostrata (quella del cursore) in notazione FEN.
    #[must_use]
    pub fn fen(&self) -> String {
        Fen::from_position(self.current(), EnPassantMode::Legal).to_string()
    }

    /// La posizione di partenza in notazione FEN.
    #[must_use]
    pub fn starting_fen(&self) -> String {
        Fen::from_position(&self.start, EnPassantMode::Legal).to_string()
    }

    /// Gioca una mossa scritta in notazione UCI (es. `"e2e4"`, `"e7e8q"`)
    /// nella posizione mostrata. Se la posizione non è l'ultima della linea,
    /// il seguito resta: la mossa diventa una variante (o, se c'era già, la si segue).
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
        let parent = self.current_node();
        let before = self.current().clone();
        let mut after = before.clone();
        let capture = chess_move.is_capture();
        // Calcola la notazione SAN (con + o # finale) e intanto gioca la mossa.
        let san = SanPlus::from_move_and_play_unchecked(&mut after, chess_move).to_string();
        let uci = UciMove::from_standard(chess_move).to_string();

        // La stessa mossa c'è già: la si segue, senza crearne una nuova.
        let existing = self
            .children(parent)
            .iter()
            .copied()
            .find(|id| self.nodes[*id].record.uci == uci);
        let id = existing.unwrap_or_else(|| {
            let (from, to) = (
                uci.get(0..2).unwrap_or_default().to_owned(),
                uci.get(2..4).unwrap_or_default().to_owned(),
            );
            let id = self.nodes.len();
            self.nodes.push(Node {
                record: MoveRecord {
                    ply: self.cursor + 1,
                    move_number: before.fullmoves().get(),
                    side: Side::from(before.turn()),
                    san,
                    uci: uci.clone(),
                    from,
                    to,
                    capture,
                },
                hash: after.zobrist_hash(EnPassantMode::Legal),
                position: after,
                parent,
                children: Vec::new(),
            });
            // Una nuova mossa in fondo alla linea principale ne cambia la fine:
            // un abbandono o un tempo scaduto di prima non valgono più.
            if self.is_mainline(parent) && self.children(parent).is_empty() {
                self.adjudication = None;
            }
            self.children_mut(parent).push(id);
            id
        });
        self.show_node(Some(id));
    }

    /// Ritira l'ultima mossa della linea principale e mostra la nuova fine della
    /// linea principale. Se non ci sono mosse non fa nulla.
    pub fn undo(&mut self) {
        if let Some(last) = self.mainline_ids().last().copied() {
            let parent = self.nodes[last].parent;
            self.children_mut(parent).retain(|child| *child != last);
        }
        self.adjudication = None;
        self.show_mainline_end();
    }

    /// Il giocatore `side` abbandona: vince l'avversario.
    ///
    /// # Errors
    ///
    /// [`GameError::GameOver`] se la partita è già finita.
    pub fn resign(&mut self, side: Side) -> Result<(), GameError> {
        self.adjudicate(GameResult::decisive(
            side.opponent(),
            GameEndReason::Resignation,
        ))
    }

    /// Il giocatore `loser` ha finito il tempo sull'orologio.
    ///
    /// Di regola vince l'avversario; ma se l'avversario non ha più pezzi
    /// sufficienti per dare scacco matto (ad esempio ha solo il re), la
    /// partita è patta.
    ///
    /// # Errors
    ///
    /// [`GameError::GameOver`] se la partita è già finita.
    pub fn time_forfeit(&mut self, loser: Side) -> Result<(), GameError> {
        let winner = loser.opponent();
        let result = if self.mainline_end().has_insufficient_material(winner.into()) {
            GameResult::draw(GameEndReason::TimeoutVsInsufficientMaterial)
        } else {
            GameResult::decisive(winner, GameEndReason::Timeout)
        };
        self.adjudicate(result)
    }

    /// Chiude la partita con un risultato deciso fuori dalla scacchiera e
    /// mostra la fine della linea principale.
    fn adjudicate(&mut self, result: GameResult) -> Result<(), GameError> {
        if self.final_result().is_some() {
            return Err(GameError::GameOver);
        }
        self.adjudication = Some(result);
        self.show_mainline_end();
        Ok(())
    }

    /// Mostra la posizione dopo `ply` mosse della linea guardata (0 = partenza).
    /// Un numero oltre la fine della linea mostra la sua ultima posizione.
    pub fn go_to(&mut self, ply: usize) {
        self.cursor = ply.min(self.moves.len());
    }

    /// Mostra la posizione dopo la mossa indicata dell'albero (`None` = partenza),
    /// guardando la linea che passa da lì.
    ///
    /// # Errors
    ///
    /// [`GameError::UnknownMove`] se la mossa non è (più) nell'albero.
    pub fn go_to_node(&mut self, node: Option<NodeId>) -> Result<(), GameError> {
        if node.is_some_and(|id| !self.is_reachable(id)) {
            return Err(GameError::UnknownMove);
        }
        self.show_node(node);
        Ok(())
    }

    /// Mostra l'ultima posizione della linea principale.
    pub fn go_to_mainline_end(&mut self) {
        self.show_mainline_end();
    }

    /// Fa diventare linea principale la variante che passa dalla mossa indicata.
    ///
    /// # Errors
    ///
    /// [`GameError::UnknownMove`] se la mossa non è (più) nell'albero.
    pub fn promote(&mut self, node: NodeId) -> Result<(), GameError> {
        if !self.is_reachable(node) {
            return Err(GameError::UnknownMove);
        }
        let mut current = Some(node);
        while let Some(id) = current {
            let parent = self.nodes[id].parent;
            let children = self.children_mut(parent);
            children.retain(|child| *child != id);
            children.insert(0, id);
            current = parent;
        }
        // La fine della linea principale è cambiata: l'abbandono di prima non vale più.
        self.adjudication = None;
        let shown = self.current_node();
        self.show_node(shown);
        Ok(())
    }

    /// Elimina una variante: la mossa indicata e tutto il suo seguito.
    ///
    /// # Errors
    ///
    /// - [`GameError::UnknownMove`] se la mossa non è (più) nell'albero;
    /// - [`GameError::NotAVariation`] se la mossa è della linea principale.
    pub fn delete_variation(&mut self, node: NodeId) -> Result<(), GameError> {
        if !self.is_reachable(node) {
            return Err(GameError::UnknownMove);
        }
        if self.is_mainline(Some(node)) {
            return Err(GameError::NotAVariation);
        }
        let parent = self.nodes[node].parent;
        self.children_mut(parent).retain(|child| *child != node);
        // Se si guardava proprio quella variante, si torna alla mossa da cui partiva.
        if self.line.contains(&node) {
            self.show_node(parent);
        }
        Ok(())
    }

    /// Le mosse che seguono una mossa dell'albero (`None` = la posizione di
    /// partenza): prima la continuazione principale, poi le varianti.
    #[must_use]
    pub fn children(&self, node: Option<NodeId>) -> &[NodeId] {
        match node {
            Some(id) => &self.nodes[id].children,
            None => &self.roots,
        }
    }

    /// La mossa precedente a quella indicata (`None` se parte dalla posizione di partenza).
    #[must_use]
    pub fn parent(&self, node: NodeId) -> Option<NodeId> {
        self.nodes[node].parent
    }

    /// La mossa indicata dell'albero.
    #[must_use]
    pub fn record(&self, node: NodeId) -> &MoveRecord {
        &self.nodes[node].record
    }

    /// La mossa che porta alla posizione mostrata (`None` = posizione di partenza).
    #[must_use]
    pub fn current_node(&self) -> Option<NodeId> {
        self.cursor.checked_sub(1).map(|index| self.line[index])
    }

    /// Il risultato nella posizione mostrata, se la partita lì è finita.
    #[must_use]
    pub fn result(&self) -> Option<GameResult> {
        let is_mainline_end = self.is_at_mainline_end();
        self.result_of(self.cursor, is_mainline_end)
    }

    /// Il risultato alla fine della linea principale, se la partita è finita.
    #[must_use]
    pub fn final_result(&self) -> Option<GameResult> {
        let mainline = self.mainline_ids();
        let mut hashes = vec![self.start_hash];
        hashes.extend(mainline.iter().map(|id| self.nodes[*id].hash));
        outcome(self.mainline_end(), &hashes).or_else(|| self.adjudication.clone())
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
            tree: self.tree(),
        }
    }

    /// La posizione mostrata (quella del cursore).
    pub(crate) fn current(&self) -> &Chess {
        &self.positions[self.cursor]
    }

    /// L'albero delle mosse per l'interfaccia, in ordine di lettura.
    fn tree(&self) -> MoveTree {
        let mainline: Vec<NodeId> = self.mainline_ids();
        let mut nodes = Vec::new();
        let mut pending: Vec<NodeId> = self.roots.iter().rev().copied().collect();
        while let Some(id) = pending.pop() {
            let node = &self.nodes[id];
            pending.extend(node.children.iter().rev().copied());
            nodes.push(MoveNode {
                id: to_u32(id),
                record: node.record.clone(),
                parent: node.parent.map(to_u32),
                children: node.children.iter().copied().map(to_u32).collect(),
                mainline: mainline.contains(&id),
            });
        }
        MoveTree {
            nodes,
            roots: self.roots.iter().copied().map(to_u32).collect(),
            current: self.current_node().map(to_u32),
        }
    }

    /// Le mosse della linea principale: sempre la prima continuazione.
    fn mainline_ids(&self) -> Vec<NodeId> {
        let mut ids = Vec::new();
        let mut next = self.roots.first().copied();
        while let Some(id) = next {
            ids.push(id);
            next = self.nodes[id].children.first().copied();
        }
        ids
    }

    /// L'ultima posizione della linea principale.
    fn mainline_end(&self) -> &Chess {
        self.mainline_ids()
            .last()
            .map_or(&self.start, |id| &self.nodes[*id].position)
    }

    /// `true` se la mossa (o la partenza, con `None`) sta sulla linea principale.
    fn is_mainline(&self, node: Option<NodeId>) -> bool {
        node.is_none_or(|id| self.mainline_ids().contains(&id))
    }

    /// `true` se la mossa si raggiunge dalla partenza (non è stata tolta).
    fn is_reachable(&self, node: NodeId) -> bool {
        let Some(found) = self.nodes.get(node) else {
            return false;
        };
        let mut current = (node, found.parent);
        loop {
            if !self.children(current.1).contains(&current.0) {
                return false;
            }
            match current.1 {
                Some(parent) => current = (parent, self.nodes[parent].parent),
                None => return true,
            }
        }
    }

    /// Le mosse che seguono una mossa (o la partenza), da modificare.
    fn children_mut(&mut self, node: Option<NodeId>) -> &mut Vec<NodeId> {
        match node {
            Some(id) => &mut self.nodes[id].children,
            None => &mut self.roots,
        }
    }

    /// Guarda la linea che passa dalla mossa indicata (fino in fondo, seguendo
    /// le continuazioni principali) e mostra la posizione dopo quella mossa.
    fn show_node(&mut self, node: Option<NodeId>) {
        let mut path = Vec::new();
        let mut current = node;
        while let Some(id) = current {
            path.push(id);
            current = self.nodes[id].parent;
        }
        path.reverse();
        self.cursor = path.len();
        let mut next = self.children(node).first().copied();
        while let Some(id) = next {
            path.push(id);
            next = self.nodes[id].children.first().copied();
        }
        self.set_line(path);
    }

    /// Guarda la linea principale e ne mostra l'ultima posizione.
    fn show_mainline_end(&mut self) {
        let mainline = self.mainline_ids();
        self.cursor = mainline.len();
        self.set_line(mainline);
    }

    /// Prepara posizioni, impronte e mosse della linea indicata.
    fn set_line(&mut self, line: Vec<NodeId>) {
        self.positions = std::iter::once(self.start.clone())
            .chain(line.iter().map(|id| self.nodes[*id].position.clone()))
            .collect();
        self.hashes = std::iter::once(self.start_hash)
            .chain(line.iter().map(|id| self.nodes[*id].hash))
            .collect();
        self.moves = line
            .iter()
            .map(|id| self.nodes[*id].record.clone())
            .collect();
        self.cursor = self.cursor.min(line.len());
        self.line = line;
    }

    /// Il risultato dopo `ply` mosse della linea guardata, se la partita lì è finita.
    fn result_of(&self, ply: usize, is_mainline_end: bool) -> Option<GameResult> {
        let position = self.positions.get(ply)?;
        outcome(position, &self.hashes[..=ply]).or_else(|| {
            if is_mainline_end {
                self.adjudication.clone()
            } else {
                None
            }
        })
    }
}

/// Il risultato di una posizione secondo le regole (matto, stallo, patte), se c'è.
///
/// - Riceve: la posizione e le impronte di tutte le posizioni fino a lei
///   (l'ultima è la sua), per la patta per ripetizione.
fn outcome(position: &Chess, hashes: &[Zobrist64]) -> Option<GameResult> {
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
            let hash = hashes.last()?;
            let repetitions = hashes.iter().filter(|earlier| *earlier == hash).count();
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

/// Un numero dell'albero per l'interfaccia (le partite non arrivano mai a 4 miliardi di mosse).
fn to_u32(id: NodeId) -> u32 {
    u32::try_from(id).unwrap_or(u32::MAX)
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
    fn a_move_from_an_earlier_position_becomes_a_variation() {
        let mut game = Game::new();
        play_all(&mut game, &["e2e4", "e7e5", "g1f3"]);

        game.go_to(1);
        game.play_uci("c7c5").expect("mossa legale");

        assert_eq!(last_san(&game), "c5");
        let mainline: Vec<String> = game
            .mainline()
            .into_iter()
            .map(|record| record.san)
            .collect();
        assert_eq!(mainline, ["e4", "e5", "Nf3"]);
        let tree = game.view().tree;
        assert_eq!(tree.nodes.len(), 4);
        let c5 = tree
            .nodes
            .iter()
            .find(|node| node.record.san == "c5")
            .expect("variante");
        assert!(!c5.mainline);
        assert_eq!(tree.current, Some(c5.id));
    }

    #[test]
    fn playing_an_existing_move_follows_it() {
        let mut game = Game::new();
        play_all(&mut game, &["e2e4", "e7e5"]);

        game.go_to(0);
        game.play_uci("e2e4").expect("mossa legale");

        assert_eq!(game.view().tree.nodes.len(), 2);
        assert_eq!(game.moves().len(), 2);
        assert_eq!(game.cursor(), 1);
    }

    #[test]
    fn a_variation_can_be_promoted_or_deleted() {
        let mut game = Game::new();
        play_all(&mut game, &["e2e4", "e7e5"]);
        game.go_to(1);
        game.play_uci("c7c5").expect("mossa legale");
        let c5 = game.current_node().expect("mossa mostrata");

        game.promote(c5).expect("variante");
        let mainline: Vec<String> = game
            .mainline()
            .into_iter()
            .map(|record| record.san)
            .collect();
        assert_eq!(mainline, ["e4", "c5"]);

        let e5 = game.children(Some(0))[1];
        assert_eq!(game.delete_variation(c5), Err(GameError::NotAVariation));
        game.delete_variation(e5).expect("variante");
        assert_eq!(game.view().tree.nodes.len(), 2);
        assert_eq!(game.go_to_node(Some(e5)), Err(GameError::UnknownMove));
    }

    #[test]
    fn the_end_of_the_mainline_decides_the_game() {
        let mut game = Game::new();
        play_all(&mut game, &["e2e4", "e7e5"]);
        game.go_to(1);
        game.play_uci("c7c5").expect("mossa legale");

        assert!(!game.is_at_mainline_end());
        assert_eq!(game.last_turn(), Side::White);
        game.go_to_mainline_end();
        assert!(game.is_at_mainline_end());
        assert_eq!(game.moves()[1].san, "e5");
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

    #[test]
    fn tells_whose_turn_it_is_at_the_cursor_and_at_the_end() {
        let mut game = Game::new();
        play_all(&mut game, &["e2e4"]);
        game.go_to(0);

        assert_eq!(game.cursor(), 0);
        assert!(!game.is_at_end());
        assert_eq!(game.fen(), game.starting_fen());
        assert_eq!(game.turn(), Side::White);
        assert_eq!(game.last_turn(), Side::Black);
    }

    #[test]
    fn resigning_ends_the_game() {
        let mut game = Game::new();
        play_all(&mut game, &["e2e4"]);
        game.go_to(0);

        game.resign(Side::Black).expect("la partita non è finita");

        let result = game.result().expect("partita finita");
        assert_eq!(result.winner, Some(Side::White));
        assert_eq!(result.reason, GameEndReason::Resignation);
        assert_eq!(game.view().cursor, 1);
        assert_eq!(game.play_uci("e7e5"), Err(GameError::GameOver));
        assert_eq!(game.resign(Side::White), Err(GameError::GameOver));
    }

    #[test]
    fn running_out_of_time_loses_the_game() {
        let mut game = Game::new();

        game.time_forfeit(Side::White)
            .expect("la partita non è finita");

        let result = game.final_result().expect("partita finita");
        assert_eq!(result.winner, Some(Side::Black));
        assert_eq!(result.reason, GameEndReason::Timeout);
    }

    #[test]
    fn running_out_of_time_is_a_draw_if_the_opponent_cannot_mate() {
        // Il nero ha solo il re: non può dare matto, quindi il tempo del bianco
        // scaduto non gli dà la vittoria.
        let mut game = Game::from_fen("4k3/8/8/8/8/8/4P3/4K3 w - - 0 1").expect("FEN valida");

        game.time_forfeit(Side::White)
            .expect("la partita non è finita");

        let result = game.final_result().expect("partita finita");
        assert_eq!(result.winner, None);
        assert_eq!(result.reason, GameEndReason::TimeoutVsInsufficientMaterial);
    }

    #[test]
    fn a_variation_keeps_the_resignation_but_undo_cancels_it() {
        let mut game = Game::new();
        play_all(&mut game, &["e2e4", "e7e5"]);
        game.resign(Side::White).expect("la partita non è finita");

        game.go_to(1);
        game.play_uci("c7c5")
            .expect("mossa legale da una posizione passata");
        assert_eq!(game.result(), None);
        assert!(game.final_result().is_some());

        game.undo();
        assert_eq!(game.final_result(), None);
    }
}

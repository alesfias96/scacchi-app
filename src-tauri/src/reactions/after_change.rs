//! Il punto unico da chiamare dopo ogni cambiamento della partita.

use std::time::Instant;

use tauri::{AppHandle, Manager};

use crate::reactions::computer_move::play_computer_move;
use crate::reactions::flag_timer::schedule_flag_check;
use crate::reactions::follow_analysis::follow_analysis;
use crate::state::database_state::DatabaseState;
use crate::state::engines_state::EnginesState;
use crate::state::session_state::SessionState;

/// Avvia tutto ciò che deve succedere dopo un cambiamento della partita.
///
/// Legge dalla partita cosa serve (richiesta per il computer, scadenza
/// dell'orologio, posizione da analizzare) e avvia i compiti corrispondenti,
/// ognuno per conto suo: questa funzione non aspetta nulla. Mette anche in
/// coda il salvataggio della partita (se ha almeno una mossa): la fotografia
/// si scatta mentre la partita è "chiusa a chiave", così i salvataggi
/// arrivano al database sempre nell'ordine giusto.
pub fn after_change(app: &AppHandle) {
    let now = Instant::now();
    let sessions = app.state::<SessionState>();
    let database = app.state::<DatabaseState>();
    let (request, flag_in) = sessions.with_session(|session| {
        // Si salva solo se la partita è cambiata davvero (non per uno
        // spostamento nella lista delle mosse) e ha almeno una mossa.
        if session.has_moves() && database.claim_revision(session.revision()) {
            database.queue_save(session.snapshot(now));
        }
        (session.engine_request(now), session.time_until_flag(now))
    });

    if let Some(request) = request {
        // Se la stessa richiesta è già partita (la partita non è cambiata), il
        // computer sta già pensando: non lo si interrompe.
        if sessions.claim_engine_request(request.token) {
            tauri::async_runtime::spawn(play_computer_move(app.clone(), request));
        }
    } else if let Some(player) = app.state::<EnginesState>().try_running_player() {
        // Non tocca al computer: se stava pensando (ad esempio prima di una
        // mossa ritirata) lo si ferma subito, così non spreca il processore.
        // Il comando parte qui e non in un compito separato, così arriva al
        // motore sempre prima di un'eventuale richiesta successiva.
        player.stop();
    }
    schedule_flag_check(app, flag_in);
    follow_analysis(app);
}

# src/app/theme-controller — Applicazione del tema

Fa sì che il tema scelto nelle Impostazioni valga in tutta l'app. Non disegna nulla:
scrive il nome del tema sull'elemento `<html>` (attributo `data-theme`), e i fogli di stile
in `src/styles/themes/` cambiano colori e font di conseguenza.

## Contenuto

| File                       | Cosa fa                                                                               |
| -------------------------- | ------------------------------------------------------------------------------------- |
| `ThemeController.tsx`      | Il componente che applica il tema e lo aggiorna appena cambia                         |
| `ThemeController.test.tsx` | I suoi test, compreso il tema automatico con il sistema in modalità chiara e scura    |
| `resolveTheme.ts`          | Decide il tema da mostrare: con "Automatico" sceglie Giorno o Sera in base al sistema |
| `resolveTheme.test.ts`     | I suoi test                                                                           |

# components/button — Il pulsante dell'app

Tutti i pulsanti dell'app usano questo componente, così hanno lo stesso aspetto
in ogni sezione e in ogni tema.

| Aspetto (`variant`) | Quando si usa                                            |
| ------------------- | -------------------------------------------------------- |
| `primary`           | L'azione principale (pieno, colore d'accento del tema)   |
| `secondary`         | Le azioni comuni (tenue) — è il predefinito              |
| `icon`              | Pulsante quadrato con la sola icona (serve `aria-label`) |

| File                | Cosa contiene          |
| ------------------- | ---------------------- |
| `Button.tsx`        | Il componente          |
| `Button.module.css` | L'aspetto dei tre tipi |
| `Button.test.tsx`   | I test                 |

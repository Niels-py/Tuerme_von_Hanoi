fn main() {
    let max_len = 4;

    // variablen initialisieren
    let mut board: [Vec<Scheibe>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    let reader = std::io::stdin();
    // einen Turm machen
    for i in 0..max_len {
        board[0].push(Scheibe::new(max_len - i));
    }

    // Die Spielbeschreibung (Formatierung nicht ändern)
    println!("Die Türme von Hanoi. Von Niels Haupt.\n\n
Das Ziel ist es den Stapel auf eine andere Stange zu bringen, so das von oben nach unten die Scheiben immer größer werden.\n
Du darfst dabei keine Scheibe auf eine kleinere Scheibe legen.\n\n
Die Scheiben verschiebst du indem du erst die Nummer des Stapels angibst von dem du die Scheibe heben möchtest, dann eine Leerstele eingibst und darauf den Stapel eingibst auf den du die Scheibe legen möchtest.\n
Hier ein Beispiel wenn du die Scheibe von Stapel 3 auf Stapel 1 verschieben möchtest:\n\n3 1\n\nViel Spaß\n\n");

    // der GAME LOOP
    loop {
        print_board(&board, max_len as usize);

        // Die Spielereingabe bekommen
        let mut input = String::new();
        match reader.read_line(&mut input) {
            Ok(_) => {}
            Err(e) => {
                println!("Gib zwei Ziffern von 1 bis 3 mit einer Leerstelle dazwischen an.\n");
                println!("ERROR: {} \n", e);
                continue;
            }
        }

        // Die Eingebe in Nummern umwandeln
        let arg: Vec<usize> = input
            .trim()
            .split(" ")
            .map(|x| {
                x.parse::<usize>()
                    .expect("\n\nDu musst Zahlen eingeben. Zahlen!\n\n")
            })
            .collect();

        // testen, ob die Nummern zu Stapeln Zeigen
        if arg.len() > 1
            && arg[1] > 0
            && arg[1] < 4
            && arg[1] > 0
            && arg[1] < 4
            && board[arg[0] - 1].len() > 0
        {
            //testet, ob da keine kleinere Scheibe liegt
            if board[arg[1] - 1].len() == 0
                || board[arg[0] - 1].last().unwrap().width < board[arg[1] - 1].last().unwrap().width
            {
                let temp = board[arg[0] - 1].pop().unwrap();
                board[arg[1] - 1].push(temp);
            } else {
                println!("Beachte die Regeln!\nDu darfst keine Scheibe auf eine kleinere Scheibe legen!\nSonst wäre es doch zu einfach.\n")
            }
        } else {
            println!("Gebe zwei Ziffern an die mit einem leerzeichen getrennt sind.\nDabei ist die erste Ziffer der Stapel von dem du die Scheibe nimmst.\nUnd die zweite Ziffer ist der Stapel auf den du die Scheibe ablegen möchtest.\n");
        }
        if won(&board, max_len as usize) {
            break;
        }
    }
    print_board(&board, max_len as usize);

    println!("\nDu hast gewonnen!");
}

// tested, ob man gewonnen hat
fn won(board: &[Vec<Scheibe>; 3], max_len: usize) -> bool {
    if board[2].len() == max_len || board[1].len() == max_len {
        return true;
    }
    false
}

// Die CLI Ausgabe
fn print_board(board: &[Vec<Scheibe>; 3], max_len: usize) {
    println!(
        "{}\x1b[95m{}\x1b[0m\n{}01{}02{}03\n",
        form_board(board, max_len),
        "_".repeat(max_len * 6 + 6),
        " ".repeat(max_len),
        " ".repeat(max_len * 2),
        " ".repeat(max_len * 2)
    );
}

// gibt einen String mit den Stapeln zurrück
fn form_board(board: &[Vec<Scheibe>; 3], max_len: usize) -> String {
    let mut output: String = String::new();

    for i in 0..max_len {
        for j in 0..3 {
            if max_len - i <= board[j].len() {
                output.push_str(board[j][max_len - 1 - i].draw(max_len as u8).as_str())
            } else {
                output.push_str(" ".repeat(max_len).as_str());
                output.push_str("\x1b[93m||\x1b[0m");
                output.push_str(" ".repeat(max_len).as_str());
            }
        }
        output.push('\n')
    }
    output
}

// Wir sollten es ja Objektorientiert machen
// Ich hoffe das reicht an Objekten
#[derive(Clone, Copy, Debug)]
struct Scheibe {
    width: u8,
}
impl Scheibe {
    pub fn new(width: u8) -> Self {
        Scheibe { width: width }
    }

    pub fn draw(&self, max_len: u8) -> String {
        let mut string: String = String::new();
        string.push(' ');

        for _ in 0..max_len - self.width {
            string.push(' ');
        }
        string.push_str("\x1b[92m");
        for _ in 0..self.width {
            string.push_str("#");
        }
        string.push_str("\x1b[0m");

        // featur, dass die länge angezeigt wird

        //if self.width < 10 {
        //    string.push('0');
        //    string.push_str(self.width.to_string().as_str())
        //} else {
        //    string.push_str(self.width.to_string().as_str());
        //}

        string.push_str("\x1b[91m");
        for _ in 0..self.width {
            string.push_str("#");
        }
        string.push_str("\x1b[0m");

        for _ in 0..max_len - self.width {
            string.push(' ');
        }

        string.push(' ');
        string
    }
}

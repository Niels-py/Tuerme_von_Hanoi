fn main() {
    // variablen initialisieren
    let mut board: [Vec<Scheibe>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    let reader = std::io::stdin();
    // einen Turm machen
    for i in 0..10 {
        board[0].push(Scheibe::new(10 - i));
    }

    // Die Spielbeschreibung (Formatierung nicht ändern)
    println!("Die Türme von Hanoi. Von Niels Haupt.\n\n
Das Ziel ist es den Stapel auf eine andere Stange zu bringen, so das von oben nach unten die Scheiben immer größer werden.\n
Du darfst dabei keine Scheibe auf eine kleinere Scheibe legen.\n\n
Die Scheiben verschiebst du indem du erst die Nummer des Stapels angibst von dem du die Scheibe heben möchtest, dann eine Leerstele eingibst und darauf den Stapel eingibst auf den du die Scheibe legen möchtest.\n
Hier ein Beispiel wenn du die Scheibe von Stapel 3 auf Stapel 1 verschieben möchtest:\n\n3 1\n\nViel Spaß\n\n");

    // der GAME LOOP
    loop {
        // Die CLI Ausgabe
        println!(
            "{}{}\n          01{}02{}03\n",
            print_board(&board),
            "_".repeat(68),
            " ".repeat(20),
            " ".repeat(20)
        );

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
                println!("Beachte die Regeln!\nDu darfst keine Scheibe auf eine kleinere Scheibe legen!\nSonst wäre es doch zu einfach.")
            }
        } else {
            println!("Gebe zwei Ziffern an die mit einem leerzeichen getrennt sind.\nDabei ist die erste Ziffer der Stapel von dem du die Scheibe nimmst.\nUnd die zweite Ziffer ist der Stapel auf den du die Scheibe ablegen möchtest.");
        }
    }
}

// gibt einen String mit den Türmen zurrück
fn print_board(board: &[Vec<Scheibe>; 3]) -> String {
    let mut output: String = String::new();

    for i in 0..10 {
        for j in 0..3 {
            if 10 - i <= board[j].len() {
                output.push_str(board[j][9 - i].draw().as_str())
            } else {
                output.push_str(" ".repeat(10).as_str());
                output.push_str("||");
                output.push_str(" ".repeat(10).as_str());
            }
        }
        output.push('\n')
    }
    output
}

// Wir sollten es ja Objektorientiert machen
// Ich hoffe das reicht an Objekten
#[derive(Clone, Copy)]
struct Scheibe {
    width: i32,
}
impl Scheibe {
    pub fn new(width: i32) -> Self {
        Scheibe { width: width }
    }
    pub fn draw(&self) -> String {
        let mut string: String = String::new();
        string.push(' ');

        for _ in 0..10 - self.width {
            string.push(' ');
        }
        for _ in 0..self.width * 2 {
            string.push('#');
        }
        for _ in 0..10 - self.width {
            string.push(' ');
        }

        string.push(' ');
        string
    }
}

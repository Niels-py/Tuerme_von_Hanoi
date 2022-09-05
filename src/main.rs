fn main() {
    let max_len: u8;

    if let Some(len) = std::env::args().nth(1) {
        max_len = len.parse::<u8>().unwrap()
    } else {
        max_len = 10;
    }
    let autosolve;
    if let Some(_) = std::env::args().nth(2) {
        autosolve = true;
    } else {
        autosolve = false;
    }
    let ausgeben;
    if let Some(_) = std::env::args().nth(3) {
        ausgeben = true;
    } else {
        ausgeben = false;
    }

    // variablen initialisieren
    let mut board: [Vec<u8>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    let mut versuche: u128 = 0;

    // einen Turm machen
    for i in 0..max_len {
        board[0].push(max_len - i);
    }

    // Die Spielbeschreibung (Formatierung nicht ändern)
    println!("Die Türme von Hanoi. Von Niels Haupt.\n\n
Das Ziel ist es den Stapel auf eine andere Stange zu bringen, so das von oben nach unten die Scheiben immer größer werden.\n
Du darfst dabei keine Scheibe auf eine kleinere Scheibe legen.\n\n
Die Scheiben verschiebst du indem du erst die Nummer des Stapels angibst von dem du die Scheibe heben möchtest, dann eine Leerstele eingibst und darauf den Stapel eingibst auf den du die Scheibe legen möchtest.\n
Hier ein Beispiel wenn du die Scheibe von Stapel 3 auf Stapel 1 verschieben möchtest:\n\n3 1\n\nViel Spaß\n\n");

    if autosolve {
        solve(
            1,
            2,
            3,
            max_len,
            &mut board,
            max_len,
            ausgeben,
            &mut versuche,
        );
    } else {
        selber_spielen(&mut board, max_len, &mut versuche)
    }

    println!("\nDu hast gewonnen!");
    println!("Versuche: {}", versuche);
    println!("Optimal: {}", (2 as u128).pow(max_len as u32) - 1)
}

fn selber_spielen(board: &mut [Vec<u8>; 3], max_len: u8, versuche: &mut u128) {
    let reader = std::io::stdin();

    // der GAME LOOP
    loop {
        *versuche += 1;
        print_board(&board, max_len);

        let mut input: String = String::new();

        // Die Spielereingabe bekommen
        let arg: [u8; 2];
        match reader.read_line(&mut input) {
            Ok(_) => {
                let iter: Vec<u8> = input
                    .trim()
                    .split(" ")
                    .map(|x| {
                        x.parse::<u8>()
                            .expect("\n\nDu musst Zahlen eingeben. Zahlen!\n\n")
                    })
                    .collect();
                arg = [iter[0] - 1, iter[1] - 1];

                if arg[0] > 3 || arg[1] > 3 {
                    println!("Gib zwei Ziffern von 1 bis 3 mit einer Leerstelle dazwischen an.\n");
                    continue;
                }
            }

            Err(e) => {
                println!("Gib zwei Ziffern von 1 bis 3 mit einer Leerstelle dazwischen an.\n");
                println!("ERROR: {} \n", e);
                continue;
            }
        }

        //bewegen
        bewegen(arg[0], arg[1], board);

        if won(&board, max_len as u8) {
            break;
        }
        print_board(&board, max_len as u8);
    }
}

// tested, ob man gewonnen hat
fn won(board: &[Vec<u8>; 3], max_len: u8) -> bool {
    if board[2].len() == max_len as usize || board[1].len() == max_len as usize {
        return true;
    }
    false
}

// bewegen von einer scheibe auf einem stapel zu einem anderen Stapel
fn bewegen(von: u8, nach: u8, board: &mut [Vec<u8>; 3]) {
    let arg = [von as usize, nach as usize];
    // testen, ob die Nummern zu Stapeln Zeigen
    //testet, ob da keine kleinere Scheibe liegt
    if board[arg[1] - 1].len() == 0
        || board[arg[0] - 1].last().unwrap() < board[arg[1] - 1].last().unwrap()
    {
        let temp: u8 = board[arg[0] - 1].pop().unwrap();
        board[arg[1] - 1].push(temp);
    } else {
        println!("Beachte die Regeln!\nDu darfst keine Scheibe auf eine kleinere Scheibe legen!\n")
    }
}

// Die CLI Ausgabe
fn print_board(board: &[Vec<u8>; 3], max_len: u8) {
    let mut output: String = String::new();
    let max_len_usize: usize = max_len as usize;

    for i in 0..max_len_usize {
        for j in 0..3 {
            if max_len_usize - i <= board[j].len() {
                output.push_str(
                    format!(
                        "{}\x1b[91m{}\x1b[92m{}\x1b[0m{}",
                        " ".repeat((max_len - board[j][max_len_usize - 1 - i] + 1) as usize),
                        "#".repeat((board[j][max_len_usize - 1 - i]) as usize),
                        "#".repeat((board[j][max_len_usize - 1 - i]) as usize),
                        " ".repeat((max_len - board[j][max_len_usize - 1 - i] + 1) as usize)
                    )
                    .as_str(),
                )
            } else {
                let space = " ".repeat(max_len_usize);
                output.push_str(format!("{}\x1b[93m||\x1b[0m{}", space, space).as_str());
            }
        }
        output.push_str("\n");
    }
    output.push_str(
        format!(
            "\n\x1b[95m{}\x1b[0m\n{}01{}02{}03\n",
            "_".repeat(max_len_usize * 6 + 6),
            " ".repeat(max_len_usize),
            " ".repeat(max_len_usize * 2),
            " ".repeat(max_len_usize * 2),
        )
        .as_str(),
    );
    println!("{}", output,);
}

// algorithmus zum lösen
fn solve(
    p_von: u8,
    p_über: u8,
    p_nach: u8,
    p_höhe: u8,
    board: &mut [Vec<u8>; 3],
    max_len: u8,
    ausgeben: bool,
    versuche: &mut u128,
) {
    if p_höhe == 0 {
    } else {
        solve(
            p_von,
            p_nach,
            p_über,
            p_höhe - 1,
            board,
            max_len,
            ausgeben,
            versuche,
        );
        let board_test = board.clone();
        bewegen(p_von, p_nach, board);
        if board[0].len() != board_test[0].len() || board[1].len() != board_test[1].len() {
            if ausgeben {
                print_board(&board, max_len);
            }
            *versuche += 1;
        }
        solve(
            p_über,
            p_von,
            p_nach,
            p_höhe - 1,
            board,
            max_len,
            ausgeben,
            versuche,
        );
    }
}

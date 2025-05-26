use std::io;

mod chess;

fn read_line() -> String {
    let mut input = String::new();
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    String::from(input.trim())
}

fn game_loop(mut game: &chess::Game) {
    let mut history: Vec<chess::Game> = Vec::new();

    game.start_game();
    let mut quit : bool = false;
    while !quit
    {
        if !game.any_valid_moves()
        {
            if game.is_check()
            {
                game.set_checkmate();
            }
            else
            {
                game.set_stalemate();
            }
            quit = true;
        }
        game.fancy_print();
        if quit == true
        {
            continue;
        }

        let input = read_line();
        if input == "quit" || input == "concede"
        {
            game.set_concede();
            quit = true;
            continue;
        }

        let mut m = match game.parse_notation(input) {
            Ok(m) => m,
            Err(s) => {game.set_error(s); continue},
        };

        if let Some(s) = game.disambiguate(&mut m)
        {
            game.set_error(s);
            continue;
        }

        game.clear_notes();
        game.clear_hl();
        let mut prev = game.do_move(m);
        if game.is_check()
        {
            game = prev;
            game.hl_king();
            continue;
        }
        else
        {
            prev.clear_hl();
            history.push(prev);
        }

        game.next_turn();
    }
    println!("");
}

fn main() {
    //let mut input = String::new();
    let mut game = chess::Game::new();

    game.default_board();
    game.fancy_print();
    let _ = read_line();
    game_loop(game);
}


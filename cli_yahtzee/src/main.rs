mod utils;
mod yahtzee;

use utils::{seed, vt_open, vt_close};
use yahtzee::{Game, main_loop};

fn main() {
    seed();
    vt_open();

    let mut game = Game {
        dice:[0; 7],
        scores:[0; 13]
    };

    main_loop(&mut game);
    vt_close("\n     PRESS ANY KEY TO CLOSE");
}

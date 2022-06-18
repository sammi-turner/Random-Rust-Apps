use crate::utils::*;

/* Data structure */
pub struct Game {
    pub dice:[i32;7],
    pub scores:[i32;13]
}

/* Main loop */
pub fn main_loop(x:&mut Game) {
    default_scores(x);
    let mut count = 0;

    while count < 13 {
        default_dice(x);
        dice_count(x);
        player_rerolls(x);

        if rolls_left(x) {
            dice_count(x);
            player_rerolls(x);
        };

        dice_count(x);
        scoring_selection(x);
        count += 1;
    }

    output_scores(x);
}

/* Sets the dice held at the beginning of each round.
The zero index represents the number of re-rolls. */
fn default_dice(x:&mut Game) {
    x.dice[0] = 5;
    let mut count = 1;
    while count < 7 {
        x.dice[count] = 0;
        count += 1;
    }
}

/* Sets the value of each score box at the begining of each game.
A value of '-1' means that the score box has not been filled. */
fn default_scores(x:&mut Game) {
    let mut count = 0;
    while count < 13 {
        x.scores[count] = -1;
        count += 1;
    }
}

/* Rolls the dice and stores the results */
fn dice_count(x:&mut Game) {
    while x.dice[0] > 0 {
        let roll = pseudo(1, 6);
        x.dice[0] -= 1;

        match roll {
            1 => x.dice[1] += 1,
            2 => x.dice[2] += 1,
            3 => x.dice[3] += 1,
            4 => x.dice[4] += 1,
            5 => x.dice[5] += 1,
            6 => x.dice[6] += 1,
            _ => vt_put_slice("Ooops")
        }
    }
}

/* Player re-rolls dice by type */
fn player_rerolls(x:&mut Game) {
    let mut count = 1;
    while count < 7 {
        if x.dice[count] > 0 {
            score_card(x);
            let mut s = String::from("\n\n     How many ");
            s.push_str(&count.to_string());
            s.push_str("s do you wish to re-roll?");
            vt_put_string(&mut s);
            let ch = vt_key_i32();
            let num = ch - 48;
            if num > 0 && num <= x.dice[count] {
                x.dice[count] -= num;
                x.dice[0] += num;
            }
        }
        count += 1;
    }
}

/* Uses the number of re-rolls in the main function. */
fn rolls_left(x:&mut Game) -> bool {
    return x.dice[0] > 0;
}

/* Scores written to both the file "output.txt", and the screen */
fn output_scores(x:&mut Game) {
    vt_cls();
    let mut output = String::from("\n\n     FINAL SCORE CARD\n");

    output.push_str("\n     Ones                     ");
    output.push_str(&x.scores[0].to_string());

    output.push_str("\n     Twos                     ");
    output.push_str(&x.scores[1].to_string());

    output.push_str("\n     Threes                   ");
    output.push_str(&x.scores[2].to_string());

    output.push_str("\n     Fours                    ");
    output.push_str(&x.scores[3].to_string());

    output.push_str("\n     Fives                    ");
    output.push_str(&x.scores[4].to_string());

    output.push_str("\n     Sixes                    ");
    output.push_str(&x.scores[5].to_string());

    output.push('\n');
    output.push_str("\n     Upper total              ");
    output.push_str(&upper_total(x).to_string());

    output.push_str("\n     Upper bonus              ");
    output.push_str(&upper_bonus(x).to_string());
    output.push('\n');

    output.push_str("\n     Three of a kind          ");
    output.push_str(&x.scores[6].to_string());

    output.push_str("\n     Four of a kind           ");
    output.push_str(&x.scores[7].to_string());

    output.push_str("\n     Full house               ");
    output.push_str(&x.scores[8].to_string());

    output.push_str("\n     Small Straight           ");
    output.push_str(&x.scores[9].to_string());

    output.push_str("\n     Large Straight           ");
    output.push_str(&x.scores[10].to_string());

    output.push_str("\n     Chance                   ");
    output.push_str(&x.scores[11].to_string());

    output.push_str("\n     Yahtzee                  ");
    output.push_str(&x.scores[12].to_string());

    output.push('\n');
    output.push_str("\n     Grand total              ");
    output.push_str(&grand_total(x).to_string());
    output.push('\n');

    vt_put_string(&mut output);
    file_append("scores.txt", &*output);
}

/* Loop to insure that a valid input has been entered */
fn scoring_selection(x:&mut Game) {
    loop {
        score_card(x);
        vt_put_slice("\n\n     Which scorebox? ");
        let ch = vt_key_char();

        if ch == 'a' && x.scores[0] == -1 {
            x.scores[0] = x.dice[1];
            break;
        }

        if ch == 'b' && x.scores[1] == -1 {
            x.scores[1] = x.dice[2] * 2;
            break;
        }

        if ch == 'c' && x.scores[2] == -1 {
            x.scores[2] = x.dice[3] * 3;
            break;
        }

        if ch == 'd' && x.scores[3] == -1 {
            x.scores[3] = x.dice[4] * 4;
            break;
        }

        if ch == 'e' && x.scores[4] == -1 {
            x.scores[4] = x.dice[5] * 5;
            break;
        }

        if ch == 'f' && x.scores[5] == -1 {
            x.scores[5] = x.dice[6] * 6;
            break;
        }

        if ch == 'g' && x.scores[6] == -1 {
            x.scores[6] = three_of_a_kind(x);
            break;
        }

        if ch == 'h' && x.scores[7] == -1 {
            x.scores[7] = four_of_a_kind(x);
            break;
        }

        if ch == 'i' && x.scores[8] == -1 {
            x.scores[8] = full_house(x);
            break;
        }

        if ch == 'j' && x.scores[9] == -1 {
            x.scores[9] = small_straight(x);
            break;
        }

        if ch == 'k' && x.scores[10] == -1 {
            x.scores[10] = large_straight(x);
            break;
        }

        if ch == 'l' && x.scores[11] == -1 {
            x.scores[11] = chance(x);
            break;
        }

        if ch == 'm' && x.scores[12] == -1 {
            x.scores[12] = yahtzee(x);
            break;
        }
    }
}

/* Prints out the score card so far. */
fn score_card(x:&mut Game) {
    vt_cls();
    vt_cursor_off();
    let mut s = String::from("\n     SCORE CARD\n");

    s.push_str("\n     Ones                     ");
    if x.scores[0] == -1 {
        s.push('a');
    }
    else {
        s.push_str(&x.scores[0].to_string());
    }

    s.push_str("\n     Twos                     ");
    if x.scores[1] == -1 {
        s.push('b');
    }
    else {
        s.push_str(&x.scores[1].to_string());
    }

    s.push_str("\n     Threes                   ");
    if x.scores[2] == -1 {
        s.push('c');
    }
    else {
        s.push_str(&x.scores[2].to_string());
    }

    s.push_str("\n     Fours                    ");
    if x.scores[3] == -1 {
        s.push('d');
    }
    else {
        s.push_str(&x.scores[3].to_string());
    }

    s.push_str("\n     Fives                    ");
    if x.scores[4] == -1 {
        s.push('e');
    }
    else {
        s.push_str(&x.scores[4].to_string());
    }

    s.push_str("\n     Sixes                    ");
    if x.scores[5] == -1 {
        s.push('f');
    }
    else {
        s.push_str(&x.scores[5].to_string());
    }

    s.push_str("\n     Three of a kind          ");
    if x.scores[6] == -1 {
        s.push('g');
    }
    else {
        s.push_str(&x.scores[6].to_string());
    }

    s.push_str("\n     Four of a kind           ");
    if x.scores[7] == -1 {
        s.push('h');
    }
    else {
        s.push_str(&x.scores[7].to_string());
    }

    s.push_str("\n     Full house               ");
    if x.scores[8] == -1 {
        s.push('i');
    }
    else {
        s.push_str(&x.scores[8].to_string());
    }

    s.push_str("\n     Small straight           ");
    if x.scores[9] == -1 {
        s.push('j');
    }
    else {
        s.push_str(&x.scores[9].to_string());
    }

    s.push_str("\n     Large straight           ");
    if x.scores[10] == -1 {
        s.push('k');
    }
    else {
        s.push_str(&x.scores[10].to_string());
    }

    s.push_str("\n     Chance                   ");
    if x.scores[11] == -1 {
        s.push('l');
    }
    else {
        s.push_str(&x.scores[11].to_string());
    }

    s.push_str("\n     Yahtzee                  ");
    if x.scores[12] == -1 {
        s.push('m');
    }
    else {
        s.push_str(&x.scores[12].to_string());
    }

    s.push_str("\n\n     DICE HELD\n\n     ");

    s.push_str(&x.dice[1].to_string());
    s.push_str(" 1s, ");

    s.push_str(&x.dice[2].to_string());
    s.push_str(" 2s, ");

    s.push_str(&x.dice[3].to_string());
    s.push_str(" 3s, ");

    s.push_str(&x.dice[4].to_string());
    s.push_str(" 4s, ");

    s.push_str(&x.dice[5].to_string());
    s.push_str(" 5s, ");

    s.push_str("and ");
    s.push_str(&x.dice[6].to_string());
    s.push_str(" 6s. ");

    s.push_str("\n\n     You have ");
    s.push_str(&x.dice[0].to_string());
    s.push_str(" re-rolls.");

    vt_put_string(&mut s);
}

/* Three of a kind score function */
fn three_of_a_kind(x:&mut Game) -> i32 {
    let mut value = 0;
    if x.dice[1] > 2 || x.dice[2] > 2 || x.dice[3] > 2 || x.dice[4] > 2 || x.dice[5] > 2 || x.dice[6] > 2 {
        value = x.dice[1] + (x.dice[2] * 2) + (x.dice[3] * 3) + (x.dice[4] * 4) + (x.dice[5] * 5) + (x.dice[6] * 6);
    }
    return value;
}

/* Four of a kind score function */
fn four_of_a_kind(x:&mut Game) -> i32 {
    let mut value = 0;
    if x.dice[1] > 3 || x.dice[2] > 3 || x.dice[3] > 3 || x.dice[4] > 3 || x.dice[5] > 3 || x.dice[6] > 3 {
        value = x.dice[1] + (x.dice[2] * 2) + (x.dice[3] * 3) + (x.dice[4] * 4) + (x.dice[5] * 5) + (x.dice[6] * 6);
    }
    return value;
}

/* Full house score function */
fn full_house(x:&mut Game) -> i32 {
    let mut value = 0;
    let mut pair_test = false;
    let mut triple_test = false;

    if x.dice[1] == 3 || x.dice[2] == 3 || x.dice[3] == 3 || x.dice[4] == 3 || x.dice[5] == 3 || x.dice[6] == 3 {
        triple_test = true;
    }

    if x.dice[1] == 2 || x.dice[2] == 2 || x.dice[3] == 2 || x.dice[4] == 2 || x.dice[5] == 2 || x.dice[6] == 2 {
        pair_test = true;
    }

    if triple_test && pair_test {
        value = 25;
    }

    if x.dice[1] == 5 || x.dice[2] == 5 || x.dice[3] == 5 || x.dice[4] == 5 || x.dice[5] == 5 || x.dice[6] == 5 {
        value = 25;
    }

    return value;
}

/* Small straight score function */
fn small_straight(x:&mut Game) -> i32 {
    let mut value = 0;

    if x.dice[1] > 0 && x.dice[2] > 0 && x.dice[3] > 0 && x.dice[4] > 0 {
        value = 30;
    };

    if x.dice[2] > 0 && x.dice[3] > 0 && x.dice[4] > 0 && x.dice[5] > 0 {
        value = 30;
    };

    if x.dice[3] > 0 && x.dice[4] > 0 && x.dice[5] > 0 && x.dice[6] > 0 {
        value = 30;
    };

    return value;
}

/* Large straight score function */
fn large_straight(x:&mut Game) -> i32 {
    let mut value = 0;

    if x.dice[1] > 0 && x.dice[2] > 0 && x.dice[3] > 0 && x.dice[4] > 0 && x.dice[5] > 0 {
        value = 40;
    };

    if x.dice[2] > 0 && x.dice[3] > 0 && x.dice[4] > 0 && x.dice[5] > 0 && x.dice[6] > 0 {
        value = 40;
    };

    return value;
}

/* The total of all dice. Regardless of arrangement */
fn chance(x:&mut Game) -> i32 {
    return x.dice[1] + (x.dice[2] * 2) + (x.dice[3] * 3) + (x.dice[4] * 4) + (x.dice[5] * 5) + (x.dice[6] * 6);
}

/* Yahtzee score function */
fn yahtzee(x:&mut Game) -> i32 {
    let mut value = 0;

    if x.dice[1] == 5 || x.dice[2] == 5 || x.dice[3] == 5 || x.dice[4] == 5 || x.dice[5] == 5 || x.dice[6] == 5 {
        value = 50;
    };

    return value;
}

/* Total of the upper section */
fn upper_total(x:&mut Game) -> i32 {
    return x.scores[0] + x.scores[1] + x.scores[2] + x.scores[3] + x.scores[4] + x.scores[5];
}

/* If the upper section total is greater than 62, then a bonus of 35 is awarded */
fn upper_bonus(x:&mut Game) -> i32 {
    let mut value = 0;

    if upper_total(x) > 62 {
        value = 35;
    }

    return value;
}

/* Total of all scores */
fn grand_total(x:&mut Game) -> i32 {
    return upper_total(x) + upper_bonus(x) + x.scores[6] + x.scores[7] + x.scores[8] + x.scores[9] + x.scores[10] + x.scores[11] + x.scores[12];
}
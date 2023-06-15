use wasm_bindgen::prelude::*;

fn player_count(s: &str) -> Option<u32> {
    let last = s.trim().lines().last()?;
    let (seat, _) = last.split_once(':')?;
    seat.chars().into_iter().find_map(|s| s.to_digit(10))
}

fn players(s: &str) -> Option<Vec<Player>> {
    let pcount = player_count(s)? as usize;
    let player_lines = s.trim().lines().skip(2).take(pcount);
    Some(player_lines.filter_map(Player::from_str).collect::<Vec<_>>())
}

fn actions(s: &str, player_name: &str) -> Vec<String> {
    s
        .trim()
        .lines()
        .filter_map(|line| {
            match line.to_owned().contains(player_name) {
                true => Some(line.to_owned()),
                false => None,
            }
        })
        .collect::<Vec<_>>()
}

fn scan_num(s: &str) -> Result<usize, std::num::ParseIntError> {
    match s.chars().enumerate().fold(None, |acc: Option<(usize, String)>, (pos, char)| {
        if char.is_ascii_digit() {
            match acc {
                Some(prev) => {
                    if prev.0 == pos-1 {
                        return Some((pos, prev.1 + &char.to_string()))
                    }
                    return Some((pos, char.to_string()))
                },
                None => return Some((pos, char.to_string())),
            }
        }
        acc
    }) {
        Some((_, str)) => str.parse(),
        None => "".parse(),
    }
}

#[derive(Debug, Default)]
#[wasm_bindgen]
pub struct Player {
    seat: String,
    name: String,
    chips: usize,
}

impl Player {
    fn from_str(s: &str) -> Option<Self> {
        let start = s.find(':')?;
        let end = s.to_owned().find('(')?;
        let name = &s[start+2..end-1];

        let chips = match scan_num(&s[end..]) {
            Ok(chips) => chips,
            Err(_) => return None,
        };

        Some(Player {
            seat: s[..start].to_string(),
            name: name.to_string(),
            chips,
        })
    }
}

#[wasm_bindgen]
pub fn winnings(s: &str, player: Player) -> isize {
    let actions = actions(s, &player.name);
    let sanitized_actions = actions.iter().map(|action| {
        action
            .replace(&player.name, "")
            .replace(&player.seat, "")
    }).collect::<Vec<_>>();

    match scan_num(sanitized_actions.last().unwrap()) {
        Ok(won) => won as isize,
        Err(_lost) => {
            let loss = sanitized_actions.iter().skip(1).fold(player.chips, |acc, action| {
                match scan_num(action) {
                    Ok(loss) => acc - loss,
                    Err(_) => acc,
                }
            });
            loss as isize - player.chips as isize
        },
    }
}

struct Hand {
    timestamp: usize,
    level: usize,
    blinds: (usize, usize),
    player_count: usize,
}


#[derive(Debug)]
enum Variant {
    HoldEm,
}

#[derive(Debug)]
enum Limits {
    NoLimit,
}

#[derive(Debug)]
enum MaxSize {
    Nine([Option<Player>; 9]),
    Six([Player; 6]),
}

#[derive(Debug)]
struct Poker {
    variant: Variant,
    limits: Limits,
    table_size: MaxSize,
}

#[cfg(test)]
mod tests {
    use crate::Player;


    #[test]
    fn poker_test() {


        let pcount = crate::players(TEST).unwrap();

        let mut a: [Player; 9] = Default::default();
        a.copy_from_slice(&pcount);

        let pcount = crate::player_count(TEST).unwrap() as usize;
        let player_lines = s.trim().lines().skip(2).take(pcount);

        let poker = crate::Poker {
            variant: crate::Variant::HoldEm,
            limits: crate::Limits::NoLimit,
            table_size: crate::MaxSize::Nine(Default::default())
        };

        dbg!(poker);
    }

    const TEST: &str = "PokerStars Hand #237451615499: Tournament #3443583345, $0.91+$0.09 USD Hold'em No Limit - Level IX (200/400) - 2022/07/19 12:45:55 EET [2022/07/19 5:45:55 ET]
Table '3443583345 2' 9-max Seat #5 is the button
Seat 1: lennart65632 (12354 in chips)
Seat 2: DENbarcelona (8432 in chips)
Seat 3: doppelkorn63 (4728 in chips)
Seat 4: 12blackartus (8984 in chips)
Seat 5: Cargat (4895 in chips)
Seat 6: Dol Cheep (9285 in chips)
Seat 7: lolo011274 (4663 in chips)
Seat 8: johnnyjohnM (3213 in chips)
Seat 9: kahoona1092 (10946 in chips)
lennart65632: posts the ante 25
DENbarcelona: posts the ante 25
doppelkorn63: posts the ante 25
12blackartus: posts the ante 25
Cargat: posts the ante 25
Dol Cheep: posts the ante 25
lolo011274: posts the ante 25
johnnyjohnM: posts the ante 25
kahoona1092: posts the ante 25
Dol Cheep: posts small blind 200
lolo011274: posts big blind 400
*** HOLE CARDS ***
Dealt to Dol Cheep [Kh Ah]
johnnyjohnM: folds
kahoona1092: raises 1225 to 1625
lennart65632: calls 1625
DENbarcelona: folds
doppelkorn63: folds
12blackartus: folds
Cargat: folds
Dol Cheep: calls 1425
lolo011274: folds
*** FLOP *** [9h 5c 3h]
Dol Cheep: checks
kahoona1092: checks
lennart65632: bets 5500
Dol Cheep: raises 2135 to 7635 and is all-in
kahoona1092: folds
lennart65632: calls 2135
*** TURN *** [9h 5c 3h] [9c]
*** RIVER *** [9h 5c 3h 9c] [Js]
*** SHOW DOWN ***
Dol Cheep: shows [Kh Ah] (a pair of Nines)
lennart65632: shows [5s Ad] (two pair, Nines and Fives)
lennart65632 collected 20770 from pot
*** SUMMARY ***
Total pot 20770 | Rake 0
Board [9h 5c 3h 9c Js]
Seat 1: lennart65632 showed [5s Ad] and won (20770) with two pair, Nines and Fives
Seat 2: DENbarcelona folded before Flop (didn't bet)
Seat 3: doppelkorn63 folded before Flop (didn't bet)
Seat 4: 12blackartus folded before Flop (didn't bet)
Seat 5: Cargat (button) folded before Flop (didn't bet)
Seat 6: Dol Cheep (small blind) showed [Kh Ah] and lost with a pair of Nines
Seat 7: lolo011274 (big blind) folded before Flop
Seat 8: johnnyjohnM folded before Flop (didn't bet)
Seat 9: kahoona1092 folded on the Flop";

    #[test]
    fn player_count_test() {
        assert_eq!(crate::player_count(TEST), Some(9));
    }

    #[test]
    fn players_test() {
        let pcount = crate::players(TEST).unwrap();
        dbg!(pcount);
    }

    #[test]
    fn action_test() {
        let pcount = crate::actions(TEST, "Dol Cheep");
        dbg!(pcount);
        //assert_eq!(pcount, vec!["Dol Cheep"]);
    }

    #[test]
    fn winnings_test() {
        let pcount = crate::players(TEST).unwrap();
        for p in pcount {
            let pcount = crate::winnings(TEST, p);
            dbg!(pcount);
        }
        //assert_eq!(pcount, vec!["Dol Cheep"]);
    }
}

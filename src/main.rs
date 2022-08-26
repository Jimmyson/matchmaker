use clap::Parser;
use rand::{thread_rng, Rng};
use std::fs;
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1


struct RoomDimension {
    capacity: usize,
    amount: usize
}

/// Types of Balancing Methods
#[derive(clap::ValueEnum, Clone, Debug, EnumIter, PartialEq)]
enum BalanceMode {
    SingleRoom,
    FullRooms,
    EvenSplit,
    FullTailBalance,
    FullBalance,
    QuietRooms,
    LotsOfMatches,
    Automatic,
}

/// Option Flags for the Application
#[derive(Parser, Debug)]
#[clap(author = "Jimmyson", version, about = "I sort people in games", long_about = None)]
struct Args {
    /// Maximum players in a room.
    #[clap(short = 'x', long, value_parser, default_value_t = 4)]
    maximum: usize,

    /// Minimum players a room can have.
    #[clap(short = 'n', long, value_parser, default_value_t = 2)]
    minimum: usize,

    /// Total Player Count
    // #[clap(short = 'p', long, value_parser)]
    // players: usize,

    /// Mode to calculate the Player Balacing.
    #[clap(short, long = "balance-mode", value_enum, default_value_t = BalanceMode::Automatic)]
    mode: BalanceMode,

    /// Text file with names of the players.
    #[clap(short, long, value_parser)]
    input: std::path::PathBuf,
}

/// Divide the Player Base into even sized teams until
fn fn_even_split(min_size: usize, max_size: usize, player_base: usize, single_pass: bool, rooms: &mut Vec<RoomDimension>) -> bool {
    //println!("**** TRY EVEN SPLIT ****");
    let mut team_count = 2;
    while team_count < max_size {
        let team_size = player_base / team_count; // Drops the decimal, use modules to get the remainder

        //println!("Room Count: {team_count}, Room Size {team_size}");
        // Check team size meets match conditions, and no remainders
        if (team_size <= max_size) && (team_size >= min_size) && (player_base % team_count == 0) {
            //println!("{team_count} rooms can be made of {team_size} players");
            rooms.push(RoomDimension {
                capacity: team_size,
                amount: team_count
            });

            return true;
        }

        // Increment Teams Count
        team_count += 1;
    }

    if single_pass{
        println!("Unable to evenly split teams");
    }

    return false;
}

// fn break_input() {
//     let mut line = String::new();
//     let b1 = std::io::stdin().read_line(&mut line).unwrap();
// }

/// Fill rooms, and balance remainder into matches so there are two sets to capacities
fn fn_full_balance(min_size: usize, max_size: usize, player_base: usize, single_pass: bool, rooms: &mut Vec<RoomDimension>) -> bool {
    //println!("**** TRY FULL BALANCE ****");
    // ** Full Matches
    let full_rooms = player_base / max_size;
    // ** Remainder
    let remainder = player_base % max_size;
    //println!("Remainder: {remainder}");

    // ** Check Remainders can play a game...
    if remainder >= min_size || remainder == 0
    {
        // *** Start the game
        //println!("Enough remainders to enter their own game.");
        //let open_rooms = remainder / min_size;
        //println!("Full Rooms: {full_rooms}, Open Rooms: {open_rooms} with {remainder} players");
        
        rooms.push(RoomDimension {
            capacity: max_size,
            amount: full_rooms
        });        
        rooms.push(RoomDimension {
            capacity: remainder,
            amount: 1
        });

        return true;
    }
    else
    {
        //println!("Going to rebalance");
        // *** Require Rebalance
        //let open_places = min_size - remainder;
        //println!("Open Places: {open_places}");

        // -----------------------

        let mut balance_remainder = remainder;
        let mut matches_to_affect = 1;
        let mut balanced_cap: usize = 0;
        
        while balance_remainder > 0 {
            matches_to_affect += 1;
            let players_to_affect = (max_size * (matches_to_affect - 1)) + remainder;
            balanced_cap = players_to_affect / matches_to_affect;
            balance_remainder = players_to_affect % balanced_cap;
            //println!("Matches to Affect: {matches_to_affect}, Players to Affect: {players_to_affect}");
            //println!("Bal Cap: {balanced_cap}, Bal Remainder: {balance_remainder}");
            //break_input();
        }

        if balance_remainder == 0 {
            let real_full_match = full_rooms + 1 - matches_to_affect;
            //let players_in_full = real_full_match * max_size;
            //println!("Real full matches: {real_full_match}, Players: {players_in_full}");

            rooms.push(RoomDimension {
                capacity: real_full_match,
                amount: max_size
            });

            let players_in_balaced = matches_to_affect * balanced_cap;
            //println!("Balanced full matches: {matches_to_affect}, Players: {players_in_balaced}");
            
            rooms.push(RoomDimension {
                capacity: players_in_balaced,
                amount: matches_to_affect
            });

            //let full_balance_players = players_in_full + players_in_balaced;
            //println!("Balanced Players: {full_balance_players}");
            
            return true;
        }
    }

    if single_pass{
        println!("Unable to balance rooms");
    }
    return false;
}

/// Fill matches, and balance last 2 matches. OK if uneven.
fn fn_full_tail_balance(min_size: usize, max_size: usize, player_base: usize, _single_pass: bool, rooms: &mut Vec<RoomDimension>) -> bool {
    //println!("**** TRY FULL TAIL BALANCE ****");
    // ** Full Matches
    let mut full_rooms = player_base / max_size;
    //println!("Full Rooms: {full_rooms}");
    // ** Remainder
    let remainder = player_base % max_size;
    //println!("Remainder: {remainder}");

    // ** Check Remainders can play a game...
    if remainder >= min_size || remainder == 0 {
        // *** Start the game
        //println!("Enough remainders to enter their own game.");
        //let open_rooms = remainder / min_size;

        //println!("Full Rooms: {full_rooms}, Open Rooms: {open_rooms} with {remainder} players");
        
        rooms.push(RoomDimension {
            amount: full_rooms,
            capacity: max_size
        });

        if remainder != 0 {
            rooms.push(RoomDimension {
                capacity: remainder,
                amount: 1
            });
        }

        return true;
    } else {
        //println!("Going to rebalance");
        // *** Require Rebalance
        //let open_places = min_size - remainder;
        //println!("Open Places: {open_places}");

        // -----------------------

        let players_to_affect = remainder + max_size;
        full_rooms -= 1;
        
        rooms.push(RoomDimension {
            amount: full_rooms,
            capacity: max_size
        });

        let new_balance = players_to_affect / 2;
        let is_uneven = players_to_affect % 2 == 0;
        //println!("Full Rooms: {full_rooms} with {max_size} players");
        
        if !is_uneven {
            //println!("Balanced Rooms: 2 rooms with {new_balance} players");
            
            rooms.push(RoomDimension {
                capacity: new_balance,
                amount: 2
            });

            return true;
        } else {
            // println!(
            //     "Balanced Rooms: 2 rooms with {0} and {1}",
            //     new_balance,
            //     new_balance + 1
            // );

            rooms.push(RoomDimension {
                capacity: new_balance,
                amount: 1
            });
            rooms.push(RoomDimension {
                capacity: new_balance + 1,
                amount: 1
            });

            return true;
        }
    }

    //return false; //UNREACHABLE
}

/// Play a single match as not enough players to split into multiple rooms
fn fn_single_round(min_size: usize, max_size: usize, player_base: usize, single_pass: bool, rooms: &mut Vec<RoomDimension>) -> bool {
    //println!("**** TRY SINGLE ROUND ****");
    if (player_base <= max_size) && (player_base >= min_size) {
        //println!("Everyone verse Everyone");
        rooms.push(RoomDimension {
            capacity: player_base,
            amount: 1
        });

        return true;
    }

    if single_pass{
        println!("Not enough players to conduct a match");
    }

    return false;
}

/// Create matches based on lowest supported capacity, and places remainders into a match
/// THIS IS A TIME WASTER, AND BEST TO AVOID UNLESS YOU WANT TO RUIN PEOPLE'S DAY
fn fn_minimum_fill(min_size: usize, max_size: usize, player_base: usize, single_pass: bool, rooms: &mut Vec<RoomDimension>) -> bool {
    //println!("**** TRY MINIMUM FILL ****");
    let matches_to_populate = player_base / min_size;

    //println!("Matches to Populate: {matches_to_populate}");
    let players_to_assign = player_base - (matches_to_populate * min_size);
    //println!("Players to Assign: {players_to_assign}");
    //let matches_with_bare_minimum = matches_to_populate - players_to_assign;
    let balanced_cap = ((min_size * players_to_assign) + players_to_assign) / players_to_assign;
    //println!("Balanced Cap: {balanced_cap}");

    if balanced_cap > max_size
    {
        if single_pass{
            println!("**** People need to drop out, or find more players");
        }
        return false;
    }

    //println!("Players spread around.");
    //println!("{matches_with_bare_minimum} matches with bare minimum of {min_size} player.");
    //println!("{players_to_assign} matches with {balanced_cap} players");

    rooms.push(RoomDimension {
        capacity: balanced_cap,
        amount: matches_to_populate
    });

    //let minimum_fill_player_count =
    //    (matches_with_bare_minimum * min_size) + (players_to_assign * balanced_cap);
    // println!(
    //     "* Total Players: {minimum_fill_player_count} from original player based of {player_base}"
    // );

    return true;
}

/// Create matches based on lowest supported capacity, and
/// THIS IS A TIME WASTER, AND BEST TO AVOID UNLESS YOU WANT TO RUIN PEOPLE'S DAY
//fn fn_lots_of_matches(min_size: usize, max_size: usize, player_base: usize, rooms: &mut Vec<RoomDimension>) -> bool {
fn fn_lots_of_matches(min_size: usize, player_base: usize, single_pass: bool, rooms: &mut Vec<RoomDimension>) -> bool {
    //println!("**** TRY MINIMUM ****");
    if player_base % min_size == 0 {
        let room_setup = player_base / min_size;
        //println!("Enough people to fill lowest common denominator. Setup {room_setup} rooms");

        rooms.push(RoomDimension {
            capacity: min_size,
            amount: room_setup
        });

        return true;
    }

    if single_pass {
        println!("Unable to split on lowest team size");
    }
    return false;
}

/// Setup rooms where all slots are filled
//fn fn_full_rooms(min_size: usize, max_size: usize, player_base: usize, rooms: &mut Vec<RoomDimension>) -> bool {
fn fn_full_rooms(max_size: usize, player_base: usize, single_pass: bool, rooms: &mut Vec<RoomDimension>) -> bool {
    //println!("**** TRY FULL ROOMS ****");
    if player_base % max_size == 0 {
        let room_setup = player_base / max_size;
        //println!("Every match is full. Setup {room_setup} rooms");
        
        rooms.push(RoomDimension {
            capacity: max_size,
            amount: room_setup
        });

        return true;
    }

    if single_pass {
        println!("Unable to split on maximum room size");
    }
    
    return false;
}

fn main() {
    let args = Args::parse();

    // GET MATCH SIZING
    // * Max Lobby Size
    let max_size = args.maximum;
    // * Min Group Size
    let min_size = args.minimum;
    // * Player Count
    //let player_base = args.players;
    // * Mode Algorithum
    let mode = args.mode;
    // * Input File
    let input = args.input;

    if max_size < min_size {
        println!("Invalid Parameters");
        return;
    }

    // READ NAMES
    let data = fs::read_to_string(input).expect("Unable to read file");

    let players = data.lines().count();
    //println!("Name count: {players}");

    let player_base = players;

    let mut rooms = Vec::<RoomDimension>::new();
    let mut result: bool = false;

    // BALANCE THE TEAMS
    match mode {
        // * Not enough players to fill a lobby
        BalanceMode::SingleRoom => result = fn_single_round(min_size, max_size, player_base, true, &mut rooms),

        // * Full Rooms
        BalanceMode::FullRooms => result = fn_full_rooms(max_size, player_base, true, &mut rooms),

        // * Even Split
        BalanceMode::EvenSplit => result = fn_even_split(min_size, max_size, player_base, true, &mut rooms),
        
        // * Minimum Fill
        BalanceMode::QuietRooms => result = fn_minimum_fill(min_size, max_size, player_base, true, &mut rooms),

        // * Full and balanced tail
        BalanceMode::FullTailBalance => result = fn_full_tail_balance(min_size, max_size, player_base, true, &mut rooms),

        // * Full and balanced
        BalanceMode::FullBalance => result = fn_full_balance(min_size, max_size, player_base, true, &mut rooms),

        // * Lots of Matches
        BalanceMode::LotsOfMatches => result = fn_lots_of_matches(min_size, player_base, true, &mut rooms),

        // * Automatic, Try all methods
        _ =>
        {
            //println!("Landed in Automatic");
            for m in BalanceMode::iter()
            {

                match m {
                    BalanceMode::SingleRoom => {
                        result = fn_single_round(min_size, max_size, player_base, false, &mut rooms);
                    }
                    BalanceMode::FullRooms => {
                        result = fn_full_rooms(max_size, player_base, false, &mut rooms);
                    }
                    BalanceMode::EvenSplit => {
                        result = fn_even_split(min_size, max_size, player_base, false, &mut rooms);
                    }
                    BalanceMode::FullTailBalance => {
                        result = fn_full_tail_balance(min_size, max_size, player_base, false, &mut rooms);
                    }
                    BalanceMode::FullBalance => {
                        result = fn_full_balance(min_size, max_size, player_base, false, &mut rooms);
                    }
                    BalanceMode::QuietRooms => {
                        result = fn_minimum_fill(min_size, max_size, player_base, false, &mut rooms);
                    }
                    _ => continue
                }

                if result
                {
                    break;
                }
            };
        }
    };

    if !result
    {
        println!("No rooms generated");
        return;
    }

    // SHUFFLE THE PLAYERS
    //println!("*** Suffle Players ***");
    let mut rng = thread_rng();
    let mut names: Vec<&str> = Vec::new();

    for name in data.lines()
    {
        names.push(name);
    }

    let mut order: Vec<&str> = Vec::new();

    while names.len() > 0
    {
        let pos = rng.gen_range(0..names.len());
        order.push(names[pos]);
        names.remove(pos);
    }

    // println!("** Print Names");
    // for name in order
    // {
    //     println!("{}", name);
    // }

    // PUT INTO MATCHES
    let mut match_count = 0;
    for i in 0..rooms.len()
    {
        for _j in 0..rooms[i].amount
        {
            match_count += 1;
            println!("--- MATCH {0} ---", match_count);

            for _l in 0..rooms[i].capacity
            {
                let player = order[0];
                order.remove(0);
                println!("{}", player);
            }

            println!();
        }
        //let cap = rooms[i].capacity;
        //let amt = rooms[i].amount;
        //println!("{cap}, {amt}");
    }
}

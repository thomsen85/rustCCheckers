use text_io::read;
use std::thread;
use rand::Rng;
use std::time::Instant;

use crate::board::*;
use crate::mcts::Tree;
use std::sync::{Arc, Mutex};

const NUM_OF_TRAINS: usize = 10_000;

pub fn start() {
    let time = Instant::now();
    let num_of_cpu = num_cpus::get();
    println!("Num of threads = {}", num_of_cpu);
    let ex_per_thread = NUM_OF_TRAINS / num_of_cpu;
    println!("Num of sims running = {}", ex_per_thread * num_of_cpu);
    let mut threads = Vec::with_capacity(num_of_cpu); 

    let tree = Arc::new(Mutex::new(Tree::new()));

    for _ in 0..num_of_cpu {
        let tree = Arc::clone(&tree);
        threads.push(thread::spawn(move || {
            let mut wins: usize = 0;
            let mut lost: usize = 0;
            let mut plays: usize = 0;

            for _ in 0..ex_per_thread {

                let (moves, result) = training();

                if result == 1 {
                    wins += 1;
                } else if result == -1 {
                    lost += 1;
                }

                tree.lock().unwrap().add_game(moves, result);

            plays += 1;
            }
            (wins, lost, plays)
        }));
    }

    let mut wins: usize = 0;
    let mut lost: usize = 0;
    let mut plays: usize = 0;

    for t in threads {
        let (w, l, p) = t.join().unwrap();
        wins += w;
        lost += l;
        plays += p;
    }

    println!("W: {}, D: {}, L: {}, P: {}", wins, plays-wins-lost, lost, plays);
    let res =  tree.lock().unwrap().root.lock().unwrap().result;
    let p = tree.lock().unwrap().root.lock().unwrap().plays;
    let size = std::mem::size_of_val(&*tree);
    println!("Tree: R: {}, P: {}, mem: {}", res , p, size);
    println!("Took: {} sec", time.elapsed().as_secs_f64());
}

fn training() -> (Vec<Move>, i8){
    let mut board = Board::new();
    board.add_player(PLAYER_1_POS, 1);
    board.add_player(PLAYER_2_POS, 2);
    let mut rng = rand::thread_rng();
    let mut player = 1;
    let mut counter = 0;
    let mut moves: Vec<Move> = Vec::with_capacity(300);

    while !board.is_won() {
        if rng.gen_range(0..100) > 20 {
            let mov = board.closest_move(player);
            board.move_pice(mov);
            moves.push(mov);
        } else {
            let mov = board.random_move(player, &mut rng);
            board.move_pice(mov);
            moves.push(mov);
        }

        if player == 1 {
            player = 2;
        } else {
            player = 1;
        } 
        counter += 1;

        if counter > 600 {
            break;
        }
    }
    
    // println!("{}", board.to_string());
    let mut result: i8 = 0;
    if board.is_won() {
        if board.player_in_other_terretory(0) {
            result = 1;
        } else {
            result = -1;
        }
    } 
    (moves, result)
}

fn test() {

    let mut board = Board::new();
    board.add_player(PLAYER_1_POS, 1);
    board.add_player(PLAYER_2_POS, 2);

    println!("{}", board.to_string());

}


fn manual() {
    let mut board = Board::new();

    board.add_player(PLAYER_1_POS, 1);
    board.add_player(PLAYER_2_POS, 2);
    
    let mut inp: i8 = 0;

    while inp != -1 {
        println!("{}", board.to_string());
        inp = read!();
        println!("Got: {}", inp);
        let best_move = board.closest_move(inp);
        println!("Best Move {:?}", best_move);
        board.move_pice(best_move);
    }
}


const PLAYER_1_POS: [Point; 10] = [(12,0), (11,1), (13, 1), (10, 2),
      (12, 2), (14,2), (9, 3), (11, 3), (13, 3), (15, 3)];

const PLAYER_2_POS: [Point; 10] = [(12,16), (11, 15), (13, 15), (10, 14), (12, 14), (14, 14),
                (9, 13), (11, 13), (13, 13), (15, 13)];



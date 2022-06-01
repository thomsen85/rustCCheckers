use text_io::read;
use rand::prelude::*;
use indicatif::ProgressBar;
use std::thread;

const NUM_OF_THREADS: usize = 100; 
const NUM_OF_TRAINS: usize = 100_000;

pub fn start() {
    let ex_per_thread = NUM_OF_TRAINS / NUM_OF_THREADS;
    let mut threads = Vec::with_capacity(NUM_OF_THREADS); 

    for _ in 0..NUM_OF_THREADS {
        threads.push(thread::spawn(move || {
            for i in 0..ex_per_thread {
                training();
            }
        }));
    }

    for t in threads {
        t.join().unwrap();
    }
}

fn training() {
    let mut board = Board::new();
    board.add_player(PLAYER_1_POS, 1);
    board.add_player(PLAYER_2_POS, 2);
    let mut rng = rand::thread_rng();
    let mut player = 1;
    let mut counter = 0;

    while !board.is_won() {
        if random() {
            let mov = board.closest_move(player);
            board.move_pice(mov.0, mov.1);
        } else {
            let mov = board.random_move(player, &mut rng);
            board.move_pice(mov.0, mov.1);
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
    
    //println!("{}", board.to_string());
    /*
    if board.is_won() {
        if board.player_in_other_terretory(0) {
            println!("Player 1 Won");
        } else {
            println!("Player 2 Won");
        }
    } else {
        println!("No one won");
    }
      */  
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
        board.move_pice(best_move.0, best_move.1);
    }
}

fn delta_distance(pre_move: Point, post_move: Point, to: Point) -> i32 {
    let v_pre = ((pre_move.0 - to.0) as i32).pow(2) + ((pre_move.1 - to.1) as i32).pow(2);
    let v_post = ((post_move.0 - to.0) as i32).pow(2) + ((post_move.1 - to.1) as i32).pow(2);
    v_pre - v_post
}



const PLAYER_1_POS: [Point; 10] = [(12,0), (11,1), (13, 1), (10, 2),
      (12, 2), (14,2), (9, 3), (11, 3), (13, 3), (15, 3)];

const PLAYER_2_POS: [Point; 10] = [(12,16), (11, 15), (13, 15), (10, 14), (12, 14), (14, 14),
                (9, 13), (11, 13), (13, 13), (15, 13)];


type Point = (i8, i8);
type Move = (Point, Point);

struct Board {
    width: i8,
    height: i8,
    board: [[i8; 25]; 17],
    players_start_pos: Vec<[Point; 10]>, 

}

impl Board {
    fn new() -> Self {

        let board = [[-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1], [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, -1, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1], [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, -1, 0, -1, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1], [-1, -1, -1, -1, -1, -1, -1, -1, -1, 0, -1, 0, -1, 0, -1, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1], [0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0], [-1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1], [-1, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, -1], [-1, -1, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, -1, -1], [-1, -1, -1, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, -1, -1, -1], [-1, -1, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, -1, -1], [-1, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, -1], [-1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1], [0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0], [-1, -1, -1, -1, -1, -1, -1, -1, -1, 0, -1, 0, -1, 0, -1, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1], [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, -1, 0, -1, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1], [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, -1, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1], [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1]];

        Board {
            width: 25,
            height: 17,
            board,
            players_start_pos: Vec::new(),
        }
    }

    fn get(&self, (x, y): Point) -> i8{
       self.board[(self.height - y - 1) as usize][x as usize] 
    } 

    fn set(&mut self, (x, y): Point, var: i8) {
        self.board[(self.height - y - 1) as usize][x as usize] = var;
    }

    fn move_pice(&mut self, from: Point, to: Point) {
        let val = self.get(from);
        std::assert!(val != 0 && val != -1);
        self.set(from, 0);
        self.set(to, val);
    }

    fn add_player(&mut self, player_pos: [Point; 10], player_nr: i8) {
        for pos in player_pos {
            self.set(pos, player_nr);
        }
        self.players_start_pos.push(player_pos);
    }

    fn is_won(&self) -> bool{
        (0..self.players_start_pos.len())
            .into_iter()
            .map(|i| self.player_in_other_terretory(i))
            .any(|b| b)
    }

    fn player_in_other_terretory(&self, player_index: usize) -> bool {
        for (i, pos) in self.players_start_pos.iter().enumerate() {
            if i == player_index {
                continue;
            } else {
                for coord in self.get_player_positons((player_index+1) as i8) {
                    //println!("Player:{}, {:?} in {:?}", player_index + 1, coord, pos);
                    if !pos.contains(&coord) {
                        return false;
                    } 
                    
                }
            }
        }
        true
        
    }

    fn closest_move(&self, player: i8) -> (Point, Point){
        // TODO: fix this
        
        let opponent_point: Point = {
            if player == 1{
                (12, 16)
            } else {
                (12, 0)
            }
        };
        let mut best_moves: Vec<(Point, Point)> = Vec::new();

        for pos in self.get_player_positons(player) {
            let best = self.get_legal_moves(pos, false, Vec::new(), 0)
                .into_iter()
                .max_by(|&m1, &m2| delta_distance(pos, m1, opponent_point).cmp(&delta_distance(pos, m2, opponent_point)));
                        
            if let Some(mov) = best {
                best_moves.push((pos, mov));
            }
        }
        // TODO: can be optimized
        for mov in best_moves.clone() {
            //println!("{:?}: {}", mov, delta_distance(mov.0, mov.1, opponent_point))
        } 

        best_moves
            .into_iter()
            .max_by(|&m1, &m2| delta_distance(m1.0, m1.1, opponent_point).cmp(&delta_distance(m2.0, m2.1, opponent_point)))
            .unwrap()
    }

    fn random_move(&self, player: i8, thread_rng: &mut ThreadRng) -> (Point, Point) {
        let mut moves: Vec<(Point, Point)> = Vec::new();

        for pos in self.get_player_positons(player) {
            for mov in self.get_legal_moves(pos, false, Vec::new(), 0) {
                moves.push((pos, mov));
            }
        }
         
        moves[thread_rng.gen_range(0..moves.len())]

    }
            
    


    fn get_legal_moves(&self, point: Point, jump: bool, mut prev: Vec<Point>, dim: u32) -> Vec<Point>{

        if self.get(point) <= 0 && !jump {
            return Vec::new();
        }

        let mut moves: Vec<Point> = Vec::new();
        
        if jump {
            moves.push(point);
        } else {
            prev.clear();
        }

        for surr_point in self.get_surrounding(point) {
            if self.get(surr_point) == 0 && !jump {
                moves.push(surr_point);
            } else if self.get(surr_point) > 0{
                if let Some(over_point) = self.get_coord_over_piece(point, surr_point) {
                        if !prev.contains(&over_point) && self.get(over_point) == 0 {
                            prev.push(point);
                            //println!("{}At {:?}, Checking: {:?}"," ".repeat(dim as usize), surr_point, over_point);
                            moves.extend(self.get_legal_moves(over_point, true, prev.clone(), dim + 1))
                        } 
                    }
            }
        }
        prev.clear();
        moves
    }

    fn get_player_positons(&self, player: i8) -> [Point; 10]{
        let mut players = [(0, 0); 10];
        let mut counter = 0;

        for x in 0..self.width{ 
            for y in 0..self.height{
                if self.get((x,y)) == player {
                    players[counter] = (x, y);
                    counter += 1;
                }
            }
        }
        players
    }

    fn get_coord_over_piece(&self, current_point: Point, surrounding_point: Point) -> Option<Point> {
        let v_x = surrounding_point.0  - current_point.0 ;
        let v_y = surrounding_point.1  - current_point.1 ;

        if surrounding_point.0 < 0 - v_x || surrounding_point.0 + v_x >= self.width ||
            surrounding_point.1 < 0 - v_y || surrounding_point.1 + v_y >= self.height {
                return None;
            }
    

        let x = surrounding_point.0 + v_x;
        let y = surrounding_point.1 + v_y;

        Some((x, y)) 
    }

    fn get_surrounding(&self, (x, y): Point) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();

        if x >= 2 {
            points.push((x-2, y))
        }
        if x  >= 1 && y + 1 < self.height {
            points.push((x-1, y+1))
        }
        if x + 1 < self.width && y + 1 < self.height{
            points.push((x+1, y+1))
        }
        if x + 2 < self.width{
            points.push((x+2, y))
        }
        if x + 1 < self.width && y >= 1 {
            points.push((x+1, y-1))
        }
        if x >= 1 && y>= 1 {
            points.push((x-1, y-1))
        }

        points.into_iter().filter(|&p| self.get(p) >= 0).collect()

    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        for row in self.board {
            for val in row {
                if val.is_negative() {
                    s.push_str(" ");
                } else {
                    s.push_str(&val.to_string());
                }
            }    
            s.push_str("\n")
        }
        s
    }
}

struct Node {

}


#[cfg(test)]
pub mod tests {
    use super::*;
    
    #[test]
    fn test_delta_distance() {
        let to = (12, 15);
        let point1 = (13,3);
        let point2 = (12,0);
        let move1 = (13, 7);
        let move2 = (12, 8);

        let dist1 = delta_distance(point1, move1, to);
        let dist2 = delta_distance(point2, move2, to);
        println!("{} < {}", dist1, dist2);

        std::assert!(dist1 < dist2);
    }
}
    
 

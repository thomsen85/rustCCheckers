use rand::prelude::*;

pub type Point = (i8, i8);
pub type Move = (Point, Point);

pub struct Board {
    width: i8,
    height: i8,
    board: [[i8; 25]; 17],
    players_start_pos: Vec<[Point; 10]>,
}

#[inline]
fn delta_distance(pre_move: Point, post_move: Point, to: Point) -> i32 {
    let v_pre = ((pre_move.0 - to.0) as i32).pow(2) + ((pre_move.1 - to.1) as i32).pow(2);
    let v_post = ((post_move.0 - to.0) as i32).pow(2) + ((post_move.1 - to.1) as i32).pow(2);
    v_pre - v_post
}

impl Board {
    pub fn new() -> Self {
        let board = [
            [
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, -1, -1, -1, -1, -1, -1, -1, -1,
                -1, -1, -1, -1,
            ],
            [
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, -1, 0, -1, -1, -1, -1, -1, -1, -1,
                -1, -1, -1, -1,
            ],
            [
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, -1, 0, -1, 0, -1, -1, -1, -1, -1, -1,
                -1, -1, -1, -1,
            ],
            [
                -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, -1, 0, -1, 0, -1, 0, -1, -1, -1, -1, -1, -1,
                -1, -1, -1,
            ],
            [
                0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1,
                0,
            ],
            [
                -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0,
                -1,
            ],
            [
                -1, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0,
                -1, -1,
            ],
            [
                -1, -1, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1,
                -1, -1,
            ],
            [
                -1, -1, -1, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, -1,
                -1, -1,
            ],
            [
                -1, -1, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1,
                -1, -1,
            ],
            [
                -1, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0,
                -1, -1,
            ],
            [
                -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0,
                -1,
            ],
            [
                0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, -1,
                0,
            ],
            [
                -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, -1, 0, -1, 0, -1, 0, -1, -1, -1, -1, -1, -1,
                -1, -1, -1,
            ],
            [
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, -1, 0, -1, 0, -1, -1, -1, -1, -1, -1,
                -1, -1, -1, -1,
            ],
            [
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, -1, 0, -1, -1, -1, -1, -1, -1, -1,
                -1, -1, -1, -1,
            ],
            [
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, -1, -1, -1, -1, -1, -1, -1, -1,
                -1, -1, -1, -1,
            ],
        ];

        Board {
            width: 25,
            height: 17,
            board,
            players_start_pos: Vec::new(),
        }
    }

    pub fn get(&self, (x, y): Point) -> i8 {
        self.board[(self.height - y - 1) as usize][x as usize]
    }

    pub fn set(&mut self, (x, y): Point, var: i8) {
        self.board[(self.height - y - 1) as usize][x as usize] = var;
    }

    pub fn move_pice(&mut self, (from, to): Move) {
        let val = self.get(from);
        std::assert!(val != 0 && val != -1);
        self.set(from, 0);
        self.set(to, val);
    }

    pub fn add_player(&mut self, player_pos: [Point; 10], player_nr: i8) {
        for pos in player_pos {
            self.set(pos, player_nr);
        }
        self.players_start_pos.push(player_pos);
    }

    pub fn is_won(&self) -> bool {
        (0..self.players_start_pos.len())
            .into_iter()
            .map(|i| self.player_in_other_terretory(i))
            .any(|b| b)
    }

    pub fn player_in_other_terretory(&self, player_index: usize) -> bool {
        for (i, pos) in self.players_start_pos.iter().enumerate() {
            if i == player_index {
                continue;
            } else {
                for coord in self.get_player_positons((player_index + 1) as i8) {
                    //println!("Player:{}, {:?} in {:?}", player_index + 1, coord, pos);
                    if !pos.contains(&coord) {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn closest_move(&self, player: i8) -> (Point, Point) {
        let opponent_point: Point = {
            if player == 1 {
                (12, 16)
            } else {
                (12, 0)
            }
        };
        let mut best_moves: Vec<(Point, Point)> = Vec::new();

        for pos in self.get_player_positons(player) {
            /*

            let best = self.get_legal_moves(pos, false, Vec::new(), 0)
                .into_iter()
                .max_by(|&m1, &m2| delta_distance(pos, m1, opponent_point).cmp(&delta_distance(pos, m2, opponent_point)));
            */

            let best = {
                let mut max = i32::MIN;
                let mut max_point = (0, 0);
                for mov in self.get_legal_moves(pos, false, Vec::new(), 0) {
                    let dist = delta_distance(pos, mov, opponent_point);
                    if dist > max {
                        max = dist;
                        max_point = mov;
                    }
                }
                if max == 0 {
                    None
                } else {
                    Some(max_point)
                }
            };

            if let Some(mov) = best {
                best_moves.push((pos, mov));
            }
        }

        best_moves
            .into_iter()
            .max_by(|&m1, &m2| {
                delta_distance(m1.0, m1.1, opponent_point).cmp(&delta_distance(
                    m2.0,
                    m2.1,
                    opponent_point,
                ))
            })
            .unwrap()
    }

    pub fn random_move(&self, player: i8, thread_rng: &mut ThreadRng) -> (Point, Point) {
        let mut moves: Vec<(Point, Point)> = Vec::new();

        for pos in self.get_player_positons(player) {
            for mov in self.get_legal_moves(pos, false, Vec::new(), 0) {
                moves.push((pos, mov));
            }
        }

        moves[thread_rng.gen_range(0..moves.len())]
    }

    pub fn get_legal_moves(
        &self,
        point: Point,
        jump: bool,
        mut prev: Vec<Point>,
        dim: u32,
    ) -> Vec<Point> {
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
            } else if self.get(surr_point) > 0 {
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

    pub fn get_player_positons(&self, player: i8) -> [Point; 10] {
        let mut players = [(0, 0); 10];
        let mut counter = 0;

        for x in 0..self.width {
            for y in 0..self.height {
                if self.get((x, y)) == player {
                    players[counter] = (x, y);
                    counter += 1;
                }
            }
        }
        players
    }

    fn get_coord_over_piece(
        &self,
        current_point: Point,
        surrounding_point: Point,
    ) -> Option<Point> {
        let v_x = surrounding_point.0 - current_point.0;
        let v_y = surrounding_point.1 - current_point.1;

        if surrounding_point.0 < 0 - v_x
            || surrounding_point.0 + v_x >= self.width
            || surrounding_point.1 < 0 - v_y
            || surrounding_point.1 + v_y >= self.height
        {
            return None;
        }

        let x = surrounding_point.0 + v_x;
        let y = surrounding_point.1 + v_y;

        Some((x, y))
    }

    fn get_surrounding(&self, (x, y): Point) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();

        if x >= 2 {
            points.push((x - 2, y))
        }
        if x >= 1 && y + 1 < self.height {
            points.push((x - 1, y + 1))
        }
        if x + 1 < self.width && y + 1 < self.height {
            points.push((x + 1, y + 1))
        }
        if x + 2 < self.width {
            points.push((x + 2, y))
        }
        if x + 1 < self.width && y >= 1 {
            points.push((x + 1, y - 1))
        }
        if x >= 1 && y >= 1 {
            points.push((x - 1, y - 1))
        }

        points.into_iter().filter(|&p| self.get(p) >= 0).collect()
    }

    pub fn to_string(&self) -> String {
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

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_delta_distance() {
        let to = (12, 15);
        let point1 = (13, 3);
        let point2 = (12, 0);
        let move1 = (13, 7);
        let move2 = (12, 8);

        let dist1 = delta_distance(point1, move1, to);
        let dist2 = delta_distance(point2, move2, to);
        println!("{} < {}", dist1, dist2);

        std::assert!(dist1 < dist2);
    }
}

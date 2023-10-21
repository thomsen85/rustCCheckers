use std::sync::{Arc, Weak, Mutex};
use crate::board::Move;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub mov: Move,
    parent: Option<Weak<Mutex<Node>>>,
    children: Vec<Arc<Mutex<Node>>>,
    pub plays: u32,
    pub result: i32,
}

impl Node {
    pub fn new_root() -> Self {
        Node {
            mov: ((0,0), (0,0)),
            parent: None,
            children: Vec::new(),
            plays: 0,
            result: 0,
        }
    }

    pub fn new_child(mov: Move, parent: Weak<Mutex<Node>>) -> Node{
        Node {
            mov,
            parent: Some(parent),
            children: Vec::new(),
            plays: 0,
            result: 0,
        }
    }

    pub fn backpropegate(&mut self, result: i8) {
        self.result += result as i32;
        self.plays += 1;

        if let Some(parent) = &self.parent {
            if let Some(ptr) = parent.upgrade(){
                ptr.lock().unwrap().backpropegate(result);
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tree {
    pub root: Arc<Mutex<Node>>
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            root: Arc::new(Mutex::new(Node::new_root())),
        }
    }

    pub fn add_game(&mut self, game: Vec<Move>, result: i8) {
        let mut parent = Arc::clone(&self.root);

        for mov in game {

            let mut been_played = false;

            for child in &Arc::clone(&parent).lock().unwrap().children {
                if child.lock().unwrap().mov == mov {
                    parent = Arc::clone(child);
                    been_played = true;
                    break;
                }
            }
            if !been_played {
                let n = Arc::new(Mutex::new(Node::new_child(mov, Arc::downgrade(&mut parent))));
                parent.lock().unwrap().children.push(Arc::clone(&n));
                parent = n;
            } 
        }

        parent.lock().unwrap().backpropegate(result);
    }

    pub fn save_to_csv(&self){
        let csv_header = String::from("ID, DATA, PARENT\n");
        //csv_header.push_str(Self::make_csv_string(self.root.lock().unwrap().children, 0))
        unimplemented!();

    }

    fn make_csv_string(children: Vec<Node>, current_id: u32) -> String {
        let string = String::new();
        unimplemented!();
    }
}

pub mod tests {
    use super::*;
    
    #[test]
    pub fn test_backpropegate() {
        let mut tree = Tree::new();
        let mut game = Vec::with_capacity(3);
        game.push(((15,3),(16,4)));

        game.push(((16,4),(15,5)));

        game.push(((15,5),(16,6)));

        game.push(((16,6),(17,7)));


        tree.add_game(game, 1);

        dbg!(tree);

    }
}
    
 




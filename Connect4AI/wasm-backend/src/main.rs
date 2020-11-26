mod game;

use game::*;

fn main() {


    let mut arr = Vec::new();
    let r1 = [2, 2, 2, 2, 2, 2, 2].to_vec();
    let r2 = [2, 2, 2, 2, 2, 2, 2].to_vec();
    let r3 = [2, 2, 2, 2, 2, 2, 2].to_vec();
    let r4 = [2, 2, 2, 2, 2, 2, 2].to_vec();
    let r5 = [2, 2, 2, 2, 2, 2, 2].to_vec();
    let r6 = [2, 2, 2, 2, 2, 2, 2].to_vec();
    arr.push(r1);
    arr.push(r2);
    arr.push(r3);
    arr.push(r4);
    arr.push(r5);
    arr.push(r6);
    let mut game = Game::new(arr);
    let scores = game.minimax(15);

    for score in scores.iter() {
        println!("{}", score);
    }
}
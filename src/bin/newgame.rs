use sudoku_generator::generator;

fn main() {
  // for _ in 0..100 {
  //   let game = generator::generate_game();
  //   println!("game: {}", game);
    let game = "628519437579423618413786529857932146362147895194865273781354962235691784946278351";
    let puzzle = generator::dig_holes(&game);
    println!("puzz: {}", puzzle);
  // }
}
use mininvaders::{window, Game};

#[macroquad::main(window)]
async fn main() {
    let mut game = Game::new().await;
    game.run().await;
}

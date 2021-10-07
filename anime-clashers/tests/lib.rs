use anime_clashers::player::player::Player;

#[test]
pub fn new_player() {
    let player = Player::new("teste");
    assert_eq!(player.name(), "teste")
}
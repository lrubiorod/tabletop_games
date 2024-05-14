pub trait GameState {
    fn current_player(&self) -> usize;
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Default)]
pub enum State {
    #[default]
    Placed,
    ToPlaced,
    Wrong,
}

#[derive(Clone, Debug, Default)]
pub struct CharState {
    pub state: State,
    pub char: u8,
}
impl CharState {
    pub fn new(state: State, char: u8) -> Self {
        Self { state, char }
    }
}

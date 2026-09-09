#[derive(Clone)]
#[derive(Copy)]
struct Movement<const D: usize> {
    pos: [f32; D],
    speed: [f32; D],
}
impl<const D: usize> Default for Movement<D> {
    fn default() -> Self {
        let tmp: [f32; D] = [0.0; D];
        Self {
            pos: tmp,
            speed: tmp,
        }
    }
}
struct StateSpace {
    ball: Movement<2>,
    players: [Movement<1>; 2],
}
impl Default for StateSpace {
    fn default() -> Self {
        Self {
            ball: Movement::default(),
            players: [Movement::default(); 2],
        }
    }
}
struct ParameterSpace {
    field_size: [i32; 2],
    player_size: [i32; 2],
}
impl Default for ParameterSpace {
    fn default() -> Self {
        Self {
            field_size: [500, 500],
            player_size: [25, 100],
    }
}
fn main() {
    let p = ParameterSpace::default()
    let mut s = StateSpace::default();
    println!("Your speed is {}", s.players[0].speed[0]);
}

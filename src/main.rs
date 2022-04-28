use rltk::RltkBuilder;
use rltk::{GameState, Rltk};

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Zug Zug");
    }
}

fn main() -> rltk::BError {
    let ctx = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let gs = State {};
    rltk::main_loop(ctx, gs)
}

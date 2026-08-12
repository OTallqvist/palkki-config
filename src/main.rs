mod widgets;
const BAR_HEIGHT: u32 = 20;
use std::fmt::Debug;

use palkki::Bar;
fn main() {
    let mut bar = Bar::new(BAR_HEIGHT);
    //TODO:  bar.add_bg(Bg::new)
    bar.add_widgets(&[
        &widgets::Clock::new_dyn,
        &widgets::Ram::new_dyn,
        &widgets::Cpu::new_dyn,
        &widgets::Battery::new_dyn,
    ]);
    bar.run();
}

pub(crate) trait LogErr: Sized {
    fn log(self) -> Self;
}

impl<T, E: Debug> LogErr for Result<T, E> {
    fn log(self) -> Self {
        self.inspect_err(|e| {
            dbg!(e);
        })
    }
}

mod widgets;
const BAR_HEIGHT: u32 = 20;
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

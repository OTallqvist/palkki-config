mod widgets;
const BAR_HEIGHT: u32 = 19;
use palkki::Bar;
fn main() {
    let mut bar = Bar::with_height(BAR_HEIGHT);
    bar.add_widgets(Box::new([
        widgets::Clock::new_dyn(),
        widgets::Ram::new_dyn(),
        widgets::Cpu::new_dyn(),
        widgets::Battery::new_dyn(),
        widgets::Net::new_dyn("wlo1"),
        widgets::Net::new_dyn("eno2"),
    ]));
    bar.run();
}

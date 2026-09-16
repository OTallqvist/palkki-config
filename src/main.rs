mod widgets;
const BAR_HEIGHT: u32 = 19;

use palkki::Bar;
fn main() {
    let mut bar = Bar::with_height(BAR_HEIGHT);
    //TODO:  bar.add_bg(Bg::new)
    bar.add_widgets(&[
        &widgets::Clock::new_dyn,
        &widgets::Ram::new_dyn,
        &widgets::Cpu::new_dyn,
        &widgets::Battery::new_dyn,
    ]);
    bar.run();
}

#[macro_export]
macro_rules! log_pass {
    ($val:expr) => {
        Result::inspect_err($val, |e| {
            std::eprintln!(
                "[{}:{}:{}] {} = {:#?}",
                std::file!(),
                std::line!(),
                std::column!(),
                std::stringify!($val),
                &&e as &dyn std::fmt::Debug,
            );
        })
    };
}

#[macro_export]
macro_rules! log {
    ($val:expr) => {
        let _ = Result::inspect_err({ $val }, |e| {
            std::eprintln!(
                "[{}:{}:{}] {} = {:#?}",
                std::file!(),
                std::line!(),
                std::column!(),
                std::stringify!($val),
                &&e as &dyn std::fmt::Debug,
            );
        });
    };
}

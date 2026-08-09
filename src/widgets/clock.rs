use std::time::Duration;

use palkki::{
    Damage, Vec2,
    widget::{DrawableBlock, Pixel, Positioning, TextPosition, Widget},
};

pub struct Clock {
    last_time: time::Time,
}

impl Clock {
    pub fn new_dyn() -> Box<dyn Widget> {
        Box::new(Self {
            last_time: time::OffsetDateTime::now_local()
                .unwrap_or(time::OffsetDateTime::now_utc())
                .time(),
        })
    }
}

impl Widget for Clock {
    fn postioning(&self, _: Vec2) -> Positioning {
        Positioning::LeftAlign { width: 100 }
    }
    fn redraw(&mut self, block: &mut DrawableBlock) {
        let time_now = time::OffsetDateTime::now_local().unwrap_or(time::OffsetDateTime::now_utc());
        if time_now.time() == self.last_time {
            return;
        }
        self.last_time = time_now.time();
        block.set_bg_color(Pixel::rgb(0x3A, 0x3A, 0x3A));
        let mut time_str = time_now.time().truncate_to_second().to_string();
        //for some reason the string contains a ".0" at the end
        time_str.truncate(time_str.len() - 2);
        let _ = block.draw_text(&time_str, 12., TextPosition::Center, Pixel::WHITE);
        block.damage = Damage::from_0_0(block.block.size)
    }
    fn update_time(&self) -> std::time::Duration {
        Duration::from_millis(200)
    }
}

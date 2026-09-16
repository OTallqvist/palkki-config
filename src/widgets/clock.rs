use std::time::Duration;

use palkki::{
    Rect, Vec2,
    widget::{DrawableBlock, Pixel, Positioning, TextPosition, Widget},
};
use time::Date;

use crate::log;

pub struct Clock {
    last_date: time::Date,
}

impl Clock {
    pub fn new_dyn() -> Box<dyn Widget> {
        Box::new(Self {
            last_date: Date::MIN, //replaced immidiately
        })
    }
}

impl Widget for Clock {
    fn postioning(&self, _: Vec2) -> Positioning {
        Positioning::LeftAlign { width: 150 }
    }
    fn redraw(&mut self, block: &mut DrawableBlock) {
        let now = time::OffsetDateTime::now_local().unwrap_or(time::OffsetDateTime::now_utc());
        let date = now.date();
        block.set_bg_color(Pixel::rgb(0x3A, 0x3A, 0x3A));
        if date != self.last_date {
            let date_str = format!("{}-{}", date.day(), date.month());
            log!(block.draw_text("test", 12., TextPosition::Right { y: 2 }, Pixel::WHITE));
            self.last_date = date;
        }
        let mut time_str = now.time().truncate_to_second().to_string();
        //for some reason the string contains a ".0" at the end
        time_str.truncate(time_str.len() - 2);
        log!(block.draw_text(
            &time_str,
            12.,
            TextPosition::Absolute(Vec2 { x: 1, y: 2 }),
            Pixel::WHITE,
        ));
        block.damage = Rect::from_0_0(block.block.size)
    }
    fn update_time(&self) -> std::time::Duration {
        Duration::from_millis(200)
    }
}

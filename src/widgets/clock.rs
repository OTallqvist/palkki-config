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
const BG_COLOR: Pixel = Pixel::rgb(0x3A, 0x3A, 0x3A);

impl Widget for Clock {
    fn postioning(&self, _: Vec2) -> Positioning {
        Positioning::LeftAlign { width: 120 }
    }
    fn redraw(&mut self, block: &mut DrawableBlock) {
        let now = time::OffsetDateTime::now_local().unwrap_or(time::OffsetDateTime::now_utc());
        let date = now.date();
        const TIME_DATE_SPLIT: u32 = 70;
        log!(block.set_bg_color_region(
            Rect::from_0_0(Vec2::new(TIME_DATE_SPLIT, block.height())),
            BG_COLOR
        ));
        if date != self.last_date {
            log!(block.set_bg_color_region(
                Rect::new(
                    Vec2::from_x(TIME_DATE_SPLIT),
                    Vec2::new(block.width() - TIME_DATE_SPLIT, block.height())
                ),
                BG_COLOR
            ));
            let date_str = format!("{}-{}", date.day(), date.month() as u8);
            log!(block.draw_text(
                &date_str,
                12.,
                TextPosition::Right { y: 2, padd: 5 },
                Pixel::WHITE
            ));
            self.last_date = date;
        }
        let mut time_str = now.time().truncate_to_second().to_string();
        //for some reason the string contains a ".0" at the end
        time_str.truncate(time_str.len() - 2);
        log!(block.draw_text(
            &time_str,
            12.,
            TextPosition::Left { padd: 3, y: 2 },
            Pixel::WHITE,
        ));
        block.damage = Rect::from_0_0(block.block.size)
    }
    fn update_time(&self) -> std::time::Duration {
        Duration::from_millis(200)
    }
}

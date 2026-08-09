use std::{
    fs::File,
    io::{Read, Seek},
    num::NonZero,
    str::FromStr,
};

use palkki::{
    Damage, Vec2,
    widget::{DrawableBlock, Pixel, Positioning, TextPosition, Widget},
};

const BATTERY_PATH: &str = "/sys/class/power_supply/BAT0";

#[derive(PartialEq, Clone, Copy)]
enum BatteryStatus {
    Full,
    Discharging,
    Charging,
}

impl FromStr for BatteryStatus {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Discharging" => Ok(BatteryStatus::Discharging),
            "Charging" => Ok(BatteryStatus::Charging),
            "Not charging" => Ok(BatteryStatus::Full),
            _ => Err(()),
        }
    }
    type Err = ();
}

pub struct Battery {
    ///Battery percentage in tenths of a percent
    prev_battery_permillage: u16,
    prev_status: BatteryStatus,
    energy_full: f32,
    energy_now_file: File,
    status_file: File,
}

impl Battery {
    pub(crate) fn new_dyn() -> Box<dyn Widget> {
        let mut energy_full = String::new();
        File::open(format!("{BATTERY_PATH}/energy_full"))
            .unwrap_or_else(|_| panic!("Failed to read {BATTERY_PATH}/energy_full. Make sure you have the right device path for your battery"))
            .read_to_string(&mut energy_full)
            .unwrap();
        //remove newline from end
        energy_full.truncate(energy_full.len() - 1);
        let energy_full = energy_full.parse::<f32>().unwrap();
        Box::new(Self {
            prev_battery_permillage: u16::MAX, //gets updated
            energy_full,
            energy_now_file: File::open(format!("{BATTERY_PATH}/energy_now")).unwrap(),
            status_file: File::open(format!("{BATTERY_PATH}/status")).unwrap(),
            prev_status: BatteryStatus::Full,
        })
    }
    fn get_battery_permillage(&mut self) -> Option<NonZero<u16>> {
        let mut energy_now = String::new();
        self.energy_now_file.read_to_string(&mut energy_now).ok()?;
        let _ = self.energy_now_file.rewind();
        //remove newline from end
        energy_now.truncate(energy_now.len() - 1);
        let energy_now = energy_now.parse::<f32>().unwrap();
        let permillage = (energy_now / self.energy_full * 1000.) as u16;
        if permillage == self.prev_battery_permillage {
            None
        } else {
            self.prev_battery_permillage = permillage;
            NonZero::new(permillage)
        }
    }
    fn get_battery_status(&mut self) -> Option<BatteryStatus> {
        let mut status = String::new();
        self.status_file.read_to_string(&mut status).unwrap();
        let _ = self.status_file.rewind();
        let status = status.trim();
        let status = status.parse().ok()?;
        if status == self.prev_status {
            None
        } else {
            self.prev_status = status;
            Some(status)
        }
    }
    //TODO:
    fn draw_battery_icon(&mut self, block: &mut DrawableBlock, color: Pixel, width: u32) {
        const ICON_HEIGHT: u32 = 8;
        let offset = Vec2 {
            x: 1,
            y: (block.height() - ICON_HEIGHT) / 2,
        };
        draw_battery_icon_ends(block, offset, width, ICON_HEIGHT, color);
        let lines = offset.y..offset.y + ICON_HEIGHT;
        for y in lines.clone() {
            let line_len = width as usize;
            let line = block
                .mutate_line(Vec2 { x: offset.x, y }, line_len)
                .unwrap();
            line.iter_mut()
                .enumerate()
                .filter(|(x, _)| *x == 0 || *x == line_len - 1)
                .for_each(|(_, px)| *px = color);
        }
        block
            .set_pixel(Vec2::new(width, ICON_HEIGHT / 2 - 1) + offset, color)
            .unwrap();
        block
            .set_pixel(Vec2::new(width, ICON_HEIGHT / 2) + offset, color)
            .unwrap();
        block
            .set_pixel(Vec2::new(width, ICON_HEIGHT / 2 + 1) + offset, color)
            .unwrap();
    }
}

fn draw_battery_icon_ends(
    block: &mut DrawableBlock,
    offset: Vec2,
    width: u32,
    height: u32,
    color: Pixel,
) {
    //top
    block
        .mutate_line(offset, width as usize)
        .unwrap()
        .iter_mut()
        .for_each(|px| *px = color);
    //bottom
    block
        .mutate_line(offset + Vec2::from_y(height), width as usize)
        .unwrap()
        .iter_mut()
        .for_each(|px| *px = color);
}

impl Widget for Battery {
    fn redraw(&mut self, block: &mut palkki::widget::DrawableBlock) {
        let permillage = self.get_battery_permillage();
        let status = self.get_battery_status();
        if permillage.is_none() && status.is_none() {
            return;
        }
        let permillage = permillage.unwrap_or(NonZero::new(self.prev_battery_permillage).unwrap());
        let status = status.unwrap_or(self.prev_status);
        let text_color = match status {
            BatteryStatus::Full => Pixel::rgb(255, 255, 255),
            BatteryStatus::Discharging => Pixel::rgb(255, 100, 100),
            BatteryStatus::Charging => Pixel::rgb(100, 255, 100),
        };
        block.set_bg_color(Pixel::rgb(0x3A, 0x3A, 0x3A));
        // self.draw_battery_icon(block, text_color, 17);
        let percentage = format!("b:{:.1}%", u16::from(permillage) as f32 / 10.);
        let _ = block.draw_text(&percentage, 12., TextPosition::Center, text_color);
        block.damage = Damage::from_0_0(block.block.size)
    }
    fn postioning(&self, _: palkki::Vec2) -> palkki::widget::Positioning {
        Positioning::RightAlign { width: 70 }
    }
}

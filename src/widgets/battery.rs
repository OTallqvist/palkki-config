use crate::LogErr;
use std::{
    fs::File,
    io::{Read, Seek},
    num::NonZero,
    str::FromStr,
};

use dbg_if::dbg_if_hash_ne;
use palkki::{
    Rect,
    widget::{Pixel, Positioning, TextPosition, Widget},
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
    prev_power: u32,
    energy_full: f32,
    energy_now_file: File,
    status_file: File,
    power_now_file: File,
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
            prev_power: 0,
            energy_now_file: File::open(format!("{BATTERY_PATH}/energy_now")).unwrap(),
            status_file: File::open(format!("{BATTERY_PATH}/status")).unwrap(),
            power_now_file: File::open(format!("{BATTERY_PATH}/power_now")).unwrap(),
            prev_status: BatteryStatus::Full,
        })
    }

    fn get_battery_permillage(&mut self) -> Option<NonZero<u16>> {
        let mut energy_now = String::new();
        self.energy_now_file
            .read_to_string(&mut energy_now)
            .log()
            .ok()?;
        let _ = self.energy_now_file.rewind();
        //remove newline from end
        energy_now.truncate(energy_now.len() - 1);
        let energy_now = energy_now.parse::<f32>().log().ok()?;
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
        self.status_file.read_to_string(&mut status).log().ok()?;
        let _ = self.status_file.rewind();
        let status = status.trim();
        let status = status.parse().log().ok()?;
        if status == self.prev_status {
            None
        } else {
            self.prev_status = status;
            Some(status)
        }
    }

    //returns power in deciWatts
    fn get_power(&mut self) -> Option<u32> {
        let mut power_now = String::new();
        self.power_now_file
            .read_to_string(&mut power_now)
            .log()
            .ok()?;
        let _ = self.power_now_file.rewind();
        power_now.truncate(power_now.len() - 1);
        let power_now = power_now.parse::<u32>().log().ok()? / 100_000;
        if power_now == self.prev_power {
            None
        } else {
            self.prev_power = power_now;
            Some(power_now)
        }
    }
}

impl Widget for Battery {
    fn redraw(&mut self, block: &mut palkki::widget::DrawableBlock) {
        let permillage = self.get_battery_permillage();
        let status = self.get_battery_status();
        let power = self.get_power();
        if permillage.is_none() && status.is_none() && power.is_none() {
            return;
        }
        let permillage = permillage.unwrap_or(NonZero::new(self.prev_battery_permillage).unwrap());
        let status = status.unwrap_or(self.prev_status);
        let power = power.unwrap_or(self.prev_power);
        let text_color = match status {
            BatteryStatus::Full => Pixel::rgb(255, 255, 255),
            BatteryStatus::Discharging => Pixel::rgb(255, 100, 100),
            BatteryStatus::Charging => Pixel::rgb(100, 255, 100),
        };
        block.set_bg_color(Pixel::rgb(0x3A, 0x3A, 0x3A));
        let display_text = if u16::from(permillage) < 1000 {
            format!(
                "b:{:.1}% {:.1}W",
                u16::from(permillage) as f32 / 10.,
                power as f32 / 10.
            )
        } else {
            format!(
                "b:{:.0}% {:.1}W",
                u16::from(permillage) as f32 / 10.,
                power as f32 / 10.
            )
        };
        block
            .draw_text(&display_text, 12., TextPosition::Center, text_color)
            .unwrap();
        block.damage = Rect::from_0_0(block.block.size)
    }
    fn postioning(&self, _: palkki::Vec2) -> palkki::widget::Positioning {
        Positioning::RightAlign { width: 120 }
    }
}

use std::{
    fmt,
    fs::File,
    io::{Read, Seek},
    num::NonZero,
    str::FromStr,
};

use anyhow::Context;
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

#[derive(Debug)]
struct StatusParseError;

impl fmt::Display for StatusParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StatusParseError")
    }
}

impl std::error::Error for StatusParseError {}

impl FromStr for BatteryStatus {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Discharging" => Ok(BatteryStatus::Discharging),
            "Charging" => Ok(BatteryStatus::Charging),
            "Not charging" => Ok(BatteryStatus::Full),
            _ => Err(StatusParseError),
        }
    }
    type Err = StatusParseError;
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

    fn get_battery_charge(&mut self) -> anyhow::Result<Option<NonZero<u16>>> {
        let mut energy_now = String::new();
        if self
            .energy_now_file
            .read_to_string(&mut energy_now)
            .is_err()
        {
            return Ok(None);
        }
        self.energy_now_file.rewind().context("energy_now rewind")?;
        //remove newline from end
        energy_now.truncate(energy_now.len() - 1);
        let energy_now = energy_now
            .parse::<f32>()
            .context(format!("energy_now parse: energy_now = {energy_now}"))?;
        let charge = (energy_now / self.energy_full * 10000.) as u16;
        if charge == self.prev_battery_permillage {
            Ok(None)
        } else {
            self.prev_battery_permillage = charge;
            Ok(NonZero::new(charge))
        }
    }

    fn get_battery_status(&mut self) -> anyhow::Result<Option<BatteryStatus>> {
        let mut status = String::new();
        if self.status_file.read_to_string(&mut status).is_err() {
            return Ok(None);
        }
        self.status_file.rewind().context("status_file rewind")?;
        let status = status.trim();
        let status = status
            .parse()
            .context(format!("bat status parse: status = {status}"))?;
        if status == self.prev_status {
            Ok(None)
        } else {
            self.prev_status = status;
            Ok(Some(status))
        }
    }

    //returns power in deciWatts
    fn get_power(&mut self) -> anyhow::Result<Option<u32>> {
        let mut power_now = String::new();
        if self.power_now_file.read_to_string(&mut power_now).is_err() {
            return Ok(None);
        }
        self.power_now_file.rewind().context("power_now rewind")?;
        power_now.truncate(power_now.len() - 1);
        let power_now = "semi tavi";
        let power_now = power_now
            .parse::<u32>()
            .context(format!("power_now parse: power_now = {power_now}"))?
            / 100_000;
        if power_now == self.prev_power {
            Ok(None)
        } else {
            self.prev_power = power_now;
            Ok(Some(power_now))
        }
    }
}

impl Widget for Battery {
    fn name(&self) -> &'static str {
        "Battery"
    }
    fn redraw(&mut self, block: &mut palkki::widget::DrawableBlock) -> Result<(), anyhow::Error> {
        let permillage = self.get_battery_charge()?;
        let status = self.get_battery_status()?;
        let power = self.get_power()?;
        if permillage.is_none() && status.is_none() && power.is_none() {
            return Ok(());
        }
        let charge = permillage.unwrap_or(NonZero::new(self.prev_battery_permillage).unwrap());
        let status = status.unwrap_or(self.prev_status);
        let power = power.unwrap_or(self.prev_power);
        let text_color = match status {
            BatteryStatus::Full => Pixel::rgb(255, 255, 255),
            BatteryStatus::Discharging => Pixel::rgb(255, 100, 100),
            BatteryStatus::Charging => Pixel::rgb(100, 255, 100),
        };
        block.set_bg_color(Pixel::rgb(0x3A, 0x3A, 0x3A));
        let display_text = if u16::from(charge) < 10000 {
            format!(
                "{:.2}% {:.1}W",
                u16::from(charge) as f32 / 100.,
                power as f32 / 10.
            )
        } else {
            format!(
                "{:.0}% {:.1}W",
                u16::from(charge) as f32 / 100.,
                power as f32 / 10.
            )
        };
        block
            .draw_text(&display_text, 12., TextPosition::Center, text_color)
            .unwrap();
        block.damage = Rect::from_0_0(block.block.size);
        Ok(())
    }
    fn postioning(&self, _: palkki::Vec2) -> palkki::widget::Positioning {
        Positioning::RightAlign { width: 150 }
    }
}

use std::{
    fs::File,
    io::{Read, Seek},
};

use palkki::{
    Damage,
    widget::{Pixel, Positioning, TextPosition, Widget},
};

pub struct Cpu {
    last_usage: u8,
    last_work_jiffies: u32,
    last_total_jiffies: u32,
    proc_stat_file: File,
}

impl Cpu {
    fn get_cpu_usage(&mut self) -> Option<u8> {
        let usage = self.take_usage_measurement();
        let usage = (usage * 100.) as u8;
        if usage == self.last_usage {
            return None;
        }
        self.last_usage = usage;
        Some(usage)
    }
    fn take_usage_measurement(&mut self) -> f32 {
        let mut buf = [0; 100];
        self.proc_stat_file.read_exact(&mut buf).unwrap();
        self.proc_stat_file.rewind().unwrap();
        let mut line = str::from_utf8(&buf).unwrap().lines().next().unwrap();
        line = line.split_at(5).1;
        let mut number_strings = line.split_whitespace();
        let total_jiffies: u32 = number_strings
            .clone()
            .map(|str| str.parse::<u32>().unwrap())
            .sum::<u32>();
        let work_jiffies = total_jiffies - number_strings.nth(3).unwrap().parse::<u32>().unwrap();
        let work_jiffies_diff = work_jiffies - self.last_work_jiffies;
        let total_jiffies_diff = total_jiffies - self.last_total_jiffies;
        let usage = work_jiffies_diff as f32 / total_jiffies_diff as f32;
        self.last_work_jiffies = work_jiffies;
        self.last_total_jiffies = total_jiffies;
        usage
    }
    pub fn new_dyn() -> Box<dyn Widget> {
        Box::new(Self {
            last_usage: 255, //so that usage gets recalculated on the first frame
            last_work_jiffies: 0,
            last_total_jiffies: 0,
            proc_stat_file: File::open("/proc/stat").unwrap(),
        })
    }
}

impl Widget for Cpu {
    fn postioning(&self, _: palkki::Vec2) -> palkki::widget::Positioning {
        Positioning::RightAlign { width: 40 }
    }
    fn redraw(&mut self, block: &mut palkki::widget::DrawableBlock) {
        let Some(usage) = self.get_cpu_usage() else {
            return;
        };
        block.set_bg_color(Pixel::rgb(0x2A, 0x2A, 0x2A));
        let usage = format!("{usage}%");
        let _ = block.draw_text(&usage, 12., TextPosition::Center, Pixel::WHITE);
        block.damage = Damage::from_0_0(block.block.size)
    }
}

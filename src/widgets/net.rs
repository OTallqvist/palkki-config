use std::{net::Ipv4Addr, process::Command};

use anyhow::Context;
use palkki::{
    Rect,
    widget::{Pixel, Widget},
};

pub struct Net {
    ip: Option<Ipv4Addr>,
    device: &'static str,
}

impl Net {
    pub fn new_dyn(device: &'static str) -> Box<dyn Widget> {
        Box::new(Self {
            ip: Some(Ipv4Addr::UNSPECIFIED),
            device,
        })
    }
}

impl Widget for Net {
    fn name(&self) -> &'static str {
        "Network"
    }
    fn postioning(&self, _: palkki::Vec2) -> palkki::widget::Positioning {
        palkki::widget::Positioning::LeftAlign { width: 150 }
    }
    fn redraw(&mut self, block: &mut palkki::widget::DrawableBlock) -> anyhow::Result<()> {
        let ip = get_ipv4(self.device)?;
        if ip == self.ip {
            return Ok(());
        }
        self.ip = ip;
        block.set_bg_color(Pixel::rgb(0x20, 0x20, 0x20));
        if let Some(ip) = ip {
            block.draw_text(
                &format!("{:.2} {}", self.device, ip),
                12.,
                palkki::widget::TextPosition::Center,
                Pixel::rgb(100, 255, 100),
            )?
        } else {
            block.draw_text(
                &format!("{:.2} DOWN", self.device),
                12.,
                palkki::widget::TextPosition::Center,
                Pixel::rgb(255, 100, 100),
            )?
        }
        block.damage = Rect::from_0_0(block.block.size);
        Ok(())
    }
}

#[derive(Debug)]
struct ParseError;
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error")
    }
}

impl std::error::Error for ParseError {}

fn get_ipv4(device_name: &str) -> anyhow::Result<Option<Ipv4Addr>> {
    let ip_cmd = Command::new("/usr/bin/ip")
        .args(["addr", "show", device_name])
        .output()?;
    // the command outputs three lines if there is no network
    let ip_row = ip_cmd
        .stdout
        .split(|b| *b == b'\n')
        .nth(4)
        .ok_or(ParseError)
        .context("getting ip line")?;
    if ip_row.is_empty() {
        return Ok(None);
    }
    let ip = ip_row
        .trim_ascii_start()
        .split(|b| *b == b' ')
        .nth(1)
        .ok_or(ParseError)
        .context("parsing ip line")?;
    let ip = str::from_utf8(ip.split(|b| *b == b'/').next().ok_or(ParseError)?)?;
    Ok(Some(ip.parse().context(format!("parsing ip {ip}"))?))
}

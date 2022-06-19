pub mod terminal;
pub mod porcelain;

use crate::formatters::{
    porcelain::PorcelainFormatter,
    terminal::TerminalFormatter
};

pub trait Formatter {
    fn output(&self);
}

pub fn get_formatter(porcelain: &bool) -> Box< dyn Formatter> {
    if *porcelain {
        Box::new(porcelain::PorcelainFormatter {})
    }

    else {
        Box::new(terminal::TerminalFormatter {})
    }
}
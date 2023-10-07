use super::super::Error;

use super::field::*;

pub trait Action : Field {
    fn get_kind(&self) -> u16;
}



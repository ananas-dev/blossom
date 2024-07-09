use std::fmt::Display;

use crate::str_buffer::StrBuffer;

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum Sex {
    None = 0,
    Man = 1,
    Woman = 2,
}

impl Display for Sex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sex::None => "",
            Sex::Man => "m",
            Sex::Woman => "w",
        }
        .fmt(f)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum Title {
    None = 0,
    Gm = 1,
    Im = 2,
    Fm = 3,
    Cm = 4,
    Wgm = 5,
    Wim = 6,
    Wfm = 7,
    Wcm = 8,
}

impl Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Title::None => "",
            Title::Gm => "GM",
            Title::Im => "IM",
            Title::Fm => "FM",
            Title::Cm => "CM",
            Title::Wgm => "WGM",
            Title::Wim => "WIM",
            Title::Wfm => "WFM",
            Title::Wcm => "WCM",
        }
        .fmt(f)
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: StrBuffer<34>,
    pub federation: StrBuffer<4>,
    pub sex: Sex,
    pub title: Title,
    pub fide_rating: i32,
    pub fide_id: StrBuffer<12>,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            name: StrBuffer::new(),
            federation: StrBuffer::new(),
            sex: Sex::None,
            title: Title::None,
            fide_rating: 0,
            fide_id: StrBuffer::new(),
        }
    }
}

pub struct Tournament {
    pub name: StrBuffer<50>,
    pub city: StrBuffer<50>,
    pub federation: StrBuffer<50>,
    pub chief_arbiter: StrBuffer<50>,
    // date_of_rounds: Vec<StrBuffer<50>>,
}

impl Default for Tournament {
    fn default() -> Self {
        Tournament {
            name: StrBuffer::new(),
            city: StrBuffer::new(),
            federation: StrBuffer::new(),
            chief_arbiter: StrBuffer::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum GameResult {
    None = 0,
    WhiteForfeitWin = 1,
    BlackForfeitWin = 2,
    WhiteUnratedWin = 3,
    BlackUnratedLoss = 4,
    UnratedDraw = 5,
    WhiteWin = 6,
    BlackWin = 7,
    Draw = 8,
    HalfPointBye = 9,
    FullPointBye = 10,
    PairingBye = 11,
    ZeroPointBye = 12,
}

pub struct Game {
    pub white: usize,
    pub black: usize,
    pub result: GameResult,
}

pub struct Round {
    pub games: Vec<Game>,
}

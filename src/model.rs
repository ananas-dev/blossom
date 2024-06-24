#[derive(Debug)]
#[repr(u8)]
pub enum Sex {
    None = 0,
    Man = 1,
    Woman = 2,
}

#[derive(Debug)]
#[repr(u8)]
pub enum Title {
    None = 0,
    Gm = 1,
    Im = 2,
    Fm = 3,
    Cm = 4,
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub federation: String,
    pub sex: Sex,
    pub title: Title,
    pub fide_rating: i32,
    pub fide_id: String,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            name: String::new(),
            federation: String::new(),
            sex: Sex::None,
            title: Title::None,
            fide_rating: 0,
            fide_id: String::new(),
        }
    }
}

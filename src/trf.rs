use crate::model::{Player, Tournament};
use std::fmt::Write;

pub fn export(tournament: &Tournament, players: &[Player]) -> Result<String, ()> {
    let mut trf = String::new();

    write!(&mut trf, "012 {}\r", tournament.name.as_str().unwrap()).unwrap();
    write!(&mut trf, "022 {}\r", tournament.city.as_str().unwrap()).unwrap();
    write!(
        &mut trf,
        "032 {}\r",
        tournament.federation.as_str().unwrap()
    )
    .unwrap();
    write!(
        &mut trf,
        "102 {}\r",
        tournament.chief_arbiter.as_str().unwrap()
    )
    .unwrap();

    // todo
    write!(&mut trf, "XXR {}\r", 9).unwrap();
    write!(&mut trf, "XXC white1\r").unwrap();

    for (row, player) in players.iter().enumerate() {
        write!(
            &mut trf,
            "001 {:>4} {:1} {:>2} {:<33} {:>4} {:>3} {:>11} {:<11} {:>3} {:>4}\r",
            row + 1,
            player.sex,
            player.title,
            player.name.as_str().unwrap(),
            player.fide_rating,
            player.federation.as_str().unwrap(),
            player.fide_id.as_str().unwrap(),
            "1978",
            "0.0",
            row + 1,
        )
        .unwrap();
    }

    Ok(trf)
}

#[cfg(test)]
mod tests {
    use crate::{
        model::{Sex, Title},
        str_buffer::StrBuffer,
    };

    use super::*;

    #[test]
    fn format_player() {
        let tournament = Tournament::default();
        let mut players = Vec::new();

        players.push(Player {
            name: StrBuffer::from_str("Mirzoev Azer"),
            federation: StrBuffer::from_str("AZE"),
            sex: Sex::Man,
            title: Title::GM,
            fide_rating: 2527,
            fide_id: StrBuffer::from_str("13400304"),
        });

        assert_eq!(
            export(&tournament, &players).unwrap(),
            "001    1 m  g Mirzoev Azer                      2527 AZE    13400304 1978        0.0    1\r".to_string()
        );
    }
}

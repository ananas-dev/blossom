use crate::model::{Player, Tournament};
use crate::trf;
use std::fs::{self, File};
use std::io::prelude::*;
use std::process::Command;
use std::str;

pub fn pair_players(
    tournament: &Tournament,
    players: &[Player],
    engine_path: &str,
) -> Vec<(i32, i32)> {
    // write trf to temp file
    let temp_file_path = "pairing_input.txt";

    let mut file = File::create(temp_file_path).unwrap();

    let trf = trf::export(tournament, players).unwrap();

    file.write_all(trf.as_bytes()).unwrap();
    file.flush().unwrap();

    // parse results
    let output = Command::new(engine_path)
        .args(["--dutch", temp_file_path, "-p"])
        .output()
        .expect("cannot use execute pairing command");

    if !output.status.success() {
        // panic!(format!("{}", str::from_utf8(&output.stderr).unwrap()));
        println!("{}", str::from_utf8(&output.stderr).unwrap());
        panic!()
    }

    let stdout = str::from_utf8(&output.stdout).expect("AAAA");

    let pairings = stdout
        .lines()
        .skip(1)
        .map(|l| {
            let mut ids = l.split_whitespace();
            let first = ids.next().unwrap().parse().unwrap();
            let second = ids.next().unwrap().parse().unwrap();
            (first, second)
        })
        .collect();

    fs::remove_file(temp_file_path).expect("could not remove temp file");

    pairings
}

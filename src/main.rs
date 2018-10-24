use std::fs;
use std::env;
use std::process;

extern crate gio;

use gio::SettingsExt;

extern crate rand;
use rand::Rng;

use std::time::Duration;
use std::thread;

fn main() {

    //Setup Rotae
    let dir: String;
    match env::home_dir() {
        Some(path) => {
            let path = path.as_path().join(".rotae");
            fs::create_dir_all(&path).unwrap();
            dir = path.into_os_string().into_string().unwrap();
        },
        None => {
            println!("Unable to get your home dir!");
            process::exit(1);
        }
    }

    let settings = gio::Settings::new_with_path("org.bmandesigns.rotae", "/org/bmandesigns/rotae/");
    let gnome = gio::Settings::new_with_path("org.gnome.desktop.background", "/org/gnome/desktop/background/");

    loop {
        let mut pictures: Vec<fs::DirEntry> = Vec::new();
        let paths = fs::read_dir(&dir).unwrap();
        for path in paths {
            let path = path.unwrap();
            let active = settings.get_strv("active-folders");
            if path.file_type().unwrap().is_dir() {
                let p = path.path();
                let name = p.file_name().unwrap().to_str().unwrap();
                if active.iter().any(|x| x == name) {
                    let pics = fs::read_dir(&p).unwrap();
                    for pic in pics {
                        pictures.push(pic.unwrap());
                    }
                }
            }
        }
        let picture = rand::thread_rng().choose(&pictures).unwrap();
        gnome.set_string("picture-uri", &format!("file://{}", picture.path().to_str().unwrap()));
        thread::sleep(Duration::from_secs(settings.get_int("delay") as u64));
    }
}

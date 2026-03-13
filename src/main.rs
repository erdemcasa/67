use rfd::FileDialog;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{PathBuf};
use std::process::Command;
use image::{ImageBuffer, Luma};


fn main() {
    let input = get_input("Appuyez sur [Entrée] pour l'explorateur ou tapez le chemin : ");

    let path_buf: Option<PathBuf> = if input.is_empty() {
        FileDialog::new().set_title("SÉLECTIONNEZ UN FICHIER").pick_file()
    } else {
        Some(PathBuf::from(input))
    };

    if let Some(path) = path_buf {
        let _ = process_file(path);
    }
}

fn process_file(path: PathBuf) -> io::Result<()> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut matches = Vec::new();

    for (index, line_result) in reader.lines().enumerate() {
        if let Ok(line) = line_result {
            if line.contains("67") {
                matches.push((index + 1, line));
            }
        }
    }

    if matches.len() == 67 {
        println!("\x1b[46;30m INCROYABLE : 67 OCCURRENCES ! \x1b[0m");
        //generate_png_from_bytes().expect("Erreur lors de la création du PNG");
    } else {
        println!("\nOccurrences : {} dans {:?}", matches.len(), path);
    }

    Ok(())
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur");
    input.trim().to_string()
}
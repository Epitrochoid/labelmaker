use chrono::Utc;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs::copy;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::tempdir;

use eframe::egui;
use egui::Key;

fn main() -> Result<(), eframe::Error> {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let temp_dir = tempdir().expect("Could not create temp directory");
    let temp_filename = fill_name(&args.name_template, &random_string());
    let temp_filepath = temp_dir.path().join(temp_filename);

    call_command(&args.command, &temp_filepath);

    let initial_state = Labelmaker {
        name_entry: "".to_owned(),
        save_clicked: false,
        temp_filepath,
        path: args.path,
        name_template: args.name_template,
    };

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(410.0, 40.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Labelmaker",
        options,
        Box::new(|_cc| Box::new(initial_state)),
    )
}

#[derive(Debug)]
struct LabelmakerArgs {
    command: String,
    path: PathBuf,
    name_template: String,
}

fn parse_args() -> Result<LabelmakerArgs, pico_args::Error> {
    let mut raw_args = pico_args::Arguments::from_env();

    let args = LabelmakerArgs {
        command: raw_args.value_from_str("--command")?,
        path: raw_args.value_from_os_str("--path", parse_path)?,
        name_template: raw_args.value_from_str("--name")?,
    };

    Ok(args)
}

fn parse_path(s: &std::ffi::OsStr) -> Result<PathBuf, &'static str> {
    Ok(s.into())
}

struct Labelmaker {
    name_entry: String,
    save_clicked: bool,
    temp_filepath: PathBuf,
    path: PathBuf,
    name_template: String,
}

impl eframe::App for Labelmaker {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.save_clicked {
            let filename = fill_name(&self.name_template, &self.name_entry);
            let filepath = &self.path.join(filename);

            // Copy in case /tmp is on another filesystem. Temp directories and any containing files are
            // deleted when the TempDir value goes out of scope at the end of main()
            copy(&self.temp_filepath, filepath).expect("Could not move file to final location");

            frame.close();
        }

        if ctx.input(|i| i.key_pressed(Key::Enter)) {
            self.save_clicked = true
        }

        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            frame.close();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let name_label = ui.label("File Name:");
                ui.text_edit_singleline(&mut self.name_entry)
                    .labelled_by(name_label.id)
                    .request_focus();
                if ui.button("Save").clicked() {
                    self.save_clicked = true
                }
            });
        });
    }
}

fn fill_name(template_string: &str, name: &str) -> String {
    let now = Utc::now();

    template_string
        .to_owned()
        .replace("<name>", name)
        .replace("<timestamp>", &now.format("%F").to_string())
}

fn call_command(command: &str, filepath: &Path) {
    let mut full_command = command.to_owned();
    full_command.push(' ');
    full_command.push_str(
        filepath
            .to_str()
            .expect("Could not convert filepath to string"),
    );
    Command::new("sh")
        .arg("-c")
        .arg(full_command)
        .output()
        .expect("Failed to execute");
}

fn random_string() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}

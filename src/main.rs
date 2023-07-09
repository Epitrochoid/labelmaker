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
    println!("{:?}", args);

    let initial_state = Labelmaker {
        name_entry: "".to_owned(),
        save_clicked: false,
        args: args,
    };

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(410.0, 80.0)),
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
    path: std::path::PathBuf,
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

fn parse_path(s: &std::ffi::OsStr) -> Result<std::path::PathBuf, &'static str> {
    Ok(s.into())
}

struct Labelmaker {
    name_entry: String,
    save_clicked: bool,
    args: LabelmakerArgs,
}

impl eframe::App for Labelmaker {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.save_clicked {
            let filename = fill_name(self.args.name_template.clone(), &self.name_entry);
            println!("{}", filename);
            frame.close();
        }

        if ctx.input(|i| i.key_pressed(Key::Enter)) {
            self.save_clicked = true
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

fn fill_name(template_string: String, name: &str) -> String {
    template_string.replace("<name>", name)
}

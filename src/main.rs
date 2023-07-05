use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };
    println!("{:?}", args);

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(410.0, 80.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Labelmaker",
        options,
        Box::new(|_cc| Box::<Labelmaker>::default()),
    )
}


#[derive(Debug)]
struct LabelmakerArgs {
    command: std::path::PathBuf
}


fn parse_args() -> Result<LabelmakerArgs, pico_args::Error> {
    let mut raw_args = pico_args::Arguments::from_env();

    let args = LabelmakerArgs {
        command: raw_args.value_from_os_str("--command", parse_path)?
    };

    Ok(args)
}

fn parse_path(s: &std::ffi::OsStr) -> Result<std::path::PathBuf, &'static str> {
    Ok(s.into())
}

struct Labelmaker {
    name_entry: String,
}

impl Default for Labelmaker {
    fn default() -> Self {
        Self {
            name_entry: "".to_owned(),
        }
    }
}

impl eframe::App for Labelmaker {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let name_label = ui.label("File Name:");
                ui.text_edit_singleline(&mut self.name_entry)
                    .labelled_by(name_label.id);
                if ui.button("Save").clicked() {
                    println!("{}", self.name_entry)
                }
            });
        });
    }
}

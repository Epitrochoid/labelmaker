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
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
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
    name: String,
    age: u32,
}

impl Default for Labelmaker {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            age: 0,
        }
    }
}

impl eframe::App for Labelmaker {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}

use clap::Parser;
use spinners::{Spinner, Spinners};
use std::fs;
use std::process::{Command, Stdio};


/// Quickly initialise and configure React+Typescript projects with tailwindcss and other dependancies.
/// Currently compatible with: Vite, Next JS
#[derive(Parser, Debug)]
#[clap(author="Oscar Pickerill <me@oscars.dev>", version="v0.1.0", about, long_about = None, usage = "quick-init <PROJECT NAME> [OPTIONS]")]

struct Args {
    /// Name of the project to be initialised
    name: String,
    
    /// [vite|next]
    #[clap(short, long, value_parser, default_value = "vite")]
    template: String,
    
    /// Use Javascript instead of Typescript
    #[clap(short, long = "javascript", value_parser, default_value = "false")]
    js: bool,
}

fn main() {
    // define dependancies
    // TODO: have these loaded from a config file
    let dev_dependancies = vec!["tailwindcss", "postcss", "autoprefixer"];
    let project_dependancies = vec!["react-router-dom"];
    let valid_templates = vec!["vite", "next"];
    
    // extract arguments
    let args = Args::parse();

    // check if template is valid
    if !valid_templates.contains(&args.template.as_str()) {
        println!("Invalid template {}. Valid templates are: {}", &args.template, valid_templates.join(", "));
        std::process::exit(1);
    }

    // define current working directory
    let cwd = std::env::current_dir().unwrap();

    // define final project directory
    let project_dir = cwd.join(&args.name);

    // TODO: panic if template is not valid
    // ...


    // project initialisation
    let init_command = generate_init_command(&args);
    let mut sp = Spinner::new(
        Spinners::Dots9,
        format!("Starting new {} project", &args.template).into(),
    );
    dynamic_exec(init_command, &cwd);
    sp.stop_and_persist("⚡", "Project created".to_string());

    // dependancy installation
    install_dependancies(&dev_dependancies, &project_dir, true);
    install_dependancies(&project_dependancies, &project_dir, false);

    // tailwindcss config
    if dev_dependancies.contains(&"tailwindcss") {
        init_tailwind(&args, &project_dir);
    }

    println!("\nQuick init complete!");

    // give option to run development server
    start_server(&project_dir);

    // end script
    std::process::exit(1);
}

/// execute a terminal command for the current OS
fn dynamic_exec(command: Vec<&str>, dir: &std::path::PathBuf) -> std::process::Output {
    // define current OS and use appropriate command
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .args(command)
            .current_dir(dir)
            .output()
            .expect("failed to execute process")
    } else {
        Command::new(&command[0])
            // .arg("-c")
            .args(&command[1..])
            .current_dir(dir)
            .output()
            .expect("failed to execute process")
    };
    return output;
}

/// determine initial project creation command
fn generate_init_command(args: &Args) -> Vec<&str> {
    // define initial project creation command
    match &args.template.as_str() {
        &"vite" => {
            return vec![
                "npm",
                "create",
                "vite@latest",
                &args.name,
                "--",
                "--template",
                if args.js { "react" } else { "react-ts" },
                "&&",
                "cd",
                &args.name,
                "&&",
                "npm",
                "install",
            ];
        }
        &"next" => {
            return vec![
                "npx",
                "create-next-app@latest",
                &args.name,
                if !args.js { "--typescript" } else { "" },
                "--use-npm",
            ];
        }
        _ => {
            return vec![""];
        }
    }
}

/// install dependancies
fn install_dependancies(dependancies: &Vec<&str>, dir: &std::path::PathBuf, is_dev: bool) {
    // show "dev" keyword if installing dev dependancies
    let dev = if is_dev { "dev " } else { "" };

    // iterate through dependancies and install
    dependancies.iter().for_each(|dependancy| {
        let mut sp = Spinner::new(
            Spinners::Dots12,
            format!("Installing {}dependancy {}", dev, dependancy).into(),
        );
        let install_command = vec!["npm", "install", dependancy];
        dynamic_exec(install_command, &dir);
        sp.stop_and_persist("✅", format!("{} installed", dependancy));
    });
}

/// tailwindcss configuration procedure
fn init_tailwind(args: &Args, project_dir: &std::path::PathBuf) {
    // define tailwind initialisation command
    let tailwind_init_command = vec!["npx", "tailwindcss", "init", "-p"];

    // defile tailwind CSS content
    let tailwind_stylesheet = "@tailwind base;\n@tailwind components;\n@tailwind utilities;";

    // start loading
    let mut sp = Spinner::new(Spinners::Line, format!("Configuring tailwindcss").into());

    // execute tailwind initialisation
    dynamic_exec(tailwind_init_command, &project_dir);

    // define source file locations for tailwind config
    let mut content_path = String::new();
    match &args.template.as_str() {
        &"vite" => {
            content_path = r#"
            "./src/**/*.{js,jsx,ts,tsx}",
            "#
            .to_string();
        }
        &"next" => {
            content_path = r#"
            "./pages/**/*.{js,ts,jsx,tsx}",
            "./components/**/*.{js,ts,jsx,tsx}",
            "#
            .to_string();
        }
        _ => {
            println!("No template selected");
        }
    }

    // format tailwind content config
    let content_config = format!("{}{}{}", "content: [", content_path, "]");

    // locate valid tailwind content file
    let tailwind_config_path = if project_dir.join("tailwind.config.js").exists() {
        project_dir.join("tailwind.config.js")
    } else if project_dir.join("tailwind.config.cjs").exists() {
        project_dir.join("tailwind.config.cjs")
    } else {
        return;
    };

    // read default tailwind config
    let mut tailwind_config = fs::read_to_string(&tailwind_config_path).unwrap();

    // replace default content config with new content config
    tailwind_config = tailwind_config.replace("content: []", &content_config);

    // write new tailwind config
    fs::write(tailwind_config_path, tailwind_config).expect("Unable to write file");

    // locate valid stylesheet
    let global_stylesheet_path = if project_dir.join("src/index.css").exists() {
        project_dir.join("src/index.css")
    } else if project_dir.join("styles/globals.css").exists() {
        project_dir.join("styles/globals.css")
    } else {
        return;
    };

    // write tailwind variables to valid stylesheet
    fs::write(global_stylesheet_path, tailwind_stylesheet).expect("Unable to write file");
    // stop loading
    sp.stop_and_persist("⚙️ ", "tailwindcss configured".to_string())
}

/// optionally run development server
fn start_server(dir: &std::path::PathBuf) {
    let start_command = vec!["npm", "run", "dev"];

    // ask if user wants to run server
    println!("Do you want to start the development server now? (y/n)");
    // read user input
    let mut server_start = String::new();
    std::io::stdin().read_line(&mut server_start).unwrap();

    // if response is "yes", spawn command with IO
    if server_start.trim().to_uppercase() == "Y" {
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg("/C")
                .args(start_command)
                .current_dir(&dir)
                .stdout(Stdio::inherit())
                .stdin(Stdio::null())
                .stderr(Stdio::inherit())
                .spawn()
                .unwrap();
        } else {
            Command::new(start_command[0])
                .args(&start_command[1..])
                .current_dir(&dir)
                .stdout(Stdio::inherit())
                .stdin(Stdio::null())
                .stderr(Stdio::inherit())
                .spawn()
                .unwrap();
        }
    }
}

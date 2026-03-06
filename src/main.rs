use std::fs;
use std::str::FromStr;

enum App {
    Android,
    iOS,
}

impl App {
    fn get_format(&self) -> Option<&str> {
        match &self {
            App::Android => Some("android"),
            _ => None,
        }
    }

    fn get_file_extension(&self) -> &str {
        match &self {
            App::Android => "xml",
            App::iOS => "strings",
        }
    }

    fn get_output_filename(&self) -> &str {
        match &self {
            App::Android => "strings.xml",
            App::iOS => "Localizable.strings",
        }
    }

    fn get_charset(&self) -> Option<&str> {
        match &self {
            App::Android => None,
            App::iOS => Some("utf16"),
        }
    }
}

enum Lang {
    France,
    Spain,
    Brasilian,
    German,
}

impl Lang {
    fn get_code(&self) -> &str {
        match &self {
            Lang::France => "fr-FR",
            Lang::German => "de-DE",
            Lang::Brasilian => "pt-BR",
            Lang::Spain => "es-ES",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LangError;

#[derive(Debug, PartialEq, Eq)]
struct AppError;

impl FromStr for Lang {
    type Err = LangError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FR" => Ok(Lang::France),
            "ES" => Ok(Lang::Spain),
            "DE" => Ok(Lang::German),
            "BR" => Ok(Lang::Brasilian),
            _ => Err(LangError),
        }
    }
}

impl FromStr for App {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "android" => Ok(App::Android),
            "ios" => Ok(App::iOS),
            _ => Err(AppError),
        }
    }
}

fn parse_arg(args: &[String], flag: &str) -> Option<String> {
    args.windows(2).find(|w| w[0] == flag).map(|w| w[1].clone())
}

fn print_help() {
    println!(
        r#"LocoDownloader - Download localization files from Localise.biz

USAGE:
    loco [--app <app>] [--lang <lang>] [--output-dir <dir>] [--help]

ENVIRONMENT VARIABLES:
    LOCO_TOKEN        (required) Your Localise.biz API token.
                      Set it with: export LOCO_TOKEN=your_token_here

ARGUMENTS:
    --app <app>       Target app platform. Default: iOS
                      Values: iOS, Android

    --lang <lang>     Language to download. Default: FR
                      Values: FR (French), ES (Spanish), DE (German), BR (Brazilian Portuguese)

    --output-dir <dir>  Directory where the output file will be saved.
                        Default: current directory

OUTPUT FILES:
    iOS     → Localizable.strings
    Android → strings.xml

EXAMPLES:
    LOCO_TOKEN=abc123 ./loco --app iOS --lang FR
    LOCO_TOKEN=abc123 ./loco --app Android --lang ES --output-dir ./res"#
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return;
    }

    // Inputs
    let token = std::env::var("LOCO_TOKEN").expect("LOCO_TOKEN environment variable is not set");
    let lang_str = parse_arg(&args, "--lang").unwrap_or_else(|| "FR".to_string());
    let app_str = parse_arg(&args, "--app").unwrap_or_else(|| "iOS".to_string());
    let output_dir = parse_arg(&args, "--output-dir");

    let lang = Lang::from_str(&lang_str).expect("Unknown lang code. Use: FR, ES, DE, BR");
    let app = App::from_str(&app_str).expect("Unknown app code. Use: Android, iOS");

    let format_param = app
        .get_format()
        .map(|f| format!("&format={}", f))
        .unwrap_or_default();

    let charset_param = app
        .get_charset()
        .map(|c| format!("&charset={}", c))
        .unwrap_or_default();

    let url = format!(
        "https://localise.biz/api/export/locale/{}.{}?filter=!deprecated{}{}",
        lang.get_code(),
        app.get_file_extension(),
        format_param,
        charset_param
    );
    println!("Download from {}", url);

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("Authorization", format!("Loco {}", token))
        .send()
        .expect("HTTP request failed");

    let body = response.bytes().expect("Failed to read response body");
    let output_path = match &output_dir {
        Some(dir) => format!("{}/{}", dir, app.get_output_filename()),
        None => format!("{}", app.get_output_filename()),
    };
    fs::write(&output_path, body).expect(format!("{}", app.get_output_filename()).as_str());

    println!("Saved {}", output_path);
}

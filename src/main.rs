use clap::Parser;

use core::fmt;
use std::{
    any,
    error::Error,
    fs::File,
    io::{self, Write},
    path::PathBuf,
};

const DEFAULT_LINENO: usize = 1;
const DEFAULT_PATH: &str = "<stdin>";
const DEFAULT_ERRNUM: usize = 69;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    output: Option<PathBuf>,
}

impl Args {
    fn get_output(&self) -> Result<Box<dyn io::Write>, io::Error> {
        match self.output {
            Some(ref path) => File::options()
                .write(true)
                .create(true)
                .open(path)
                .map(|f| Box::new(f) as Box<dyn io::Write>),
            None => Ok(Box::new(io::stdout())),
        }
    }
}

#[derive(Debug)]
enum RoostError {
    ValueError { details: String },
}

impl fmt::Display for RoostError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            RoostError::ValueError { details } => write!(f, "{}", details),
        }
    }
}

impl Error for RoostError {
    fn description(&self) -> &str {
        match &self {
            RoostError::ValueError { details } => &details,
        }
    }
}

struct ErrorData {
    summary: String,
    line: String,
    message: String,
    spos: usize,
    epos: usize,
    lineno: usize,
    path: String,
    errnum: usize,
}

impl ErrorData {
    fn get_errid(&self) -> String {
        format!("E{:0fill$}", self.errnum, fill = 4)
    }

    fn print(&self, output: &mut Box<dyn io::Write>) {
        let lineno_len = self.lineno.to_string().len();
        let empty_line = color(format!("{}| ", " ".repeat(lineno_len + 1)), 4);

        let mut string = bold(color(format!("error[{}]", self.get_errid()), 1));
        string.extend(bold(format!(": {}\n", self.summary)).chars());
        string.extend(
            format!(
                "{}{}{}:{}:{}\n",
                " ".repeat(lineno_len),
                color("--> ".to_owned(), 4),
                self.path,
                self.lineno,
                self.spos + 1,
            )
            .chars(),
        );
        string.extend(empty_line.chars());
        string.extend("\n".chars());
        string.extend(color(format!("{} | ", self.lineno), 4).chars());
        string.extend(self.line[0..self.spos].chars());
        string.extend(bold(color(self.line[self.spos..self.epos].to_string(), 1)).chars());
        string.extend(self.line[self.epos..].chars());
        string.extend("\n".chars());
        string.extend(empty_line.chars());
        string.extend(" ".repeat(self.spos).chars());
        string.extend(bold(color("^".repeat(self.epos - self.spos), 1)).chars());
        string.extend(format!(" {}", bold(color(self.message.clone(), 1))).chars());
        string.extend("\n".chars());
        string.extend(empty_line.chars());

        writeln!(output, "{}", string).expect("unexpected error happened");
    }
}

fn string(string: &str) -> Result<String, RoostError> {
    Ok(string.to_string())
}

fn bold(string: String) -> String {
    format!("\x1b[1m{}\x1b[0m", string)
}

fn color(string: String, code: u8) -> String {
    format!("\x1b[3{}m{}\x1b[39m", code, string)
}

fn make_prompt(name: String, default: Option<String>) -> String {
    let mut prompt = name;

    if let Some(default_value) = default {
        prompt.extend(color(format!(" (default={})", default_value), 4).chars());
    }

    return bold(format!("{}: ", prompt));
}

fn field<T, F>(name: &str, field_type: &F, default: Option<T>) -> T
where
    T: fmt::Display + 'static + Clone,
    F: Fn(&str) -> Result<T, RoostError>,
{
    loop {
        let prompt = make_prompt(name.to_owned(), default.as_ref().map(|t| t.to_string()));

        print!("{}", prompt);
        io::stdout().flush().expect("could not flush stdout");

        let mut result = String::new();
        io::stdin().read_line(&mut result).expect("failed input");

        result = result.trim_end().to_string();

        if result.is_empty() {
            if let Some(default) = &default {
                return default.clone();
            }
            eprintln!(
                "{}",
                bold(color(format!("ERR: field '{}' cannot be empty", name), 1))
            );
        }

        match field_type(&result) {
            Ok(value) => return value,
            Err(_) => {
                eprintln!(
                    "{}",
                    bold(color(
                        format!("ERR: '{}' is not a valid {}", result, any::type_name::<F>()),
                        3
                    ))
                );
            }
        }
    }
}

fn int_factory(min_value: usize, max_value: usize) -> impl Fn(&str) -> Result<usize, RoostError> {
    move |raw_value: &str| {
        let value = raw_value
            .parse::<usize>()
            .map_err(|_| RoostError::ValueError {
                details: "invalid value".to_string(),
            })?;

        if value < min_value {
            return Err(RoostError::ValueError {
                details: "value is too smol".to_string(),
            });
        }
        if value > max_value {
            return Err(RoostError::ValueError {
                details: "value is too big".to_string(),
            });
        }

        Ok(value)
    }
}

fn print_line_helper(line: String) {
    let last_char_no_len = line.len().to_string().len() + 1;
    let helper_len = last_char_no_len * line.len();

    println!("{}", "─".repeat(helper_len));

    for (i, _) in line.chars().enumerate() {
        print!("{:^width$}", i, width = last_char_no_len);
    }
    println!();

    for c in line.chars() {
        print!("{c:^width$}", c = c, width = last_char_no_len);
    }
    println!();

    println!("{}", "─".repeat(helper_len));
}

fn main() {
    let mut output = Args::parse()
        .get_output()
        .unwrap_or_else(|_| panic!("An unknown error occurred"));

    let summary = field("summary", &mut string, None);
    let line: String = field("line", &mut string, None);

    print_line_helper(line.clone());

    let spos = field("error start position", &int_factory(0, line.len()), Some(0));
    let epos = field(
        "error end position",
        &int_factory(spos + 1, line.len() - 1),
        Some(line.len() - 1),
    ) + 1;
    let message = field("message", &mut string, None);
    let lineno = field(
        "line number",
        &int_factory(usize::MIN, usize::MAX),
        Some(DEFAULT_LINENO),
    );
    let path = field("path", &mut string, Some(DEFAULT_PATH.to_owned()));
    let errnum = field(
        "error number",
        &int_factory(usize::MIN, usize::MAX),
        Some(DEFAULT_ERRNUM),
    );

    println!();

    let err = ErrorData {
        summary,
        line,
        message,
        spos,
        epos,
        lineno,
        path,
        errnum,
    };

    err.print(&mut output);
}

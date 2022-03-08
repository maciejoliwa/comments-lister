use std::env;
use std::path;
use std::fs; 

#[derive(Debug)]
struct Line {
    contents: String,
    number: u32
}

#[derive(Debug)]
struct CheckedFile {
    path: path::PathBuf,
    comment_lines: Vec<Line>
}

impl CheckedFile {

    fn check_file(entry: &fs::DirEntry, comment_type: &CommentType) -> CheckedFile {
        let comment_str_type = comment_type_to_single_line_comment(comment_type);
        let contents = fs::read_to_string(entry.path()).expect("Couldn't read file");
        let mut lines: Vec<Line> = vec![];

        for (number, line) in contents.split("\n").enumerate() {
            if line.trim().starts_with(comment_str_type) {
                // We add one to the number because the iteration starts at 0, and code lines start at 1
                lines.push(Line{ number: (number as u32) + 1, contents: line.trim().to_string() });
            }
        }

        return CheckedFile{ comment_lines: lines, path: entry.path() };
    }

    fn describe(self: &Self) {
        let amount_of_comment_lines = self.comment_lines.len();
        let filename = self.path.file_name();

        println!("There are {} comments in \x1b[93m {:?}\x1b[0m", amount_of_comment_lines, filename.unwrap());
        
        for line in &self.comment_lines {
            println!("{} - at line \x1b[33m{}\x1b[0m", line.contents, line.number);
        }
    }

}

#[derive(Debug)]
enum CommentType {
    C,
    LISP,
    PYTHON,
    DEFAULT
}

fn str_to_comment_type(s: &str) -> CommentType {
    match s.to_lowercase().as_str() {
        "c" => CommentType::C,
        "js" => CommentType::C,
        "python" => CommentType::PYTHON,
        "py" => CommentType::PYTHON,
        "clj" => CommentType::LISP,
        "lsp" => CommentType::LISP,
        "lisp" => CommentType::LISP,
        _ => CommentType::DEFAULT
    }
}

fn comment_type_to_single_line_comment(comment_type: &CommentType) -> &str {
    match comment_type {
        CommentType::C => "//",
        CommentType::DEFAULT => "//",
        CommentType::LISP => ";",
        CommentType::PYTHON => "#"
    }
}

fn collect_cli_args() -> Option<Vec<String>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        return Some(args[1..].to_vec());
    }

    return None;
}

fn main() {
    let arguments: Option<Vec<String>> = collect_cli_args();
    let mut files: Vec<CheckedFile> = vec![];

    match arguments {
        Some(args) => {
            let project_path: &path::Path = path::Path::new(args[0].as_str());
            let comment_type: CommentType = str_to_comment_type(args.last().unwrap());

            if !project_path.exists() {
                println!("Path does not exist.");
                return;
            }

            if project_path.is_dir() {
                for path in project_path.read_dir().expect("Failed reading the directory.") {
                    match path {
                        Ok(entry) => {
                            files.push(CheckedFile::check_file(&entry, &comment_type));
                        },
                        Err(_) => print!("error")
                    }
                }
            }

            println!("\n=========== RESULTS ===========\n");
            for file in files {
                file.describe();
                println!("\n");
            }
        },
        None => {
            println!("Not enough arguments passed.")
        }
    }
}

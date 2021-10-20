use std::fs;
use std::process::{Command, ExitStatus};

use crate::logging::Log;

pub struct Compiler {
    exec: String,
    global: String,
    lib: String,
    options: Vec<String>,
}

pub struct CompileOutput {
    pub status: ExitStatus,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub file: String,
}

impl Default for Compiler {
    // TODO: add an option allowing the user to specify a path to exec, global and lib.
    fn default() -> Self {
        Compiler {
            exec: String::from("./node_modules/assemblyscript/bin/asc"),
            global: String::from("./node_modules/@graphprotocol/graph-ts/global/global.ts"),
            lib: String::from("./node_modules/"),
            options: vec![String::from("--explicitStart")],
        }
    }
}

#[allow(dead_code)]
impl Compiler {
    pub fn export_table(mut self) -> Self {
        self.options.push("--exportTable".to_string());
        self
    }

    pub fn optimize(mut self) -> Self {
        self.options.push("--optimize".to_string());
        self
    }

    pub fn debug(mut self) -> Self {
        self.options.push("--debug".to_string());
        self
    }

    pub fn export_runtime(mut self) -> Self {
        self.options.push("--exportRuntime".to_string());
        self
    }

    pub fn runtime(mut self, s: &str) -> Self {
        self.options.push("--runtime".to_string());
        self.options.push(s.to_string());
        self
    }

    pub fn enable(mut self, s: &str) -> Self {
        self.options.push("--enable".to_string());
        self.options.push(s.to_string());
        self
    }

    fn get_paths_for(datasource: &str) -> (Vec<String>, String) {
        let entry = fs::read_dir("./tests/")
            .unwrap_or_else(|err| {
                panic!(
                    "{}",
                    Log::Critical(format!(
                        "Something went wrong while trying to read `tests/`: {}",
                        err,
                    )),
                );
            })
            .find_map(|entry| {
                let entry = entry.unwrap();
                if entry
                    .file_name()
                    .to_str()
                    .unwrap()
                    .to_ascii_lowercase()
                    .starts_with(datasource)
                {
                    Some(entry)
                } else {
                    None
                }
            })
            .unwrap_or_else(|| {
                panic!(
                    "{}",
                    Log::Critical(format!("No tests were found for '{}'.", datasource)),
                );
            });

        let in_files = if entry.file_type().unwrap().is_dir() {
            entry
                .path()
                .read_dir()
                .unwrap()
                .map(|file| file.unwrap().path().to_str().unwrap().to_string())
                .filter(|path| path.ends_with(".test.ts"))
                .collect()
        } else {
            vec![entry.path().to_str().unwrap().to_string()]
        };

        fs::create_dir_all("./tests/.bin/").unwrap_or_else(|err| {
            panic!(
                "{}",
                Log::Critical(format!(
                    "Something went wrong when trying to crate `./tests/.bin/`: {}",
                    err,
                )),
            );
        });

        return (in_files, format!("./tests/.bin/{}.wasm", datasource));
    }

    pub fn compile(&self, datasource: &str) -> CompileOutput {
        let (in_files, out_file) = Compiler::get_paths_for(datasource);
        let output = Command::new(&self.exec)
            .args(in_files)
            .arg(&self.global)
            .arg("--lib")
            .arg(&self.lib)
            .args(&self.options)
            .arg("--outFile")
            .arg(out_file.clone())
            .output()
            .expect("Internal error during compilation.");

        CompileOutput {
            status: output.status,
            stdout: output.stdout,
            stderr: output.stderr,
            file: out_file,
        }
    }
}

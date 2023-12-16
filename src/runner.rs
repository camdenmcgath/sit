use std::process::{Command, Output};

pub struct PlatformRunner {
    executable: &'static str,
    optional_args: &'static [&'static str],
}
impl PlatformRunner {
    pub fn for_platform() -> PlatformRunner {
        let operating_system = std::env::consts::OS;
        match operating_system {
            "windows" => PlatformRunner {
                executable: "powershell",
                optional_args: &["-Command"],
            },
            "macos" => PlatformRunner {
                executable: "zsh",
                optional_args: &["-c"],
            },
            "linux" => PlatformRunner {
                executable: "sh",
                optional_args: &["-c"],
            },
            _ => panic!("Running on unsupported platform {}!", operating_system),
        }
    }

    // This AsRef<str> allows this function to accept
    // &String, String, or &str
    pub fn execute<T: AsRef<str>>(&self, cmd: T) -> std::io::Result<Output> {
        let mut runner = Command::new(self.executable);
        for o in self.optional_args {
            runner.arg(o);
        }
        runner.arg(cmd.as_ref());
        runner.output()
    }
}

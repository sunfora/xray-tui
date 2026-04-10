use std::env;
use std::path::Path;
use std::process::Command;

enum SystemConfig {
  LinuxSystemd,
  LinuxGeneric,
  Unsupported(String),
}

fn determine_linux_config() -> SystemConfig {
  if Path::new("/run/systemd/system").exists() {
    SystemConfig::LinuxSystemd
  } else {
    SystemConfig::LinuxGeneric
  }
}

fn determine_os_config() -> SystemConfig {
  let os_t = env::consts::OS;
  match os_t {
    "linux" => determine_linux_config(),
    _ => SystemConfig::Unsupported ( os_t.to_string() )
  }
}

fn parse_exec_start(service: &str) -> Option<Vec<String>> {
  let output = Command::new("systemctl")
  .args(["cat", service])
  .output()
  .ok()?;

  let command = String::from_utf8_lossy(&output.stdout);
  
  let mut entries: Vec<String> = Vec::new();
  let mut result: Vec<&str> = Vec::new();
  let mut collecting = false;
  let mut ignored_line;

  for line in command.lines() {
    ignored_line = true;
    let ends_with_slash = line.ends_with("\\");
    //
    // gather
    //
    if !collecting {
      if let Some(hadexec) = line.trim_start().strip_prefix("ExecStart") {
        if let Some(cmd) = hadexec.trim_start().strip_prefix("=") {
          ignored_line = false;

          let start = cmd.trim_start();
          result.push(start);
        }
      }
    } else {
      ignored_line = false;
      result.push(line);
    }
    //
    // decide
    //
    if !ignored_line {
      if !ends_with_slash {
        // 
        // wipe out and collect
        //
        let text = result.join("\n");
        result.clear();

        if text.trim().is_empty() {
          // behaviour of empty ExecStart
          // aka ExecStart=
          entries.clear();
        } else {
          entries.push(text);
        }
      }
      collecting = ends_with_slash;
    }
  }
  
  if entries.len() > 0 {
    Some(entries)
  } else {
    None
  }
}

fn main() {
  let config = determine_os_config();
  
  match config {
    SystemConfig::LinuxSystemd => {
      println!("linux::systemd found");

      if let Some(commands) = parse_exec_start("cowsay-test") {
        for (i, command) in commands.iter().enumerate() {
          println!("found {} `{}`", i, command);
        }
      } else {
        println!("xray not found");
      }
    },
    SystemConfig::LinuxGeneric => {
      eprintln!(concat!(
          "linux::generic found", "\n",
          "not supported yet"
      ));
      std::process::exit(1);
    },
    SystemConfig::Unsupported(os) => {
      eprintln!("your operating system {} is not supported yet", os);
      std::process::exit(1);
    }
  }
}

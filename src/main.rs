use std::env;
use std::path::Path;
use std::process::Command;

use std::fs::File;

use std::error::Error;
use serde::{Deserialize, Serialize};

use display_json::DisplayAsJson;

//
// https://github.com/XTLS/Xray-core/blob/e5a9fb752e0dcc127dd1740316c853571c16052f/infra/conf/xray.go#L346-L365
//
#[allow(non_snake_case)]
#[derive(Debug, Clone, Deserialize, Serialize, DisplayAsJson)]
pub struct XrayConfig {
  #[serde(rename = "log", skip_serializing_if = "Option::is_none")]
  pub LogConfig: Option<serde_json::Value>,
  #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
	pub RouterConfig: Option<serde_json::Value>,
  #[serde(rename = "dns", skip_serializing_if = "Option::is_none")]
	pub DNSConfig: Option<serde_json::Value>,
  #[serde(rename = "inbounds")]
	pub InboundConfigs: Vec<serde_json::Value>,
  #[serde(rename = "outbounds")]
	pub OutboundConfigs: Vec<OutboundDetourConfig>,
  #[serde(rename = "policy", skip_serializing_if = "Option::is_none")]
	pub Policy: Option<serde_json::Value>,
  #[serde(rename = "api", skip_serializing_if = "Option::is_none")]
	pub API: Option<serde_json::Value>,
  #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
	pub Metrics: Option<serde_json::Value>,
  #[serde(rename = "stats", skip_serializing_if = "Option::is_none")]
	pub Stats: Option<serde_json::Value>,
  #[serde(rename = "reverse", skip_serializing_if = "Option::is_none")]
	pub Reverse: Option<serde_json::Value>,
  #[serde(rename = "fakeDns", skip_serializing_if = "Option::is_none")]
	pub FakeDNS: Option<serde_json::Value>,
  #[serde(rename = "observatory", skip_serializing_if = "Option::is_none")]
	pub Observatory: Option<serde_json::Value>,
  #[serde(rename = "burstObservatory", skip_serializing_if = "Option::is_none")]
	pub BurstObservatory: Option<serde_json::Value>,
  #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
	pub Version: Option<serde_json::Value>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Deserialize, Serialize, DisplayAsJson)]
pub struct OutboundDetourConfig {
  #[serde(rename = "protocol")]
	pub Protocol: String,
  #[serde(rename = "sendThrough", skip_serializing_if = "Option::is_none")]
	pub SendThrough: Option<String>,
  #[serde(rename = "tag", default)]
	pub Tag: String,
  #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
  pub Settings: Option<serde_json::Value>,
  #[serde(rename = "streamSettings", skip_serializing_if = "Option::is_none")]
  pub StreamSetting: Option<StreamConfig>,
  #[serde(rename = "proxySettings", skip_serializing_if = "Option::is_none")]
  pub ProxySettings: Option<serde_json::Value>,
  #[serde(rename = "mux", skip_serializing_if = "Option::is_none")]
  pub MuxSettings: Option<serde_json::Value>,
  #[serde(rename = "targetStrategy", skip_serializing_if = "Option::is_none")]
  pub TargetStrategy: Option<String>
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Deserialize, Serialize, DisplayAsJson)]
pub struct StreamConfig {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub Address: Option<serde_json::Value>,
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub Port: Option<u16>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub Network: Option<TransportProtocol>,
    #[serde(rename = "security", skip_serializing_if = "Option::is_none")]
    pub Security: Option<String>,
    #[serde(rename = "finalmask", skip_serializing_if = "Option::is_none")]
    pub FinalMask: Option<serde_json::Value>,
    #[serde(rename = "tlsSettings", skip_serializing_if = "Option::is_none")]
    pub TLSSettings: Option<serde_json::Value>,
    #[serde(rename = "realitySettings", skip_serializing_if = "Option::is_none")]
    pub REALITYSettings: Option<REALITYConfig>,

    // NOTE(ivan): commented out since it is actually an alias for tcpSettings
    //
    // #[serde(rename = "rawSettings", skip_serializing_if = "Option::is_none")]
    // pub RAWSettings: Option<serde_json::Value>,
    
    #[serde(
      rename = "tcpSettings", alias="rawSettings", 
      skip_serializing_if = "Option::is_none"
    )]
    pub TCPSettings: Option<serde_json::Value>,
    
    // NOTE(ivan): commented out since it is actually an alias for splithttpSettings
    //
    // #[serde(rename = "xhttpSettings", skip_serializing_if = "Option::is_none")]
    // pub XHTTPSettings: Option<serde_json::Value>,

    #[serde(
      rename = "splithttpSettings", alias="xhttpSettings",
      skip_serializing_if = "Option::is_none"
    )]
    pub SplitHTTPSettings: Option<serde_json::Value>,
    
    #[serde(rename = "kcpSettings", skip_serializing_if = "Option::is_none")]
    pub KCPSettings: Option<serde_json::Value>,
    #[serde(rename = "grpcSettings", skip_serializing_if = "Option::is_none")]
    pub GRPCSettings: Option<serde_json::Value>,
    #[serde(rename = "wsSettings", skip_serializing_if = "Option::is_none")]
    pub WSSettings: Option<serde_json::Value>,
    #[serde(rename = "httpupgradeSettings", skip_serializing_if = "Option::is_none")]
    pub HTTPUPGRADESettings: Option<serde_json::Value>,
    #[serde(rename = "hysteriaSettings", skip_serializing_if = "Option::is_none")]
    pub HysteriaSettings: Option<serde_json::Value>,
    #[serde(rename = "sockopt", skip_serializing_if = "Option::is_none")]
    pub SocketSettings: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, DisplayAsJson)]
#[serde(rename_all = "lowercase")]
pub enum TransportProtocol {
    #[serde(alias = "raw")]
    Tcp,
    #[serde(alias = "splithttp")]
    Xhttp,
    #[serde(alias = "mkcp")]
    Kcp,
    Hysteria,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct REALITYConfig {
    #[serde(rename = "masterKeyLog", skip_serializing_if = "Option::is_none")]
    pub MasterKeyLog: Option<String>,
    #[serde(rename = "show", skip_serializing_if = "Option::is_none")]
    pub Show: Option<bool>,
    #[serde(
      rename = "target", alias="dest", 
      skip_serializing_if = "Option::is_none"
    )]
    pub Target: Option<serde_json::Value>,
    
    // NOTE(ivan): commented out since it is actually an alias for target
    //
    // #[serde(rename = "dest", skip_serializing_if = "Option::is_none")]
    // pub Dest: Option<serde_json::Value>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub Type: Option<String>,
    #[serde(rename = "xver", skip_serializing_if = "Option::is_none")]
    pub Xver: Option<u64>,

    // NOTE(ivan): MUST FOR SERVER
    #[serde(rename = "serverNames", default, skip_serializing_if = "Vec::is_empty")]
    pub ServerNames: Vec<String>,
    // NOTE(ivan): MUST FOR SERVER
    #[serde(rename = "privateKey", skip_serializing_if = "Option::is_none")]
    pub PrivateKey: Option<String>,

    #[serde(rename = "minClientVer", skip_serializing_if = "Option::is_none")]
    pub MinClientVer: Option<String>,
    #[serde(rename = "maxClientVer", skip_serializing_if = "Option::is_none")]
    pub MaxClientVer: Option<String>,
    #[serde(rename = "maxTimeDiff", skip_serializing_if = "Option::is_none")]
    pub MaxTimeDiff: Option<u64>,

    // NOTE(ivan): MUST FOR SERVER
    #[serde(rename = "shortIds", default, skip_serializing_if = "Vec::is_empty")]
    pub ShortIds: Vec<String>,

    #[serde(rename = "mldsa65Seed", skip_serializing_if = "Option::is_none")]
    pub Mldsa65Seed: Option<String>,
    #[serde(rename = "limitFallbackUpload", skip_serializing_if = "Option::is_none")]
    pub LimitFallbackUpload: Option<serde_json::Value>,
    #[serde(rename = "limitFallbackDownload", skip_serializing_if = "Option::is_none")]
    pub LimitFallbackDownload: Option<serde_json::Value>,
    #[serde(rename = "fingerprint", skip_serializing_if = "Option::is_none")]

    // NOTE(ivan): MUST FOR CLIENT
    pub Fingerprint: Option<String>,

    // NOTE(ivan): MUST FOR CLIENT
    #[serde(rename = "serverName", skip_serializing_if = "Option::is_none")]
    pub ServerName: Option<String>,

    // NOTE(ivan): commented out since it is actually an alias for publicKey
    //
    // #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    // pub Password: Option<String>,
    
    // NOTE(ivan): MUST FOR CLIENT
    #[serde(
      rename = "publicKey", alias = "password", 
      skip_serializing_if = "Option::is_none"
    )]
    pub PublicKey: Option<String>,

    #[serde(rename = "shortId", skip_serializing_if = "Option::is_none")]
    pub ShortId: Option<String>,
    #[serde(rename = "mldsa65Verify", skip_serializing_if = "Option::is_none")]
    pub Mldsa65Verify: Option<String>,
    #[serde(rename = "spiderX", skip_serializing_if = "Option::is_none")]
    pub SpiderX: Option<String>,
}

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

fn find_config_in_command(command: &str) -> Option<String> {
  let mut return_next = false;
  if let Some(args) = shlex::split(command) {
    for arg in args {
      if return_next {
        return Some(arg)
      }
      if arg == "-config" || arg == "-c" {
        return_next = true;
      }
    }
  }
  None
}

fn read_config(path: &str) -> Result<XrayConfig, Box<dyn Error>> {
  let x = Path::new(path);
  if !x.exists() {
    return Err("file does not exist".into());
  }
  let file = File::open(&x)?;
  let json: XrayConfig = serde_json::from_reader(file)?;

  Ok(json)
}

fn main() {
  let config = determine_os_config();
  
  match config {
    SystemConfig::LinuxSystemd => {
      println!("linux::systemd found");

      if let Some(commands) = parse_exec_start("xray") {
        for (i, command) in commands.iter().enumerate() {
          println!("found {} `{}`", i, command);
          if let Some(path) = find_config_in_command(command) {
            println!("found path: {}", path);
            match read_config(path.as_str()) {
              Ok(json) => println!("{}", json),
              Err(why) => println!("{}", why.to_string())
            }
          }
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

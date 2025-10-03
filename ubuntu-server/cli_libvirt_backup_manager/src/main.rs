#![allow(dead_code)]
#![allow(unused_variables)]
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use clap::Parser;
use chrono::{Local, DateTime};

const QCOW_DIR: &str = "/var/lib/libvirt/images";
const XML_DIR: &str = "/etc/libvirt/qemu";

const QCOW_BACKUP_DIR: &str = "/home/matthias/vmbackups/images";
const XML_BACKUP_DIR: &str = "/home/matthias/vmbackups/xml";

const LOG_DIR: &str = "/home/matthias/vm_backups.log";
const RETENTION_DAYS: u16 = 365;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    restore: bool,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    backup: bool,
    #[arg(long)]
    name: Option<String> // is either backup name or vm name, depending on what's set
}

enum VirtualMachineState {
    Running,
    Idle,
    Paused,
    Shutdown,
    ShutOff,
    Crashed,
    Dying,
    PmSuspended,
    Undefined
}

trait VirtualMachine {
    fn backup_xml(self: &Self) -> PathBuf;
    fn backup_qcow(self: &Self) -> PathBuf;
    fn stop(self: &Self) -> bool;
    fn start(self: &Self) -> bool;
    fn undefine(self: &Self) -> bool;
}

struct VM {
    name: String,
    state: VirtualMachineState,
}

impl From<Backup> for VM {
    fn from(backup: Backup) -> Self {
        //let name = parse_xml(backup.xml).get("name");
        let name = "TODO".to_string();
        VM::new(
            name,
            VirtualMachineState::Undefined
        )
    }
}

impl From<String> for VM {
    fn from(string: String) -> Self {
        Self {
            name: string.clone(),
            state: VirtualMachineState::Undefined
        }
    }
}

impl VM {
    pub fn new(name: String, state: VirtualMachineState) -> Self {
        Self {
            name: name.clone(),
            state
        }
    }
}

struct Backup {
    qcow: PathBuf,
    xml: PathBuf
}


impl VM {
    fn dump_xml() -> PathBuf {
        PathBuf::new()
    }
    fn backup_qcow() -> PathBuf {
        PathBuf::new()
    }
}

struct BackupXMLParser {
    xml: PathBuf
}

impl BackupXMLParser {
    pub fn new(path_buf: PathBuf) -> Self {
        BackupXMLParser {
            xml: path_buf
        }
    }

    pub fn get_name(self: &Self) -> Option<String> {
        self.get_node_text("name")
    }

    pub fn get_uuid(self: &Self) -> Option<String> {
        self.get_node_text("uuid")
    }
    fn read_xml(self: &Self) -> String {
        let mut file = File::open(&self.xml).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        contents
    }

    fn get_node_text(&self, node: &str) -> Option<String> {
        let xml = self.read_xml();
        let doc = roxmltree::Document::parse(&xml).ok()?;
        let elem = doc.descendants().find(|n| n.has_tag_name(node));
        elem.and_then(|n| n.text()).map(|s| s.trim().to_string())
    }
}

fn main() {
    let mut cli = Cli::parse();

    let example_xml = PathBuf::from("/Users/matthias/Projects/monorepo/ubuntu-server/cli_libvirt_backup_manager/assets/example.xml");
    let xml_parser = BackupXMLParser::new(example_xml);

    println!("Name: {}, UUID: {}", xml_parser.get_name().unwrap(), xml_parser.get_uuid().unwrap());
}

fn run_backup(domain_name: String) {
    let now: DateTime<Local> = Local::now();
    let now = now.format("%Y_%m_%d").to_string();

    println!("Now: {}", now);
}

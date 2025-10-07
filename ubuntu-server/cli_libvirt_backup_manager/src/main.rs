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
    fn restore(self: &Self, backup: &Backup) -> bool;
    fn backup(self: &Self) -> bool;
}

struct VM {
    name: String,
    state: VirtualMachineState,
}
impl From<Backup> for VM {
    fn from(backup: Backup) -> Self {
        let xml_parser = BackupXMLParser::new(backup.xml);
        let name = xml_parser.get_name();
        VM::new(
            xml_parser.get_name().unwrap_or_default(),
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
    fn backup_xml() -> PathBuf {
        // todo!("virsh dumpxml <name>");

        PathBuf::new()
    }
    fn backup_qcow() -> PathBuf {
        // todo!("qemu_img convert -p -f qcow2 -O qcow2 <src> <backup>");
        PathBuf::new()
    }
    fn stop(self: &Self) -> bool {
        true
    }
    fn start(self: &Self) -> bool {
        true
    }
    fn undefine(self: &Self) -> bool {
        true
    }
}

impl VirtualMachine for VM {
    fn backup(self: &Self) -> bool {
        false
    }

    fn restore(self: &Self, backup: &Backup) -> bool {
        false
    }
}

struct Backup {
    qcow: PathBuf,
    xml: PathBuf
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
    let cli = Cli::parse();

    let example_xml = PathBuf::from("/cli_libvirt_backup_manager/example.xml");
    let xml_parser = BackupXMLParser::new(example_xml);

    println!("Name: {}, UUID: {}", xml_parser.get_name().unwrap(), xml_parser.get_uuid().unwrap());
}

/*

use std::ffi::CString;
use std::ptr;

fn main() {
    unsafe {
        // Connect to libvirt
        let conn_uri = CString::new("qemu:///system").unwrap();
        let conn = virConnectOpen(conn_uri.as_ptr());
        if conn.is_null() {
            panic!("Failed to connect to libvirt");
        }

        // Lookup the domain by name
        let dom_name = CString::new("haos").unwrap();
        let dom = virDomainLookupByName(conn, dom_name.as_ptr());
        if dom.is_null() {
            virConnectClose(conn);
            panic!("Domain not found");
        }

        let list = virDomainList(conn);
        println!("{:#?}", list);

        // Get XML with desired flags
        let xml_flags = VIR_DOMAIN_XML_INACTIVE; // For --inactive equivalent
        let xml_ptr = virDomainGetXMLDesc(dom, xml_flags);
        if xml_ptr.is_null() {
            virDomainFree(dom);
            virConnectClose(conn);
            panic!("Error fetching domain XML");
        }

        // Use the XML
        let xml = CString::from_raw(xml_ptr);
        println!("XML: {}", xml.to_str().unwrap());

        // Clean up
        virDomainFree(dom);
        virConnectClose(conn);
    }
}
 */
/*
fn run_backup(domain_name: String) {
    let now: DateTime<Local> = Local::now();
    let now = now.format("%Y_%m_%d").to_string();

    println!("Now: {}", now);
}
 */

#![allow(dead_code)]
#![allow(unused_variables)]
use std::fmt::Debug;
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
    //backups: Map<DateTime, Backup>
}

impl From<Backup> for VM {
    fn from(backup: Backup) -> Self {
        backup.vm
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
    vm: VirtualMachine,
    qcow: PathBuf,
    xml: PathBuf
}

impl Backup {
    pub fn into_vm(self: &self) -> VirtualMachine {
        VM::new(self.name, self.state)
    }
}

impl VM {
    fn dump_xml() -> PathBuf {}
    fn backup_qcow() -> PathBuf {}
}

fn main() {
    let mut cli = Cli::parse();
}

fn run_backup(domain_name: String) {
    let now: DateTime<Local> = Local::now();
    let now = now.format("%Y_%m_%d").to_string();

    println!("Now: {}", now);
}


/*
 * Ablauf des Skripts, sowie Zustände, in dem es sich befinden kann.
 *
 *
 */

/*
#!/bin/bash

# thank you to stack overflow for giving me the courage to wade through 100s of posts and hack together something that looks like it works.

# config for backups
BACKUP_DIR="/mediapool/vmbackups"
LOG_FILE="/var/log/vm_backups.log"
RETENTION_DAYS=56  # how long to keep backups

# Function to write messages to our log file`
log_message() {
    # Get the current timestamp and message
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$LOG_FILE"
}

# Function to find the actual disk path for a VM when the default path doesn't exist
# Uses virsh dumpxml to get the disk source path from the VM's XML configuration
find_vm_disk_path() {
    local vm_name=$1
    # Get the VM's XML configuration and extract the first disk source path
    # Using grep with -o to only output the matched portion
    # Using sed to extract just the path part from the source attribute
    local disk_path=$(virsh dumpxml "$vm_name" | grep -o "source file='[^']*'" | head -n1 | sed "s/source file='\(.*\)'/\1/")

    # Check if we found a path and if it exists
    if [ -n "$disk_path" ] && [ -f "$disk_path" ]; then
        echo "$disk_path"
        return 0
    else
        return 1
    fi
}

# main backup function
backup_vm() {
    local virtual_machine_name=$1  # The name of the virtual machine we're backing up
    local date_stamp=$(date +%Y%m%d)  # Today's date for the backup file name
    local source_file="/var/lib/libvirt/images/${virtual_machine_name}.qcow2"  # Where the virtual machine is

    # If the default path doesn't exist, try to find the actual disk path
    if [ ! -f "$source_file" ]; then
        log_message "Default disk path not found for ${virtual_machine_name}, searching XML configuration..."
        local found_path=$(find_vm_disk_path "$virtual_machine_name")

        # If we found a valid path, use it instead
        if [ -n "$found_path" ]; then
            log_message "Found alternate disk path: ${found_path}"
            source_file="$found_path"
        fi
    fi

    local backup_file="${BACKUP_DIR}/${virtual_machine_name}-${date_stamp}.qcow2"  # Where we're putting the backup of it
    local config_file="${BACKUP_DIR}/${virtual_machine_name}-${date_stamp}.xml"  # Where it saves the virtual machine config

    # Check if source file exists before attempting backup
    if [ ! -f "$source_file" ]; then
        log_message "ERROR: Source file $source_file does not exist for ${virtual_machine_name}"
        return 1
    fi

    # Announce backup is starting
    log_message "Starting backup process for ${virtual_machine_name}"

    # Save virtual machine's config
    virsh dumpxml "$virtual_machine_name" > "$config_file"

    # Set ownership and permissions for config file
    chown libvirt-qemu:kvm "$config_file"
    chmod 644 "$config_file"

    # Try to shut down the virtual machine nicely
    log_message "Shutting down ${virtual_machine_name}"
    virsh shutdown "$virtual_machine_name"

    # Wait patiently for the virtual machine to shut down
    local count=0
    while [ "$(virsh domstate $virtual_machine_name)" != "shut off" ] && [ $count -lt 30 ]; do
        sleep 10
        count=$((count + 1))
    done

    # If it doesn't turn off, make it turn off(like holding the power button)
    if [ "$(virsh domstate $virtual_machine_name)" != "shut off" ]; then
        log_message "WARNING: Force shutting down ${virtual_machine_name}"
        virsh destroy "$virtual_machine_name"
        sleep 10
    fi

    # Make sure it's actually off - trust but verify
    if [ "$(virsh domstate $virtual_machine_name)" != "shut off" ]; then
        log_message "ERROR: Failed to shut down ${virtual_machine_name}"
        return 1
    fi

    # Create the backup - doesn't use compression since qemu-img convert compression is single threaded and insanely slow
    log_message "Creating backup of ${virtual_machine_name}"
    if ! qemu-img convert -p -f qcow2 -O qcow2 "$source_file" "$backup_file"; then
        log_message "ERROR: Backup failed for ${virtual_machine_name}"
        virsh start "$virtual_machine_name"
        return 1
    fi

    # Set ownership and permissions for backup file
    chown libvirt-qemu:kvm "$backup_file"
    chmod 644 "$backup_file"

    # Make sure the backup isn't insanely small since that means this didn't work
    # Fixed stat command for Linux systems
    local source_size=$(stat -c%s "$source_file")
    local backup_size=$(stat -c%s "$backup_file")
    if [ "$backup_size" -lt 1048576 ]; then  # Less than 1MB is suspicious - like a $5 "genuine" Rolex
        log_message "ERROR: Backup file suspiciously small for ${virtual_machine_name}"
        rm -f "$backup_file" "$config_file"
        virsh start "$virtual_machine_name"
        return 1
    fi

    # Turn virtual machine back on when backup is done.
    log_message "Starting ${virtual_machine_name}"
    virsh start "$virtual_machine_name"

    # Wait for it to come back online
    count=0
    while [ "$(virsh domstate $virtual_machine_name)" != "running" ] && [ $count -lt 12 ]; do
        sleep 5
        count=$((count + 1))
    done

    # Make sure it actually started(inspect what you expect)
    if [ "$(virsh domstate $virtual_machine_name)" != "running" ]; then
        log_message "ERROR: Failed to start ${virtual_machine_name}"
        return 1
    fi

    # announce that it worked
    log_message "Backup of ${virtual_machine_name} completed!"

    # Clean up old backups - because nobody likes a full hard drive
    log_message "Cleaning up old backups for ${virtual_machine_name}"
    find "$BACKUP_DIR" -name "${virtual_machine_name}-*.qcow2" -mtime +${RETENTION_DAYS} -exec rm -f {} \;  # Delete old qcow2 files
    find "$BACKUP_DIR" -name "${virtual_machine_name}-*.xml" -mtime +${RETENTION_DAYS} -exec rm -f {} \;   # Delete old xml files
}

# Start of the main backup process
log_message "Starting backup process"

# Make sure we're running as root
if [ "$EUID" -ne 0 ]; then
    log_message "ERROR: Must run as root"
    exit 1
fi

# Check if the backup directory exists
if [ ! -d "$BACKUP_DIR" ]; then
    log_message "ERROR: Backup directory $BACKUP_DIR does not exist"
    exit 1
fi

# Get list of ALL virtual machines, not just running ones
# Changed to list all VMs instead of just running ones
VMS=($(virsh list --all --name))

# Check if we have enough disk space to back up
available_space=$(df -B1 "$BACKUP_DIR" | awk 'NR==2 {print $4}')
required_space=0

# Calculate how much space we need
for virtual_machine in "${VMS[@]}"; do
    if [ -n "$virtual_machine" ]; then
        # Try the default path first
        local_path="/var/lib/libvirt/images/${virtual_machine}.qcow2"

        # If default path doesn't exist, try to find actual path
        if [ ! -f "$local_path" ]; then
            local_path=$(find_vm_disk_path "$virtual_machine") || continue
        fi

        if [ -f "$local_path" ]; then
            virtual_machine_size=$(du -b "$local_path" 2>/dev/null | cut -f1)
            required_space=$((required_space + virtual_machine_size))
        fi
    fi
done

# Make sure we have enough space
if [ "$available_space" -lt "$required_space" ]; then
    log_message "ERROR: Insufficient space in backup directory"
    exit 1
fi

# loop for backing up every virtual machine
for virtual_machine in "${VMS[@]}"; do
    if [ -n "$virtual_machine" ]; then
        backup_vm "$virtual_machine"
    fi
done

# announce it's all done
log_message "Backup process completed!"
 */

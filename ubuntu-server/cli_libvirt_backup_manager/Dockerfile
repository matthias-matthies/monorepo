FROM --platform=linux/amd64 devraymondsh/ubuntu-rust:latest AS build
LABEL authors="matthias"

RUN apt-get update

RUN apt-get install -y \
    build-essential \
    gcc \
    g++ \
    pkg-config  \
    libssl-dev \
    qemu-kvm \
    qemu-utils \
    libvirt-daemon-system  \
    libvirt-clients \
    bridge-utils

WORKDIR /cli_libvirt_backup_manager
COPY . .

RUN cargo build --target x86_64-unknown-linux-gnu
#RUN cargo build --release --target x86_64-unknown-linux-gnu

FROM ubuntu:24.04

RUN apt-get update
RUN apt-get install -y \
    gcc \
    g++ \
    qemu-kvm \
    qemu-utils \
    libvirt-daemon-system  \
    libvirt-clients \
    bridge-utils

WORKDIR /cli_libvirt_backup_manager

COPY --from=build /cli_libvirt_backup_manager/target/x86_64-unknown-linux-gnu/debug/cli_libvirt_backup_manager ./cli_libvirt_backup_manager
#COPY --from=build /cli_libvirt_backup_manager/target/x86_64-unknown-linux-gnu/release/cli_libvirt_backup_manager ./cli_libvirt_backup_manager
COPY --from=build /cli_libvirt_backup_manager/assets/example.xml ./example.xml

ENTRYPOINT ["./cli_libvirt_backup_manager"]

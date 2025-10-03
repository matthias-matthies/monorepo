FROM devraymondsh/ubuntu-rust:latest AS build
LABEL authors="matthias"

RUN apt-get update

RUN apt-get install -y \
    build-essential \
    libvirt-daemon-system  \
    libvirt-clients

WORKDIR /cli_libvirt_backup_manager
COPY . .

RUN cargo build --release

FROM ubuntu:24.04

RUN apt-get update
RUN apt-get install -y \
    libvirt-daemon-system  \
    libvirt-clients

WORKDIR /cli_libvirt_backup_manager

COPY --from=build /cli_libvirt_backup_manager/target/release/cli_libvirt_backup_manager ./cli_libvirt_backup_manager
COPY --from=build /cli_libvirt_backup_manager/assets/example.xml ./example.xml

ENTRYPOINT ["./cli_libvirt_backup_manager"]

FROM ubuntu:latest
LABEL authors="matthias"

# Update default packages
RUN apt-get update

# Get Ubuntu packages
RUN apt-get install -y \
    build-essential \
    cargo \
    

WORKDIR /cli_libvirt_backup_manager
COPY Cargo.toml ./
COPY src ./
COPY build.rs ./



ENTRYPOINT ["top", "-b"]
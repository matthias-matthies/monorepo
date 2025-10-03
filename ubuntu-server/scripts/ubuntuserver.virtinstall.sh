# Setup for container server
virt-install \
    --name ubuntu24-server-manual \ # Change this to something like container-server-04 for ubuntu server for containers with ip ending on 04
    --ram 4096 \
    --vcpus 4 \
    --disk path=/var/lib/libvirt/images/ubuntu24-server-manual.qcow2,size=25 \
    --os-variant ubuntu22.04 \
    --network bridge=br0 \
    --graphics none \
    --console pty,target_type=serial \
    --location /var/lib/libvirt/images/ubuntu-24.04.3-live-server-amd64.iso,kernel=casper/vmlinuz,initrd=casper/initrd \
    --extra-args 'console=ttyS0'

# Installing docker for
sudo apt update ; sudo apt upgrade
sudo apt remove docker docker-engine docker.io containerd runc
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
sudo apt install docker-compose-plugin -y
sudo usermod -aG docker $USER
newgrp docker

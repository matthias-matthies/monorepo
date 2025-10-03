sudo virt-install \
    --name haos \
    --description "Home Assistant OS" \
    --os-variant=generic \
    --ram=4096 \
    --vcpus=2 \
    --disk /var/lib/libvirt/images/haos_ova-16.1.qcow2,bus=scsi \
    --controller type=scsi,model=virtio-scsi \
    --import --graphics none --boot uefi

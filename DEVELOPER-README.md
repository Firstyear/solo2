## Building and Flashing Firmware.

The solo2 firmware flash process is difficult to follow. This guide attempts to make it easier for
you to start creating your own images.

### Warning

This process will work directly on linux, but as each distro varies in library version and pathing
it may not always be consistent. To isolate this, this guide assumes building in docker and moving
the built artefacts out of the container.

It is not possible to build these roms on windows or macos. You must use docker.

Outside of the build process, you can run the other tools.

## Setup

### Install Local Tools

These tools should be installed on your host. They work on linux or macos.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install solo2
cargo install lpc55
```

### Build Container

Build the "builder" container.

```
docker build --progress plain -f Dockerfile.builder -t solo2/builder:latest .
```

You can then build the firmware with:

```
docker run -i -t -v .:/solo2 -w /solo2/runners/lpc55/ solo2/builder:latest bash -c "source /root/.profile && make build-pro"
```

If you want to have a "live building" experience

```
docker run -i -t -v .:/solo2 -w /solo2/runners/lpc55/ solo2/builder:latest bash
```

## Flashing

Once complete the binaries will be located at:

```
./runners/lpc55/provisioner.bin
```

Put the device into bootloader mode:

```
solo2 app admin maintenance
```

List devices in bootloader mode:

```
lpc55 ls
# Bootloader { vid: 1209, pid: B000, uuid: 2B0... }
```

You can make a backup. Solo helpefully don't tell you how to restore it though 🙃

```
# Read the program memory flash
lpc55 read-memory -vvv 0 524288 -o solo2-fw-backup-program.bin
# Also read the Protected Flash Region pages.
# CFPA scratch 0x9de00
lpc55 read-memory -vvv 646656 512 -o solo2-fw-backup-pfr-cfpa-scratch.bin
# CFPA ping 0x9e000
lpc55 read-memory -vvv 647168 512 -o solo2-fw-backup-pfr-cfpa-ping.bin
# CFPA pong 0x9e200
lpc55 read-memory -vvv 647680 512 -o solo2-fw-backup-pfr-cfpa-pong.bin
# CMPA 0x9e400
lpc55 read-memory -vvv 648192 512 -o solo2-fw-backup-pfr-cmpa.bin
```

Disable secure boot

https://hackmd.io/@solokeys/solo2-getting-started#Disabling-Secure-Boot

```
lpc55 pfr yaml > pfr-settings.yaml
cp pfr-settings.yaml pfr-settings-no-secure-boot.yaml
$EDITOR pfr-settings-no-secure-boot.yaml
```

```
factory:
  secure-boot-configuration:
    secure-boot-enabled: false
```

```
lpc55 configure factory-settings -vvv pfr-settings-no-secure-boot.yaml
```

Now the binary can be flashed.

```
lpc55 write-flash -vvv ./runners/lpc55/provisioner.bin
```

Finally reboot into the new FW

```
lpc55 reboot
```

There seems to be no verification proceed post flash. 🙃

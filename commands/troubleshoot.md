# Troubleshooting Skill

Systematic troubleshooting guide for the Raspberry Pi gaming console. Use this when something isn't working correctly.

## Diagnostic Framework

### 1. Identify the Problem Category
- **Boot issues**: System won't start, stuck at logo, no display
- **Display issues**: Black screen, wrong resolution, no 4K
- **Audio issues**: No sound, wrong output, crackling
- **Controller issues**: Not detected, wrong mapping, lag
- **Emulation issues**: Game won't launch, crashes, poor performance
- **Network issues**: No WiFi, SSH fails, Samba not working
- **Storage issues**: SSD not mounting, slow performance

### 2. Quick Diagnostics
```bash
# System logs
journalctl -b -p err    # Errors since boot
dmesg | tail -50        # Recent kernel messages

# Service status
systemctl status        # Overall system status
systemctl --failed      # Failed services

# Resource usage
htop                    # CPU/memory usage
iotop                   # Disk I/O
```

## Common Issues & Solutions

### Boot Problems (SSD Boot)
```bash
# Check boot config
cat /boot/firmware/config.txt
cat /boot/firmware/cmdline.txt

# USB/NVMe boot order
sudo rpi-eeprom-config --edit
# Set BOOT_ORDER=0xf416 for NVMe first
```

### Display Issues
```bash
# Check HDMI status
tvservice -s

# Force 4K output (config.txt)
hdmi_enable_4kp60=1
hdmi_group=2
hdmi_mode=97

# Check KMS/DRM
cat /sys/class/drm/card*/status
```

### No Audio
```bash
# List audio devices
aplay -l

# Set HDMI audio (config.txt)
dtparam=audio=on
hdmi_drive=2

# RetroArch audio settings
# Check audio driver: alsathread or sdl2
```

### Controller Not Working
```bash
# Check device exists
ls -la /dev/input/js* /dev/input/event*

# Check Bluetooth
systemctl status bluetooth
bluetoothctl show

# Reload Xbox driver
sudo modprobe -r xpad
sudo modprobe xpad
```

### Game Won't Launch
1. Check EmulationStation log: `~/.emulationstation/es_log.txt`
2. Check RetroArch log: `/tmp/retroarch.log` or `~/.config/retroarch/retroarch.log`
3. Verify ROM file integrity
4. Check required BIOS files
5. Try different emulator core

### Poor Performance
```bash
# Check for throttling
vcgencmd get_throttled

# Check CPU governor
cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor
# Set to performance:
echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor

# Check temperature
cat /sys/class/thermal/thermal_zone0/temp
```

### Network Issues
```bash
# Check WiFi
iwconfig wlan0
nmcli device status

# Restart networking
sudo systemctl restart NetworkManager
# or
sudo systemctl restart dhcpcd

# Check Samba
sudo systemctl status smbd
testparm -s
```

### Storage Issues
```bash
# Check mounts
mount | grep -E "(sda|nvme)"
lsblk

# Check SSD health
sudo smartctl -a /dev/sda  # or /dev/nvme0n1

# Remount
sudo mount -a
```

## Log Locations
- System: `journalctl`
- EmulationStation: `~/.emulationstation/es_log.txt`
- RetroArch: `~/.config/retroarch/retroarch.log`
- Xorg: `/var/log/Xorg.0.log`
- Boot: `dmesg`

## User's Issue
$ARGUMENTS

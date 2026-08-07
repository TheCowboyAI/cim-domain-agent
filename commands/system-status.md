# System Status Check Skill

Check the current status of the Raspberry Pi gaming console. This skill provides commands and interprets results for system health monitoring.

## Status Checks to Perform

### Hardware Status
```bash
# CPU temperature
cat /sys/class/thermal/thermal_zone0/temp
# Divide by 1000 for Celsius

# CPU frequency
cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq
# In kHz

# Memory usage
free -h

# Disk usage
df -h /media/games

# GPU memory (Pi 5)
vcgencmd get_mem gpu
```

### Controller Status
```bash
# Check if controller connected
ls /dev/input/js*

# Bluetooth devices
bluetoothctl devices

# Test joystick
jstest --normal /dev/input/js0
```

### Emulation Status
```bash
# Check RetroArch version
retroarch --version

# EmulationStation process
pgrep emulationstation

# RetroPie installed packages
ls /opt/retropie/emulators/
```

### Network Status
```bash
# IP address
hostname -I

# Samba status
systemctl status smbd

# SSH status
systemctl status sshd
```

### Thermal & Power
```bash
# Throttling status
vcgencmd get_throttled
# 0x0 = no throttling

# Voltage status
vcgencmd measure_volts
```

## Interpreting Results

### Temperature Guidelines
- <50°C: Excellent
- 50-65°C: Normal under load
- 65-80°C: High, ensure cooling
- >80°C: Throttling likely, improve cooling

### Throttling Flags (get_throttled)
- Bit 0: Under-voltage detected
- Bit 1: Arm frequency capped
- Bit 2: Currently throttled
- Bit 3: Soft temperature limit active

## Project Context
Reference `progress.json` for current sprint and configuration status.

## User's Request
$ARGUMENTS

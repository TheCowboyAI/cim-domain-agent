# Performance Optimization Skill

Optimize the Raspberry Pi 5 gaming console for best gaming performance. Apply these settings based on specific needs.

## Quick Optimization Checklist

### 1. CPU Performance Mode
```bash
# Set performance governor (temporary)
echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor

# Make permanent via config.txt
# Add: arm_boost=1
```

### 2. GPU Optimization (config.txt)
```ini
# Increase GPU memory for 4K
gpu_mem=512

# Enable 4K60
hdmi_enable_4kp60=1

# Force turbo (use with good cooling!)
# force_turbo=1

# Overclock (optional, requires cooling)
# over_voltage=6
# arm_freq=3000
# gpu_freq=1000
```

### 3. RetroArch Optimizations
Edit `~/.config/retroarch/retroarch.cfg` or `/opt/retropie/configs/all/retroarch.cfg`:

```ini
# Video
video_driver = "gl"
video_vsync = "true"
video_max_swapchain_images = "3"
video_frame_delay = "0"

# Audio (reduce latency)
audio_driver = "alsathread"
audio_latency = "64"

# Threading
video_threaded = "true"

# Run-ahead (reduces input lag, CPU intensive)
# run_ahead_enabled = "true"
# run_ahead_frames = "1"
```

### 4. Per-System Optimizations

#### Arcade (MAME/FBNeo)
- Use FBNeo for better Pi 5 performance
- Enable frameskip for demanding games
- Use lower internal resolution if needed

#### DOS (DOSBox-Pure)
- Set cycles=max for Pi 5
- Enable dynamic recompiler
- Match original game requirements

#### N64 (Mupen64Plus-Next)
- Use ParaLLEl RDP plugin
- Enable threaded video
- Lower internal resolution for demanding games

#### PS1 (PCSX-ReARMed)
- Enable NEON dynarec
- Use software renderer for compatibility
- Hardware renderer for speed

### 5. Reduce System Overhead
```bash
# Disable unnecessary services
sudo systemctl disable bluetooth  # if using USB controller
sudo systemctl disable cups
sudo systemctl disable avahi-daemon

# Reduce logging
sudo journalctl --vacuum-size=50M
```

### 6. Storage Optimization
```bash
# For SSD boot - enable TRIM
sudo fstrim -v /

# Add to fstab for auto-TRIM
# UUID=xxx / ext4 defaults,noatime,discard 0 1

# Use noatime to reduce writes
# Add 'noatime' to mount options
```

### 7. Display Latency
```ini
# config.txt - disable overscan
disable_overscan=1

# RetroArch - use exclusive fullscreen
video_fullscreen = "true"
video_windowed_fullscreen = "false"
```

### 8. Input Latency
```bash
# Reduce USB polling interval (cmdline.txt)
# Add: usbhid.mousepoll=1

# RetroArch input settings
input_poll_type_behavior = "2"
```

### 9. Cooling
- Target <70°C under load
- Active cooling recommended for sustained gaming
- Monitor with: `watch -n1 'cat /sys/class/thermal/thermal_zone0/temp'`

### 10. Memory
```bash
# Check memory pressure
cat /proc/meminfo | grep -E "(MemFree|Buffers|Cached)"

# With 16GB RAM, disable swap
sudo swapoff -a
```

## Benchmarking
```bash
# CPU benchmark
sysbench cpu run

# Storage benchmark
sudo hdparm -Tt /dev/sda  # or nvme0n1

# GPU test (in RetroArch)
# Enable FPS display, test demanding games
```

## User's Request
$ARGUMENTS

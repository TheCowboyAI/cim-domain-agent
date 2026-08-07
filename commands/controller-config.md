# Controller Configuration Skill

Help configure game controllers for the Raspberry Pi gaming console. Specialized for Xbox One Elite Wireless Controller but applicable to other controllers.

## Current Setup
- **Controller**: Xbox One Elite Wireless Controller
- **Connection**: Bluetooth (MAC: 98:7A:14:44:EF:B8)
- **Device**: /dev/input/js0
- **Specs**: 8 axes, 16 buttons

## Configuration Tasks

### Bluetooth Pairing (Xbox Controller)
```bash
# Ensure ERTM is disabled (required for Xbox)
echo 'options bluetooth disable_ertm=Y' | sudo tee /etc/modprobe.d/bluetooth.conf
sudo modprobe -r btusb && sudo modprobe btusb

# Pair via bluetoothctl
bluetoothctl
> power on
> agent on
> default-agent
> scan on
# Put controller in pairing mode (hold Xbox button + sync)
> pair 98:7A:14:44:EF:B8
> trust 98:7A:14:44:EF:B8
> connect 98:7A:14:44:EF:B8
> quit
```

### Test Controller
```bash
# Basic test
jstest /dev/input/js0

# Detailed events
evtest /dev/input/event*  # select controller device
```

### EmulationStation Configuration
```
~/.emulationstation/es_input.cfg
```
Configure in ES: Start > Configure Input

### RetroArch Configuration
```
~/.config/retroarch/autoconfig/
/opt/retropie/configs/all/retroarch/autoconfig/
```
RetroArch usually auto-detects Xbox controllers.

### Per-Game Controller Profiles
Create game-specific configs in:
- `/opt/retropie/configs/<system>/retroarch.cfg`
- Use runcommand menu for per-game emulator/config selection

### Hotkey Configuration
Common RetroArch hotkeys:
- Select + Start: Exit game
- Select + R1: Save state
- Select + L1: Load state
- Select + Right: Next save slot
- Select + Left: Previous save slot
- Select + X: Screenshot
- Select + Y: Toggle FPS

## Troubleshooting

### Controller Not Detected
1. Check Bluetooth: `bluetoothctl devices`
2. Check connection: `bluetoothctl connect <MAC>`
3. Check device: `ls /dev/input/js*`
4. Reload driver: `sudo modprobe -r xpad && sudo modprobe xpad`

### Drift or Calibration Issues
```bash
# Calibrate with jscal
jscal -c /dev/input/js0
```

### Connection Drops
- Ensure controller is trusted: `bluetoothctl trust <MAC>`
- Check battery level
- Reduce distance/interference

## User's Request
$ARGUMENTS

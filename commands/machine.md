# Machine Management Skill

Manage and deploy configurations for the gaming console fleet. This skill helps with machine-specific operations across gpi-blu, gpi-tommy, and g86-steele.

## Fleet Overview

| Machine | Hostname | Arch | Status |
|---------|----------|------|--------|
| gpi-blu | `gpi-blu` | ARM64 (Pi 5) | Active |
| gpi-tommy | `gpi-tommy` | ARM64 (Pi 5) | Provisioning |
| gx86-steele | `gx86-steele` | x86_64 | Active |

## Configuration Files

```
machines/
├── common/base-config.json     # Shared settings
├── gpi-blu/machine.json        # gpi-blu overrides
├── gpi-tommy/machine.json      # gpi-tommy overrides
└── gx86-steele/machine.json    # gx86-steele overrides
```

## Common Operations

### Check Machine Status
```bash
# Ping all machines
ping -c 1 gpi-blu
ping -c 1 gpi-tommy
ping -c 1 gx86-steele

# SSH test
ssh root@gpi-blu "hostname && uptime"
ssh root@gpi-tommy "hostname && uptime"
```

### Deploy Configuration
```bash
# Deploy to specific machine (Batocera)
scp machines/<machine>/batocera.conf root@<hostname>:/userdata/system/

# Apply hostname
ssh root@<hostname> "batocera-config hostname <hostname>"
```

### Sync ROMs Between Machines
```bash
# From gpi-blu to gpi-tommy
rsync -avz --progress root@gpi-blu:/userdata/roms/ root@gpi-tommy:/userdata/roms/

# Sync BIOS files
rsync -avz root@gpi-blu:/userdata/bios/ root@gpi-tommy:/userdata/bios/

# Sync saves (careful - may overwrite progress!)
rsync -avz root@gpi-blu:/userdata/saves/ root@gpi-tommy:/userdata/saves/
```

### Pair Controller to Machine
```bash
ssh root@<hostname>
bluetoothctl
> power on
> agent on
> scan on
# Put controller in pairing mode
> pair <MAC>
> trust <MAC>
> connect <MAC>
> quit
```

### Update Machine Config
After making changes:
1. Edit `machines/<machine>/machine.json`
2. Update `history` array with change log
3. Set appropriate `status` field

## Machine Provisioning Checklist

For new machines (like gpi-tommy):

- [ ] Flash OS to SSD (Batocera/RetroPie/Arch)
- [ ] Boot and verify basic operation
- [ ] Set hostname: `batocera-config hostname gpi-tommy`
- [ ] Configure WiFi (if not via config file)
- [ ] Enable SSH
- [ ] Add SSH authorized keys
- [ ] Pair Bluetooth controller (record MAC in machine.json)
- [ ] Sync ROMs from primary machine
- [ ] Sync BIOS files
- [ ] Test game launch
- [ ] Update machine.json status to "active"

## Architecture Differences

### ARM64 Machines (gpi-blu, gpi-tommy)
- Raspberry Pi 5, 16GB RAM
- Batocera or RetroPie
- NVMe SSD boot
- RetroArch + libretro cores
- Optional: Steam ARM64 + FEX-Emu

### x86_64 Machine (gx86-steele)
- Standard PC architecture
- Native Steam client
- Proton for Windows games
- RetroArch available
- No translation layer needed

## Troubleshooting

### Machine Not Reachable
1. Check physical network connection
2. Verify hostname in `/etc/hosts` or DNS
3. Try IP address directly
4. Check if machine is powered on

### Configuration Mismatch
1. Read machine.json: `cat machines/<machine>/machine.json`
2. Compare with running config on machine
3. Re-deploy if needed

### Controller Paired to Wrong Machine
1. Unpair from current machine: `bluetoothctl remove <MAC>`
2. Pair to correct machine
3. Update machine.json with correct MAC

## User's Request
$ARGUMENTS

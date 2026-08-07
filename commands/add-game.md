# Add Game Skill

Help the user add a new game to the gaming console. This skill guides through the complete process of adding ROMs, BIOS files, and configuring the emulator.

## Process

1. **Identify the game and system**
   - What game does the user want to add?
   - What platform/system is it for (arcade, DOS, NES, etc.)?

2. **Check requirements**
   - Does this game require specific BIOS files?
   - What ROM format is needed (zip, 7z, CHD, iso)?
   - For arcade: What ROM set version is required?

3. **Determine file locations**
   - RetroPie: `/media/games/roms/<system>/` or `/home/pi/RetroPie/roms/<system>/`
   - Batocera: `/userdata/roms/<system>/`
   - BIOS: Check `neo4j/bios-cids.json` for tracked BIOS files

4. **Guide the transfer**
   - Samba share: `\\<pi-ip>\roms` or `\\<pi-ip>\share`
   - SCP: `scp game.zip pi@<ip>:/media/games/roms/<system>/`
   - Direct copy if connected

5. **Post-transfer setup**
   - Refresh game list in EmulationStation (or restart ES)
   - Test the game launches correctly
   - Configure per-game settings if needed

6. **Troubleshooting**
   - Game not appearing: Check file extension, gamelist.xml
   - Game not launching: Check BIOS, ROM version, emulator core
   - Performance issues: Suggest core alternatives, settings

## Reference Documents
- `docs/game-collection-status.md` - Current game inventory
- `docs/dos-games-status.md` - DOS game specifics
- `neo4j/bios-cids.json` - BIOS file tracking
- `ARCHITECTURE.md` - System paths and structure

## User's Request
$ARGUMENTS

# PaoValues

A mnemonic PAO (Person-Action-Object) system for storing and sharing values using the major memory system technique.

## Overview

The PAO system assigns each two-digit number (00-99) a unique combination of:
- **Person**: A memorable character or figure
- **Action**: A distinctive action they perform
- **Object**: An object they interact with

This creates 100 "memory addresses" that can be used to store and retrieve information in a memory palace.

## Project Structure

```
PaoValues/
├── memory-bank/           # Project documentation (Cline)
│   ├── projectbrief.md    # Project overview and goals
│   ├── productContext.md  # Why this project exists
│   ├── activeContext.md   # Current work focus
│   ├── systemPatterns.md  # System architecture
│   ├── techContext.md     # Technologies and setup
│   └── progress.md        # What's done and what's next
├── data/                  # Data storage
│   ├── pao-systems/       # PAO system definitions (JSON)
│   │   └── default.json   # Default PAO system (100 entries)
│   ├── values/            # Stored values by index
│   │   └── default.json   # Values storage file
│   └── exports/           # Exported/shared files
├── scripts/               # Utility scripts (future)
├── .clinerules            # Cline memory bank rules
└── README.md              # This file
```

## Quick Start

### View the PAO System

The PAO system is stored in `data/pao-systems/default.json`. Each entry contains:

```json
{
  "number": "42",
  "person": "gRuNt",
  "action": "concentrates deeply",
  "object": "casino chips"
}
```

### Store a Value

Edit `data/values/default.json` to store values at specific indexes:

```json
{
  "42": {
    "data": "your value here",
    "type": "conceptual",
    "metadata": {
      "description": "Description of what's stored",
      "lastUpdated": "2026-04-10T00:00:00.000Z"
    }
  }
}
```

## Usage Examples

### Example 1: Memorizing a Number Sequence

To memorize the number 421876:
1. Break into pairs: 42-18-76
2. Recall PAO associations:
   - 42: gRuNt / concentrates deeply / casino chips
   - 18: TeFiti / throws / fireballs
   - 76: yoda / pokes / r2d2
3. Combine using PAO pattern (Person-Action-Object):
   - Person from 42: **gRuNt**
   - Action from 18: **throws**
   - Object from 76: **r2d2**
4. Create a memorable image: "A grunt throws r2d2"

### Example 2: Storing Conceptual Values

Use PAO indexes as addresses for storing information:
- Index 00: Store your core values/philosophy
- Index 50: Store important project notes
- Index 99: Store long-term goals

## PAO System Reference

| Number | Person | Action | Object |
|--------|--------|--------|--------|
| 00 | Girl from Cyberpunk 2077 | decodes/encodes | mnemonic bank |
| 01 | Buz/Woody from Toy Story | uses laser and rope | Toy Story toys |
| 02 | Starlord | forces | lightsaber |
| 03 | Lilya | prompts | AI |
| 04 | Denji | rips through | chainsaws |
| ... | ... | ... | ... |
| 99 | PoPeye | prise open a can | spinach |

See `data/pao-systems/default.json` for the complete list.

## Data Format

### PAO System Schema

```json
{
  "name": "Default PAO System",
  "description": "Primary PAO system for numbers 00-99",
  "version": "1.0",
  "createdAt": "2026-04-09T00:00:00.000Z",
  "entries": [
    {
      "number": "00",
      "person": "Character name",
      "action": "Action they perform",
      "object": "Object they use",
      "notes": "Optional memory aid"
    }
  ]
}
```

### Value Storage Schema

```json
{
  "paoSystem": "default",
  "description": "Values stored at PAO system indexes",
  "values": {
    "00": {
      "data": null,
      "type": "conceptual",
      "metadata": {
        "description": "What's stored here",
        "lastUpdated": "2026-04-10T00:00:00.000Z"
      }
    }
  }
}
```

## Future Features

- [ ] CLI commands for viewing/editing PAO entries
- [ ] Import/export functionality
- [ ] Practice/review mode
- [ ] Multiple PAO system support
- [ ] Web interface

## Contributing

This is a personal project, but feel free to fork and adapt it for your own use.

## License

MIT License
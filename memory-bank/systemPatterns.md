# System Patterns: PaoValues

## System Architecture

The PaoValues system follows a simple, file-based architecture designed for portability and extensibility:

```
┌─────────────────────────────────────────────────────────────┐
│                     PaoValues System                         │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐  │
│  │   CLI/UI     │    │   Scripts    │    │   Import/    │  │
│  │  Interface   │    │  Utilities   │    │   Export     │  │
│  └──────┬───────┘    └──────┬───────┘    └──────┬───────┘  │
│         │                   │                   │          │
│         └───────────────────┼───────────────────┘          │
│                             │                              │
│                    ┌────────▼────────┐                     │
│                    │   Data Layer    │                     │
│                    │   (JSON Files)  │                     │
│                    └────────┬────────┘                     │
│                             │                              │
│         ┌───────────────────┼───────────────────┐          │
│         │                   │                   │          │
│  ┌──────▼───────┐    ┌──────▼───────┐    ┌──────▼───────┐  │
│  │   PAO        │    │   Values     │    │   Shared     │  │
│  │   Systems    │    │   Storage    │    │   Exports    │  │
│  │ data/pao-    │    │ data/values/ │    │   exports/   │  │
│  │ systems/     │    │              │    │              │  │
│  └──────────────┘    └──────────────┘    └──────────────┘  │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Key Technical Decisions

### 1. File-Based Storage
- **Decision**: Use JSON files for all data storage
- **Rationale**: Simple, portable, version-control friendly, no database setup required
- **Trade-offs**: Limited query capabilities, but sufficient for initial scope

### 2. Modular Data Structure
- **Decision**: Separate PAO systems from stored values
- **Rationale**: Allows multiple PAO systems to coexist; values can be system-specific
- **Trade-offs**: Slightly more complex data management, but more flexible

### 3. Extensible Schema
- **Decision**: Include optional fields (notes, metadata) in PAO entries
- **Rationale**: Supports future enhancements without breaking existing data
- **Trade-offs**: Larger file sizes, but negligible for 100 entries

## Design Patterns in Use

### 1. Repository Pattern
```javascript
// PaoRepository handles all PAO data operations
class PaoRepository {
  load(systemName) { }
  save(systemName, data) { }
  getEntry(number) { }
  updateEntry(number, entry) { }
}

// ValueRepository handles value storage
class ValueRepository {
  store(index, value) { }
  retrieve(index) { }
  list() { }
}
```

### 2. Factory Pattern
```javascript
// Create different types of PAO systems
class PaoSystemFactory {
  createDefault() { }
  createFromImport(data) { }
  createEmpty() { }
}
```

### 3. Strategy Pattern
```javascript
// Different export/import strategies
class ExportStrategy {
  toJSON(data) { }
  toCSV(data) { }
  toMarkdown(data) { }
}
```

## Component Relationships

### Data Flow
1. **PAO Management**: User → CLI/UI → PaoRepository → JSON Files
2. **Value Storage**: User → CLI/UI → ValueRepository → JSON Files
3. **Import/Export**: External Files → ImportStrategy → PaoRepository → JSON Files

### File Structure
```
PaoValues/
├── memory-bank/           # Project documentation
│   ├── projectbrief.md
│   ├── productContext.md
│   ├── activeContext.md
│   ├── systemPatterns.md
│   ├── techContext.md
│   └── progress.md
├── data/                  # Data storage
│   ├── pao-systems/       # PAO system definitions
│   │   └── default.json
│   ├── values/            # Stored values by index
│   │   └── default.json
│   └── exports/           # Exported/shared files
├── src/                   # Source code (future)
├── scripts/               # Utility scripts
└── .clinerules            # Cline memory bank rules
```

## Critical Implementation Paths

### Phase 1: Foundation (Current)
1. Create data directory structure
2. Define JSON schemas for PAO and values
3. Store user's existing 100 PAO associations
4. Create basic utility scripts

### Phase 2: Utilities
1. CLI commands for viewing/editing PAO entries
2. Import/export functionality
3. Value storage and retrieval

### Phase 3: Applications
1. Web interface
2. Practice/review mode
3. Multiple PAO system support

## Data Schemas

### PAO System Schema
```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "PAO System",
  "type": "object",
  "properties": {
    "name": { "type": "string" },
    "description": { "type": "string" },
    "version": { "type": "string", "default": "1.0" },
    "createdAt": { "type": "string", "format": "date-time" },
    "entries": {
      "type": "array",
      "items": { "$ref": "#/definitions/paoEntry" },
      "minItems": 100,
      "maxItems": 100
    }
  },
  "definitions": {
    "paoEntry": {
      "type": "object",
      "properties": {
        "number": { "type": "string", "pattern": "^[0-9]{2}$" },
        "person": { "type": "string" },
        "action": { "type": "string" },
        "object": { "type": "string" },
        "notes": { "type": "string" }
      },
      "required": ["number", "person", "action", "object"]
    }
  }
}
```

### Value Storage Schema
```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Value Storage",
  "type": "object",
  "properties": {
    "paoSystem": { "type": "string" },
    "values": {
      "type": "object",
      "additionalProperties": {
        "type": "object",
        "properties": {
          "data": { },
          "type": { "type": "string" },
          "metadata": { "type": "object" }
        }
      }
    }
  }
}
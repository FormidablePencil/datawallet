# Tech Context: PaoValues

## Technologies Used

### Current Stack (File-Based Foundation)
- **JSON**: Data format for PAO systems and value storage
- **Markdown**: Documentation format
- **Node.js** (optional): For utility scripts and future CLI tools

### Future Stack (Planned)
- **Frontend**: React or Vue.js for web interface
- **Backend**: Node.js with Express or Python with Flask
- **Database**: SQLite or PostgreSQL (when scaling beyond files)
- **Storage**: File system for initial phase, cloud storage for sharing

## Development Setup

### Prerequisites
- Node.js (v18+) - optional, for scripts
- Git - for version control
- Text editor (VS Code recommended)

### Project Structure
```
PaoValues/
├── memory-bank/           # Project documentation (Cline)
│   ├── projectbrief.md
│   ├── productContext.md
│   ├── activeContext.md
│   ├── systemPatterns.md
│   ├── techContext.md
│   └── progress.md
├── data/                  # Data storage
│   ├── pao-systems/       # PAO system definitions (JSON)
│   ├── values/            # Stored values (JSON)
│   └── exports/           # Exported/shared files
├── scripts/               # Utility scripts
├── src/                   # Source code (future)
├── package.json           # Node.js config (future)
├── .clinerules            # Cline memory bank rules
└── README.md              # Project readme
```

### Setup Commands
```bash
# Clone or navigate to project
cd PaoValues

# Create data directories
mkdir -p data/pao-systems data/values data/exports

# (Optional) Initialize Node.js project
npm init -y

# (Optional) Install dependencies for scripts
npm install
```

## Technical Constraints

### Current Constraints
1. **File-based only**: No database, limited to JSON file operations
2. **Single user**: Initial design for personal use
3. **Manual sync**: No automatic backup or sync (user responsible)

### Future Considerations
1. **Multi-user support**: Will require database and authentication
2. **Large file handling**: Images/media may need separate storage
3. **API rate limiting**: When building web service
4. **Data validation**: Schema validation for PAO entries

## Dependencies

### Current (None)
The file-based approach requires no external dependencies.

### Planned Dependencies
```json
{
  "dependencies": {
    "ajv": "^8.0.0",        // JSON schema validation
    "commander": "^12.0.0"  // CLI framework
  },
  "devDependencies": {
    "prettier": "^3.0.0"    // Code formatting
  }
}
```

## Tool Usage Patterns

### Data Management
- **Read**: Load JSON files, parse, validate against schema
- **Write**: Serialize to JSON, format with Prettier, save
- **Validate**: Check schema compliance before saving

### Import/Export
- **Import**: Parse various formats (JSON, CSV), map to PAO schema
- **Export**: Generate JSON, CSV, Markdown for sharing

### Version Control
- Commit data files with meaningful messages
- Use branches for experimental PAO systems
- Tag releases of PAO system versions

## Coding Standards

### JavaScript/TypeScript (for future scripts)
- Use ES6+ syntax
- Async/await for file operations
- Modular design with separate concerns
- JSDoc comments for documentation

### JSON Data
- Use consistent key naming (camelCase)
- Include metadata (createdAt, version)
- Validate against schema before saving
- Format with 2-space indentation

### Documentation
- Markdown for all documentation
- Include examples in README
- Document all public functions
- Keep memory bank updated
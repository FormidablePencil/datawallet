//! Data module containing the PAO system and coding components

use serde::Deserialize;

/// A single PAO entry (Person-Action-Object)
#[derive(Clone, Debug, Deserialize)]
pub struct PAOEntry {
    pub number: String,
    pub person: String,
    pub action: String,
    pub object: String,
    #[serde(default)]
    pub notes: Option<String>,
}

/// PAO Index with computed display values
#[derive(Clone, Debug, PartialEq)]
pub struct PAOIndex {
    pub index: usize,
    pub person: String,
    pub action: String,
    pub object: String,
    pub logic_component: LogicComponent,
    pub visual_element: VisualElement,
}

/// A coding component (used for both Logic and Visual)
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CodingComponent {
    pub id: usize,
    pub name: String,
    pub category: String,
    pub description: String,
}

/// Logic Component type alias
pub type LogicComponent = CodingComponent;

/// Visual Element type alias
pub type VisualElement = CodingComponent;

/// Get all PAO entries from the default system
pub fn get_pao_entries() -> Vec<PAOEntry> {
    vec![
        PAOEntry { number: "00".into(), person: "Girl from Cyberpunk 2077".into(), action: "decodes/encodes".into(), object: "mnemonic bank".into(), notes: None },
        PAOEntry { number: "01".into(), person: "Buz/Woody from Toy Story".into(), action: "uses laser and rope".into(), object: "Toy Story toys".into(), notes: None },
        PAOEntry { number: "02".into(), person: "Starlord".into(), action: "forces".into(), object: "lightsaber".into(), notes: None },
        PAOEntry { number: "03".into(), person: "Lilya".into(), action: "prompts".into(), object: "AI".into(), notes: None },
        PAOEntry { number: "04".into(), person: "Denji".into(), action: "rips through".into(), object: "chainsaws".into(), notes: None },
        PAOEntry { number: "05".into(), person: "Boys".into(), action: "work with controllers".into(), object: "Animal Crossing".into(), notes: None },
        PAOEntry { number: "06".into(), person: "Kassandra from Assassin's Creed".into(), action: "assassinates".into(), object: "NPC".into(), notes: None },
        PAOEntry { number: "07".into(), person: "Zero Two from Franxx".into(), action: "aims with Franxx weapon".into(), object: "Franxx weapon(s)".into(), notes: None },
        PAOEntry { number: "08".into(), person: "Rick Sanchez".into(), action: "arms/disarms".into(), object: "bomb".into(), notes: None },
        PAOEntry { number: "09".into(), person: "Asuna from SAO".into(), action: "squats".into(), object: "pc muscles".into(), notes: None },
        PAOEntry { number: "10".into(), person: "Coolade Guy".into(), action: "ran over".into(), object: "motorcycle".into(), notes: None },
        PAOEntry { number: "11".into(), person: "Aladdin".into(), action: "bends".into(), object: "lamp".into(), notes: None },
        PAOEntry { number: "12".into(), person: "Tank Evans".into(), action: "polishes".into(), object: "surfboard".into(), notes: None },
        PAOEntry { number: "13".into(), person: "Timon".into(), action: "sucks".into(), object: "finger".into(), notes: None },
        PAOEntry { number: "14".into(), person: "T-Rex".into(), action: "claps together".into(), object: "hands".into(), notes: None },
        PAOEntry { number: "15".into(), person: "Diglett".into(), action: "digs".into(), object: "treasure map".into(), notes: None },
        PAOEntry { number: "16".into(), person: "Joker".into(), action: "smiles and laughs".into(), object: "eggs".into(), notes: None },
        PAOEntry { number: "17".into(), person: "Donkey".into(), action: "sniffs".into(), object: "grass".into(), notes: None },
        PAOEntry { number: "18".into(), person: "Te Fiti".into(), action: "throws".into(), object: "fireballs".into(), notes: None },
        PAOEntry { number: "19".into(), person: "Dumbo".into(), action: "burps".into(), object: "nuts".into(), notes: None },
        PAOEntry { number: "20".into(), person: "Jay".into(), action: "vomits".into(), object: "minions".into(), notes: None },
        PAOEntry { number: "21".into(), person: "Andy".into(), action: "backflips".into(), object: "Vans".into(), notes: None },
        PAOEntry { number: "22".into(), person: "Bunny".into(), action: "blows".into(), object: "trump".into(), notes: None },
        PAOEntry { number: "23".into(), person: "Nemo".into(), action: "daringly tapped".into(), object: "butt".into(), notes: None },
        PAOEntry { number: "24".into(), person: "Naruto".into(), action: "meditates".into(), object: "shuriken".into(), notes: None },
        PAOEntry { number: "25".into(), person: "Selina".into(), action: "uncovers".into(), object: "iPhone".into(), notes: None },
        PAOEntry { number: "26".into(), person: "Ninja".into(), action: "covers".into(), object: "camel spit".into(), notes: None },
        PAOEntry { number: "27".into(), person: "Mr. Incredible".into(), action: "lifts".into(), object: "train".into(), notes: None },
        PAOEntry { number: "28".into(), person: "Arnold Schwarzenegger".into(), action: "poses".into(), object: "mirror".into(), notes: None },
        PAOEntry { number: "29".into(), person: "Winnie the Pooh".into(), action: "pours".into(), object: "honey".into(), notes: None },
        PAOEntry { number: "30".into(), person: "Preacher Lawson".into(), action: "drops".into(), object: "mic".into(), notes: None },
        PAOEntry { number: "31".into(), person: "Minecraft Steve".into(), action: "mines".into(), object: "diamonds".into(), notes: None },
        PAOEntry { number: "32".into(), person: "Moana".into(), action: "combs".into(), object: "hair".into(), notes: None },
        PAOEntry { number: "33".into(), person: "M&M".into(), action: "rapping".into(), object: "candy".into(), notes: None },
        PAOEntry { number: "34".into(), person: "Mario".into(), action: "jump hit".into(), object: "coin box".into(), notes: None },
        PAOEntry { number: "35".into(), person: "Darth Maul".into(), action: "force pushes".into(), object: "metal crater".into(), notes: None },
        PAOEntry { number: "36".into(), person: "Magician (Clash of Clans)".into(), action: "shuffles".into(), object: "deck of cards".into(), notes: None },
        PAOEntry { number: "37".into(), person: "McQueen".into(), action: "flashes".into(), object: "headlights".into(), notes: None },
        PAOEntry { number: "38".into(), person: "Mufasa".into(), action: "shreds in two".into(), object: "stake".into(), notes: None },
        PAOEntry { number: "39".into(), person: "Simba".into(), action: "swallows".into(), object: "worms".into(), notes: None },
        PAOEntry { number: "40".into(), person: "Tarzan".into(), action: "swings".into(), object: "vines".into(), notes: None },
        PAOEntry { number: "41".into(), person: "Queen of Hearts".into(), action: "chops".into(), object: "head".into(), notes: None },
        PAOEntry { number: "42".into(), person: "Grunt".into(), action: "concentrates deeply".into(), object: "casino chips".into(), notes: None },
        PAOEntry { number: "43".into(), person: "Trump".into(), action: "builds".into(), object: "wall".into(), notes: None },
        PAOEntry { number: "44".into(), person: "Harry".into(), action: "gets hit".into(), object: "brick".into(), notes: None },
        PAOEntry { number: "45".into(), person: "Charmeleon".into(), action: "flamethrows".into(), object: "pokeball".into(), notes: None },
        PAOEntry { number: "46".into(), person: "Crush".into(), action: "head bumps".into(), object: "fish Dory".into(), notes: None },
        PAOEntry { number: "47".into(), person: "Patrick".into(), action: "latches".into(), object: "rock".into(), notes: None },
        PAOEntry { number: "48".into(), person: "Master Chief".into(), action: "reloads".into(), object: "pistol".into(), notes: None },
        PAOEntry { number: "49".into(), person: "Mr. Potato Head".into(), action: "rubs".into(), object: "mustache".into(), notes: None },
        PAOEntry { number: "50".into(), person: "Liz".into(), action: "jiggles".into(), object: "BS".into(), notes: None },
        PAOEntry { number: "51".into(), person: "Bolt".into(), action: "barks".into(), object: "watermelons".into(), notes: None },
        PAOEntry { number: "52".into(), person: "Link".into(), action: "draws".into(), object: "sword".into(), notes: None },
        PAOEntry { number: "53".into(), person: "Balmond".into(), action: "slams".into(), object: "turret".into(), notes: None },
        PAOEntry { number: "54".into(), person: "Larry the Cucumber".into(), action: "squashes".into(), object: "tomato".into(), notes: None },
        PAOEntry { number: "55".into(), person: "WALL-E".into(), action: "charges".into(), object: "battery".into(), notes: None },
        PAOEntry { number: "56".into(), person: "Michael Jackson".into(), action: "moonwalks".into(), object: "white gloves".into(), notes: None },
        PAOEntry { number: "57".into(), person: "Slinky Dog".into(), action: "climbs".into(), object: "bed".into(), notes: None },
        PAOEntry { number: "58".into(), person: "Alvin".into(), action: "workout".into(), object: "piano".into(), notes: None },
        PAOEntry { number: "59".into(), person: "Simon".into(), action: "rescues".into(), object: "hat".into(), notes: None },
        PAOEntry { number: "60".into(), person: "Jesse".into(), action: "jumps over".into(), object: "coach".into(), notes: None },
        PAOEntry { number: "61".into(), person: "Chewbacca".into(), action: "chest beating".into(), object: "shopping carts".into(), notes: None },
        PAOEntry { number: "62".into(), person: "Genie".into(), action: "shapeshifts".into(), object: "mansion".into(), notes: None },
        PAOEntry { number: "63".into(), person: "Conan O'Brien".into(), action: "dances".into(), object: "torch".into(), notes: None },
        PAOEntry { number: "64".into(), person: "Jerry".into(), action: "tiptoes".into(), object: "fur".into(), notes: None },
        PAOEntry { number: "65".into(), person: "Army Dude".into(), action: "tackles".into(), object: "tree".into(), notes: None },
        PAOEntry { number: "66".into(), person: "Chicken Joe".into(), action: "surfs".into(), object: "waves".into(), notes: None },
        PAOEntry { number: "67".into(), person: "Shrek".into(), action: "counts".into(), object: "sticks".into(), notes: None },
        PAOEntry { number: "68".into(), person: "Sheriff Woody".into(), action: "kicks".into(), object: "toy soldiers".into(), notes: None },
        PAOEntry { number: "69".into(), person: "SpongeBob".into(), action: "soaks".into(), object: "sponge".into(), notes: None },
        PAOEntry { number: "70".into(), person: "Russell Brand".into(), action: "flips off".into(), object: "TV".into(), notes: None },
        PAOEntry { number: "71".into(), person: "Megamind".into(), action: "urinates".into(), object: "raygun".into(), notes: None },
        PAOEntry { number: "72".into(), person: "Hulk".into(), action: "penetrates".into(), object: "steroids".into(), notes: None },
        PAOEntry { number: "73".into(), person: "Kermit".into(), action: "tongues".into(), object: "flies".into(), notes: None },
        PAOEntry { number: "74".into(), person: "Tigger".into(), action: "bounces".into(), object: "rubber ball".into(), notes: None },
        PAOEntry { number: "75".into(), person: "Golem".into(), action: "watering".into(), object: "flowers".into(), notes: None },
        PAOEntry { number: "76".into(), person: "Yoda".into(), action: "pokes".into(), object: "R2D2".into(), notes: None },
        PAOEntry { number: "77".into(), person: "Kakashi".into(), action: "reads".into(), object: "romantic novel".into(), notes: None },
        PAOEntry { number: "78".into(), person: "Goofy".into(), action: "waves".into(), object: "flag".into(), notes: None },
        PAOEntry { number: "79".into(), person: "Goblin".into(), action: "steals".into(), object: "elixir".into(), notes: None },
        PAOEntry { number: "80".into(), person: "General Grievous".into(), action: "combines".into(), object: "armor".into(), notes: None },
        PAOEntry { number: "81".into(), person: "Garfield".into(), action: "sleeps".into(), object: "lasagna".into(), notes: None },
        PAOEntry { number: "82".into(), person: "Kevin".into(), action: "slides".into(), object: "legs".into(), notes: None },
        PAOEntry { number: "83".into(), person: "Vileplume".into(), action: "poisons".into(), object: "carrots".into(), notes: None },
        PAOEntry { number: "84".into(), person: "Darth Vader".into(), action: "force pulls".into(), object: "lightsaber".into(), notes: None },
        PAOEntry { number: "85".into(), person: "Cruella de Ville".into(), action: "paints".into(), object: "dogs".into(), notes: None },
        PAOEntry { number: "86".into(), person: "Iron Man".into(), action: "shoots plasma beam".into(), object: "Thor hammer".into(), notes: None },
        PAOEntry { number: "87".into(), person: "Victreebel".into(), action: "whips".into(), object: "lighter".into(), notes: None },
        PAOEntry { number: "88".into(), person: "Brandon Birshard".into(), action: "writes".into(), object: "journal".into(), notes: None },
        PAOEntry { number: "89".into(), person: "Venom".into(), action: "shaves".into(), object: "weed".into(), notes: None },
        PAOEntry { number: "90".into(), person: "Bruce".into(), action: "cries out".into(), object: "bombs".into(), notes: None },
        PAOEntry { number: "91".into(), person: "Panda".into(), action: "cannonballs".into(), object: "mud puddle".into(), notes: None },
        PAOEntry { number: "92".into(), person: "Lego Batman".into(), action: "glides".into(), object: "batmobile".into(), notes: None },
        PAOEntry { number: "93".into(), person: "Spider-Man".into(), action: "smokes".into(), object: "cigar".into(), notes: None },
        PAOEntry { number: "94".into(), person: "Superman".into(), action: "flies up".into(), object: "moon".into(), notes: None },
        PAOEntry { number: "95".into(), person: "Buzz Lightyear".into(), action: "shoots lasers".into(), object: "ant".into(), notes: None },
        PAOEntry { number: "96".into(), person: "Pikachu".into(), action: "lightning bolts".into(), object: "water bottle".into(), notes: None },
        PAOEntry { number: "97".into(), person: "Pinocchio".into(), action: "grows".into(), object: "nose".into(), notes: None },
        PAOEntry { number: "98".into(), person: "Boba Fett".into(), action: "sneezes".into(), object: "helmet".into(), notes: None },
        PAOEntry { number: "99".into(), person: "Popeye".into(), action: "prise open".into(), object: "spinach".into(), notes: None },
    ]
}

/// Get all PAO indices with their associated logic and visual components
pub fn get_pao_system() -> Vec<PAOIndex> {
    let entries = get_pao_entries();
    let logic = get_logic_components();
    let visual = get_visual_elements();

    entries
        .into_iter()
        .enumerate()
        .map(|(idx, entry)| PAOIndex {
            index: idx,
            person: entry.person,
            action: entry.action,
            object: entry.object,
            logic_component: logic.get(idx).cloned().unwrap_or_else(|| LogicComponent {
                id: idx + 1,
                name: format!("Unknown Logic {}", idx + 1),
                category: "Unknown".into(),
                description: "Component not found".into(),
            }),
            visual_element: visual.get(idx).cloned().unwrap_or_else(|| VisualElement {
                id: idx + 1,
                name: format!("Unknown Visual {}", idx + 1),
                category: "Unknown".into(),
                description: "Component not found".into(),
            }),
        })
        .collect()
}

/// Get all 100 Logic Components
pub fn get_logic_components() -> Vec<LogicComponent> {
    vec![
        LogicComponent { id: 1, name: "Global State Manager".into(), category: "State & Data Flow".into(), description: "Centralized data store (Redux, Pinia)".into() },
        LogicComponent { id: 2, name: "Context Provider".into(), category: "State & Data Flow".into(), description: "Wraps components to share data without prop drilling".into() },
        LogicComponent { id: 3, name: "Data Fetcher".into(), category: "State & Data Flow".into(), description: "Handles API requests and lifecycle".into() },
        LogicComponent { id: 4, name: "Cache Controller".into(), category: "State & Data Flow".into(), description: "Manages local storage or session persistence".into() },
        LogicComponent { id: 5, name: "Mutation Observer".into(), category: "State & Data Flow".into(), description: "Watches for changes in the DOM".into() },
        LogicComponent { id: 6, name: "State Machine".into(), category: "State & Data Flow".into(), description: "Manages complex, mutually exclusive UI states".into() },
        LogicComponent { id: 7, name: "Hydrator".into(), category: "State & Data Flow".into(), description: "Re-activates server-rendered data on the client".into() },
        LogicComponent { id: 8, name: "Resource Pool".into(), category: "State & Data Flow".into(), description: "Manages a set of reusable objects (e.g., DB connections)".into() },
        LogicComponent { id: 9, name: "Dispatcher".into(), category: "State & Data Flow".into(), description: "Routes actions to their respective handlers".into() },
        LogicComponent { id: 10, name: "Computed Property".into(), category: "State & Data Flow".into(), description: "Derived data that updates based on dependencies".into() },
        LogicComponent { id: 11, name: "JWT Decoder".into(), category: "Auth & Security".into(), description: "Parses authentication tokens".into() },
        LogicComponent { id: 12, name: "Permissions Guard".into(), category: "Auth & Security".into(), description: "Checks user roles before rendering".into() },
        LogicComponent { id: 13, name: "CSRF Protector".into(), category: "Auth & Security".into(), description: "Logic to prevent cross-site request forgery".into() },
        LogicComponent { id: 14, name: "OAuth Handler".into(), category: "Auth & Security".into(), description: "Manages third-party login flows".into() },
        LogicComponent { id: 15, name: "Encryption Wrapper".into(), category: "Auth & Security".into(), description: "Encrypts data before local storage".into() },
        LogicComponent { id: 16, name: "Session Heartbeat".into(), category: "Auth & Security".into(), description: "Pings the server to keep a session alive".into() },
        LogicComponent { id: 17, name: "Input Sanitizer".into(), category: "Auth & Security".into(), description: "Strips malicious scripts from user input".into() },
        LogicComponent { id: 18, name: "Password Strength Validator".into(), category: "Auth & Security".into(), description: "Evaluates entropy in real-time".into() },
        LogicComponent { id: 19, name: "Two-Factor Authenticator (2FA)".into(), category: "Auth & Security".into(), description: "Logic for OTP verification".into() },
        LogicComponent { id: 20, name: "Biometric Interface".into(), category: "Auth & Security".into(), description: "Logic for FaceID/TouchID prompts".into() },
        LogicComponent { id: 21, name: "Retry Logic Handler".into(), category: "Networking & API".into(), description: "Automatically re-attempts failed requests".into() },
        LogicComponent { id: 22, name: "Rate Limiter (Client-side)".into(), category: "Networking & API".into(), description: "Prevents spamming the Submit button".into() },
        LogicComponent { id: 23, name: "WebSocket Listener".into(), category: "Networking & API".into(), description: "Subscribes to real-time data streams".into() },
        LogicComponent { id: 24, name: "GraphQL Client".into(), category: "Networking & API".into(), description: "Manages complex schema-based queries".into() },
        LogicComponent { id: 25, name: "Interceptor".into(), category: "Networking & API".into(), description: "Modifies headers (like Auth) for every outgoing request".into() },
        LogicComponent { id: 26, name: "Polling Service".into(), category: "Networking & API".into(), description: "Regularly checks for updates on a timer".into() },
        LogicComponent { id: 27, name: "Payload Mapper".into(), category: "Networking & API".into(), description: "Transforms API data into UI-friendly objects".into() },
        LogicComponent { id: 28, name: "Abort Controller".into(), category: "Networking & API".into(), description: "Cancels pending requests when a page changes".into() },
        LogicComponent { id: 29, name: "Service Worker".into(), category: "Networking & API".into(), description: "Logic for offline support and background sync".into() },
        LogicComponent { id: 30, name: "CDN Resolver".into(), category: "Networking & API".into(), description: "Logic to find the closest asset server".into() },
        LogicComponent { id: 31, name: "Debouncer".into(), category: "Utilities & Performance".into(), description: "Limits how often a function runs (e.g., search bars)".into() },
        LogicComponent { id: 32, name: "Throttler".into(), category: "Utilities & Performance".into(), description: "Ensures a function runs at most once in a window".into() },
        LogicComponent { id: 33, name: "Lazy Loader".into(), category: "Utilities & Performance".into(), description: "Logic to delay loading off-screen components".into() },
        LogicComponent { id: 34, name: "Error Boundary".into(), category: "Utilities & Performance".into(), description: "Catches crashes in the child tree to prevent full app failure".into() },
        LogicComponent { id: 35, name: "Logger".into(), category: "Utilities & Performance".into(), description: "Sends errors/analytics to a remote server".into() },
        LogicComponent { id: 36, name: "Environment Config".into(), category: "Utilities & Performance".into(), description: "Switches logic based on Dev/Prod mode".into() },
        LogicComponent { id: 37, name: "Internationalization (i18n) Engine".into(), category: "Utilities & Performance".into(), description: "Logic for language switching".into() },
        LogicComponent { id: 38, name: "Feature Flag Toggle".into(), category: "Utilities & Performance".into(), description: "Enables/disables features based on remote config".into() },
        LogicComponent { id: 39, name: "Dependency Injector".into(), category: "Utilities & Performance".into(), description: "Swaps out modules for testing or configuration".into() },
        LogicComponent { id: 40, name: "Memory Leak Detector".into(), category: "Utilities & Performance".into(), description: "Monitors unused event listeners".into() },
        LogicComponent { id: 41, name: "Event Bus".into(), category: "Event & Interaction Logic".into(), description: "Enables communication between unrelated components".into() },
        LogicComponent { id: 42, name: "Gesture Recognizer".into(), category: "Event & Interaction Logic".into(), description: "Translates swipes/pinches into actions".into() },
        LogicComponent { id: 43, name: "Focus Manager".into(), category: "Event & Interaction Logic".into(), description: "Controls Tab key order and accessibility focus".into() },
        LogicComponent { id: 44, name: "History Manager".into(), category: "Event & Interaction Logic".into(), description: "Manages the Back/Forward browser state".into() },
        LogicComponent { id: 45, name: "URL Parser".into(), category: "Event & Interaction Logic".into(), description: "Extracts parameters from the web address".into() },
        LogicComponent { id: 46, name: "Intersection Observer".into(), category: "Event & Interaction Logic".into(), description: "Triggers logic when an element is visible".into() },
        LogicComponent { id: 47, name: "Clipboard Controller".into(), category: "Event & Interaction Logic".into(), description: "Logic for Copy to Clipboard actions".into() },
        LogicComponent { id: 48, name: "File Uploader".into(), category: "Event & Interaction Logic".into(), description: "Logic for chunking large files for upload".into() },
        LogicComponent { id: 49, name: "Drag-and-Drop Handler".into(), category: "Event & Interaction Logic".into(), description: "Manages the transfer of data between zones".into() },
        LogicComponent { id: 50, name: "Idle Timer".into(), category: "Event & Interaction Logic".into(), description: "Triggers logic (like logout) after user inactivity".into() },
        LogicComponent { id: 51, name: "Currency Converter".into(), category: "Math & Calculation".into(), description: "Real-time exchange rate logic".into() },
        LogicComponent { id: 52, name: "Timezone Offsetter".into(), category: "Math & Calculation".into(), description: "Adjusts timestamps to local time".into() },
        LogicComponent { id: 53, name: "Physics Solver".into(), category: "Math & Calculation".into(), description: "Logic for bounce, gravity, or collisions in UI".into() },
        LogicComponent { id: 54, name: "Diff Engine".into(), category: "Math & Calculation".into(), description: "Compares two objects to find changes".into() },
        LogicComponent { id: 55, name: "Paginator".into(), category: "Math & Calculation".into(), description: "Calculates the number of pages from a list".into() },
        LogicComponent { id: 56, name: "Sort Engine".into(), category: "Math & Calculation".into(), description: "Logic for multi-criteria list sorting".into() },
        LogicComponent { id: 57, name: "Filter Predicate".into(), category: "Math & Calculation".into(), description: "Logic to include/exclude items in a view".into() },
        LogicComponent { id: 58, name: "Search Indexer".into(), category: "Math & Calculation".into(), description: "Creates a searchable map of local data".into() },
        LogicComponent { id: 59, name: "Distance Calculator".into(), category: "Math & Calculation".into(), description: "Logic for geospatial coordinates".into() },
        LogicComponent { id: 60, name: "Statistical Aggregator".into(), category: "Math & Calculation".into(), description: "Logic for sums, averages, and medians".into() },
        LogicComponent { id: 61, name: "IndexedDB Wrapper".into(), category: "Database & Persistence".into(), description: "Logic for browser-based databases".into() },
        LogicComponent { id: 62, name: "Local Storage Bridge".into(), category: "Database & Persistence".into(), description: "Syncs state to the browser's key-value store".into() },
        LogicComponent { id: 63, name: "Schema Validator".into(), category: "Database & Persistence".into(), description: "Checks data against a JSON schema".into() },
        LogicComponent { id: 64, name: "Migration Script".into(), category: "Database & Persistence".into(), description: "Updates local data structures between versions".into() },
        LogicComponent { id: 65, name: "Conflict Resolver".into(), category: "Database & Persistence".into(), description: "Logic for merging local and remote data".into() },
        LogicComponent { id: 66, name: "Optimistic UI Updater".into(), category: "Database & Persistence".into(), description: "Updates the UI before the server confirms".into() },
        LogicComponent { id: 67, name: "Garbage Collector".into(), category: "Database & Persistence".into(), description: "Logic to prune old cache entries".into() },
        LogicComponent { id: 68, name: "Cookie Manager".into(), category: "Database & Persistence".into(), description: "Logic for setting/getting browser cookies".into() },
        LogicComponent { id: 69, name: "Query Builder".into(), category: "Database & Persistence".into(), description: "Logic to programmatically create complex filters".into() },
        LogicComponent { id: 70, name: "Normalization Logic".into(), category: "Database & Persistence".into(), description: "Flattens nested data for easier state management".into() },
        LogicComponent { id: 71, name: "Video Stream Buffer".into(), category: "Media Logic".into(), description: "Manages the playback buffer".into() },
        LogicComponent { id: 72, name: "Audio Visualizer Logic".into(), category: "Media Logic".into(), description: "Converts frequencies into numerical arrays".into() },
        LogicComponent { id: 73, name: "Image Metadata Extractor".into(), category: "Media Logic".into(), description: "Reads EXIF data from photos".into() },
        LogicComponent { id: 74, name: "Playback Controller".into(), category: "Media Logic".into(), description: "Logic for play, pause, and seek".into() },
        LogicComponent { id: 75, name: "Subtitle Sync".into(), category: "Media Logic".into(), description: "Matches text timestamps to video time".into() },
        LogicComponent { id: 76, name: "Canvas Renderer".into(), category: "Media Logic".into(), description: "Logic for drawing pixels to a 2D/3D plane".into() },
        LogicComponent { id: 77, name: "Vibrator Logic".into(), category: "Media Logic".into(), description: "Triggers haptic feedback on mobile".into() },
        LogicComponent { id: 78, name: "Camera Stream Controller".into(), category: "Media Logic".into(), description: "Manages webcam permissions and feed".into() },
        LogicComponent { id: 79, name: "Microphone Analyzer".into(), category: "Media Logic".into(), description: "Logic for volume levels and noise gates".into() },
        LogicComponent { id: 80, name: "Orientation Handler".into(), category: "Media Logic".into(), description: "Logic for landscape vs. portrait changes".into() },
        LogicComponent { id: 81, name: "Cron Job Emulator".into(), category: "Automated Logic".into(), description: "Runs logic at specific intervals".into() },
        LogicComponent { id: 82, name: "Workflow Engine".into(), category: "Automated Logic".into(), description: "Chains multiple logic steps into a sequence".into() },
        LogicComponent { id: 83, name: "A/B Testing Randomizer".into(), category: "Automated Logic".into(), description: "Logic to assign users to groups".into() },
        LogicComponent { id: 84, name: "Telemetry Dispatcher".into(), category: "Automated Logic".into(), description: "Packages user behavior data for analysis".into() },
        LogicComponent { id: 85, name: "Notification Scheduler".into(), category: "Automated Logic".into(), description: "Logic for Remind me later pings".into() },
        LogicComponent { id: 86, name: "Auto-Save Logic".into(), category: "Automated Logic".into(), description: "Periodically persists drafts".into() },
        LogicComponent { id: 87, name: "Theme Switcher Logic".into(), category: "Automated Logic".into(), description: "Logic to calculate dark/light mode styles".into() },
        LogicComponent { id: 88, name: "Deep Link Handler".into(), category: "Automated Logic".into(), description: "Logic to open specific app areas from a link".into() },
        LogicComponent { id: 89, name: "Breadcrumb Generator".into(), category: "Automated Logic".into(), description: "Logic to calculate the current navigation path".into() },
        LogicComponent { id: 90, name: "Form Serializer".into(), category: "Automated Logic".into(), description: "Converts complex forms into JSON".into() },
        LogicComponent { id: 91, name: "Plugin Loader".into(), category: "Architecture & Meta".into(), description: "Logic to add third-party modules dynamically".into() },
        LogicComponent { id: 92, name: "HOC (Higher Order Component)".into(), category: "Architecture & Meta".into(), description: "Logic that wraps and enhances other components".into() },
        LogicComponent { id: 93, name: "Render Prop".into(), category: "Architecture & Meta".into(), description: "Component that takes a function as its children".into() },
        LogicComponent { id: 94, name: "Bridge/Adapter".into(), category: "Architecture & Meta".into(), description: "Logic that translates between two incompatible APIs".into() },
        LogicComponent { id: 95, name: "Facade".into(), category: "Architecture & Meta".into(), description: "Simplifies a complex underlying logic system".into() },
        LogicComponent { id: 96, name: "Decorator".into(), category: "Architecture & Meta".into(), description: "Modifies the behavior of a function or class".into() },
        LogicComponent { id: 97, name: "Singleton Manager".into(), category: "Architecture & Meta".into(), description: "Ensures only one instance of a service exists".into() },
        LogicComponent { id: 98, name: "Factory".into(), category: "Architecture & Meta".into(), description: "Logic that decides which component to create at runtime".into() },
        LogicComponent { id: 99, name: "Proxy".into(), category: "Architecture & Meta".into(), description: "Intercepts and redefines fundamental operations for an object".into() },
        LogicComponent { id: 100, name: "Kernel".into(), category: "Architecture & Meta".into(), description: "The core logic loop that initializes the entire app".into() },
    ]
}

/// Get all 100 Visual Elements
pub fn get_visual_elements() -> Vec<VisualElement> {
    vec![
        VisualElement { id: 1, name: "Text Input".into(), category: "Input & Controls".into(), description: "Standard single-line field".into() },
        VisualElement { id: 2, name: "Text Area".into(), category: "Input & Controls".into(), description: "Multi-line input for long-form content".into() },
        VisualElement { id: 3, name: "Password Toggle".into(), category: "Input & Controls".into(), description: "Input with a Show/Hide eyeball icon".into() },
        VisualElement { id: 4, name: "Checkbox".into(), category: "Input & Controls".into(), description: "Binary selection".into() },
        VisualElement { id: 5, name: "Radio Button".into(), category: "Input & Controls".into(), description: "Single selection from a group".into() },
        VisualElement { id: 6, name: "Switch/Toggle".into(), category: "Input & Controls".into(), description: "On/Off visual slide".into() },
        VisualElement { id: 7, name: "Select Dropdown".into(), category: "Input & Controls".into(), description: "Collapsible list of options".into() },
        VisualElement { id: 8, name: "Multi-Select Tag Cloud".into(), category: "Input & Controls".into(), description: "Dropdown where choices become tags".into() },
        VisualElement { id: 9, name: "Slider/Range Picker".into(), category: "Input & Controls".into(), description: "Visual track for numeric values".into() },
        VisualElement { id: 10, name: "Date Picker".into(), category: "Input & Controls".into(), description: "Calendar-style input".into() },
        VisualElement { id: 11, name: "Time Picker".into(), category: "Input & Controls".into(), description: "Clock-style or list-based time selector".into() },
        VisualElement { id: 12, name: "File Dropzone".into(), category: "Input & Controls".into(), description: "Visual area to drag and drop files".into() },
        VisualElement { id: 13, name: "Search Bar".into(), category: "Input & Controls".into(), description: "Input field with a magnifying glass icon".into() },
        VisualElement { id: 14, name: "Color Picker".into(), category: "Input & Controls".into(), description: "Visual palette or hex input".into() },
        VisualElement { id: 15, name: "Rating (Stars)".into(), category: "Input & Controls".into(), description: "Interactive visual for feedback".into() },
        VisualElement { id: 16, name: "Segmented Control".into(), category: "Input & Controls".into(), description: "Buttons grouped together (like iOS tabs)".into() },
        VisualElement { id: 17, name: "Step Input".into(), category: "Input & Controls".into(), description: "Plus/Minus buttons for numbers".into() },
        VisualElement { id: 18, name: "Autocomplete Suggestions".into(), category: "Input & Controls".into(), description: "Visual list that appears as you type".into() },
        VisualElement { id: 19, name: "Rich Text Editor".into(), category: "Input & Controls".into(), description: "Visual toolbar for bold/italic/links".into() },
        VisualElement { id: 20, name: "Pin/OTP Input".into(), category: "Input & Controls".into(), description: "Separate boxes for single digits".into() },
        VisualElement { id: 21, name: "Primary Action Button".into(), category: "Buttons & Actions".into(), description: "High-contrast call to action".into() },
        VisualElement { id: 22, name: "Ghost Button".into(), category: "Buttons & Actions".into(), description: "Outlined button for secondary actions".into() },
        VisualElement { id: 23, name: "Floating Action Button (FAB)".into(), category: "Buttons & Actions".into(), description: "Circular button floating over UI".into() },
        VisualElement { id: 24, name: "Icon Button".into(), category: "Buttons & Actions".into(), description: "Button with no text, just a glyph".into() },
        VisualElement { id: 25, name: "Split Button".into(), category: "Buttons & Actions".into(), description: "Main action + dropdown for related actions".into() },
        VisualElement { id: 26, name: "Loading Button".into(), category: "Buttons & Actions".into(), description: "Button that shows a spinner when clicked".into() },
        VisualElement { id: 27, name: "Breadcrumb".into(), category: "Buttons & Actions".into(), description: "Sequential links showing site hierarchy".into() },
        VisualElement { id: 28, name: "Pagination Bar".into(), category: "Buttons & Actions".into(), description: "Numbered buttons for list navigation".into() },
        VisualElement { id: 29, name: "Hamburger Menu".into(), category: "Buttons & Actions".into(), description: "Triple-bar icon for mobile nav".into() },
        VisualElement { id: 30, name: "Context Menu".into(), category: "Buttons & Actions".into(), description: "Popup menu on right-click".into() },
        VisualElement { id: 31, name: "Navbar".into(), category: "Navigation Elements".into(), description: "Top horizontal menu".into() },
        VisualElement { id: 32, name: "Sidebar".into(), category: "Navigation Elements".into(), description: "Vertical navigation drawer".into() },
        VisualElement { id: 33, name: "Tab Bar".into(), category: "Navigation Elements".into(), description: "Bottom navigation for mobile apps".into() },
        VisualElement { id: 34, name: "Accordion".into(), category: "Navigation Elements".into(), description: "Vertically stacked headers that expand".into() },
        VisualElement { id: 35, name: "Stepper".into(), category: "Navigation Elements".into(), description: "Visual progress through a multi-step form".into() },
        VisualElement { id: 36, name: "TreeView".into(), category: "Navigation Elements".into(), description: "Nested list for folder hierarchies".into() },
        VisualElement { id: 37, name: "Anchor Links".into(), category: "Navigation Elements".into(), description: "Visual Back to Top or section jumpers".into() },
        VisualElement { id: 38, name: "Mega Menu".into(), category: "Navigation Elements".into(), description: "Large dropdown with categories and images".into() },
        VisualElement { id: 39, name: "Dot Navigation".into(), category: "Navigation Elements".into(), description: "Visual indicators for carousels".into() },
        VisualElement { id: 40, name: "Breadcrumb Trail".into(), category: "Navigation Elements".into(), description: "History of navigation steps".into() },
        VisualElement { id: 41, name: "Card".into(), category: "Data Visualization & Display".into(), description: "Container for grouped information (image, title, text)".into() },
        VisualElement { id: 42, name: "Data Table".into(), category: "Data Visualization & Display".into(), description: "Grid for structured datasets".into() },
        VisualElement { id: 43, name: "Badge/Pill".into(), category: "Data Visualization & Display".into(), description: "Small visual indicator for counts or status".into() },
        VisualElement { id: 44, name: "Tooltip".into(), category: "Data Visualization & Display".into(), description: "Small popup on hover".into() },
        VisualElement { id: 45, name: "Progress Bar".into(), category: "Data Visualization & Display".into(), description: "Linear visual for completion percentage".into() },
        VisualElement { id: 46, name: "Circular Progress/Spinner".into(), category: "Data Visualization & Display".into(), description: "Radial loading indicator".into() },
        VisualElement { id: 47, name: "Avatar".into(), category: "Data Visualization & Display".into(), description: "Circular user profile image".into() },
        VisualElement { id: 48, name: "Statistic/Metric Card".into(), category: "Data Visualization & Display".into(), description: "Large number with a trend arrow".into() },
        VisualElement { id: 49, name: "Tag/Chip".into(), category: "Data Visualization & Display".into(), description: "Compact element for categories or filters".into() },
        VisualElement { id: 50, name: "Timeline".into(), category: "Data Visualization & Display".into(), description: "Vertical or horizontal line for chronological events".into() },
        VisualElement { id: 51, name: "Calendar View".into(), category: "Data Visualization & Display".into(), description: "Grid display for monthly events".into() },
        VisualElement { id: 52, name: "Kanban Board".into(), category: "Data Visualization & Display".into(), description: "Columns and cards for project management".into() },
        VisualElement { id: 53, name: "Gantt Chart".into(), category: "Data Visualization & Display".into(), description: "Visual timeline for project schedules".into() },
        VisualElement { id: 54, name: "Donut Chart".into(), category: "Data Visualization & Display".into(), description: "Radial data visualization".into() },
        VisualElement { id: 55, name: "Line Graph".into(), category: "Data Visualization & Display".into(), description: "Visualizing trends over time".into() },
        VisualElement { id: 56, name: "Bar Chart".into(), category: "Data Visualization & Display".into(), description: "Comparing categorical data".into() },
        VisualElement { id: 57, name: "Heatmap".into(), category: "Data Visualization & Display".into(), description: "Grid using color to represent intensity".into() },
        VisualElement { id: 58, name: "Skeleton Loader".into(), category: "Data Visualization & Display".into(), description: "Gray placeholders that mimic content".into() },
        VisualElement { id: 59, name: "Empty State Illustration".into(), category: "Data Visualization & Display".into(), description: "Visual shown when no data exists".into() },
        VisualElement { id: 60, name: "Tree Map".into(), category: "Data Visualization & Display".into(), description: "Nested rectangles showing data proportions".into() },
        VisualElement { id: 61, name: "Modal/Dialog".into(), category: "Overlays & Feedback".into(), description: "Popup that dim-blocks the background".into() },
        VisualElement { id: 62, name: "Toast/Snackbar".into(), category: "Overlays & Feedback".into(), description: "Temporary message at the bottom/top".into() },
        VisualElement { id: 63, name: "Alert Banner".into(), category: "Overlays & Feedback".into(), description: "Permanent visual warning (Top of page)".into() },
        VisualElement { id: 64, name: "Popover".into(), category: "Overlays & Feedback".into(), description: "Interactive overlay attached to an element".into() },
        VisualElement { id: 65, name: "Bottom Sheet".into(), category: "Overlays & Feedback".into(), description: "Mobile panel that slides up from the base".into() },
        VisualElement { id: 66, name: "Lightbox".into(), category: "Overlays & Feedback".into(), description: "Full-screen image/video viewer".into() },
        VisualElement { id: 67, name: "Confetti Effect".into(), category: "Overlays & Feedback".into(), description: "Visual celebration on success".into() },
        VisualElement { id: 68, name: "Backdrop/Scrub".into(), category: "Overlays & Feedback".into(), description: "The darkened area behind a modal".into() },
        VisualElement { id: 69, name: "Drawer".into(), category: "Overlays & Feedback".into(), description: "Side panel that pushes content or overlays it".into() },
        VisualElement { id: 70, name: "Guided Tour/Coachmarks".into(), category: "Overlays & Feedback".into(), description: "Visual highlights for new users".into() },
        VisualElement { id: 71, name: "Image Gallery/Grid".into(), category: "Media & Content".into(), description: "Layout for multiple photos".into() },
        VisualElement { id: 72, name: "Carousel/Slider".into(), category: "Media & Content".into(), description: "Rotates through featured content".into() },
        VisualElement { id: 73, name: "Video Player".into(), category: "Media & Content".into(), description: "UI with progress bar and volume controls".into() },
        VisualElement { id: 74, name: "Audio Player".into(), category: "Media & Content".into(), description: "Waveform or seek-bar for sound files".into() },
        VisualElement { id: 75, name: "Hero Section".into(), category: "Media & Content".into(), description: "Large visual at the top of a landing page".into() },
        VisualElement { id: 76, name: "Parallax Background".into(), category: "Media & Content".into(), description: "Image that moves slower than the scroll".into() },
        VisualElement { id: 77, name: "Icon Library".into(), category: "Media & Content".into(), description: "Set of consistent glyphs (Home, Gear, etc.)".into() },
        VisualElement { id: 78, name: "Lottie Animation".into(), category: "Media & Content".into(), description: "High-quality vector animation container".into() },
        VisualElement { id: 79, name: "Markdown Previewer".into(), category: "Media & Content".into(), description: "Visual side-by-side for text formatting".into() },
        VisualElement { id: 80, name: "PDF Viewer".into(), category: "Media & Content".into(), description: "In-browser document preview".into() },
        VisualElement { id: 81, name: "Grid System".into(), category: "Containers & Layout".into(), description: "Responsive columns and rows".into() },
        VisualElement { id: 82, name: "Flexbox Wrapper".into(), category: "Containers & Layout".into(), description: "Layout container for alignment".into() },
        VisualElement { id: 83, name: "Divider/Separator".into(), category: "Containers & Layout".into(), description: "Horizontal or vertical line".into() },
        VisualElement { id: 84, name: "Scroll Container".into(), category: "Containers & Layout".into(), description: "Area with custom-styled scrollbars".into() },
        VisualElement { id: 85, name: "Aspect Ratio Box".into(), category: "Containers & Layout".into(), description: "Keeps content at 16:9 or 1:1".into() },
        VisualElement { id: 86, name: "Collapsible Panel".into(), category: "Containers & Layout".into(), description: "Section that hides/shows details".into() },
        VisualElement { id: 87, name: "Sticky Header".into(), category: "Containers & Layout".into(), description: "Stays at the top while scrolling".into() },
        VisualElement { id: 88, name: "Footer".into(), category: "Containers & Layout".into(), description: "Site info at the very bottom".into() },
        VisualElement { id: 89, name: "Responsive Image".into(), category: "Containers & Layout".into(), description: "Swaps sources based on screen size".into() },
        VisualElement { id: 90, name: "Masonry Layout".into(), category: "Containers & Layout".into(), description: "Staggered column layout (like Pinterest)".into() },
        VisualElement { id: 91, name: "QR Code Generator".into(), category: "Special Elements".into(), description: "Visual code for mobile scanning".into() },
        VisualElement { id: 92, name: "Captcha Widget".into(), category: "Special Elements".into(), description: "Security visual to prove human status".into() },
        VisualElement { id: 93, name: "Scroll Indicator".into(), category: "Special Elements".into(), description: "Bar showing how far you've scrolled".into() },
        VisualElement { id: 94, name: "Diff Viewer".into(), category: "Special Elements".into(), description: "Visual highlighting of text changes".into() },
        VisualElement { id: 95, name: "Code Snippet Wrapper".into(), category: "Special Elements".into(), description: "Visual box with syntax highlighting".into() },
        VisualElement { id: 96, name: "Price Toggle".into(), category: "Special Elements".into(), description: "Yearly vs. Monthly pricing visuals".into() },
        VisualElement { id: 97, name: "Coupon/Promo Box".into(), category: "Special Elements".into(), description: "Visual field for discount codes".into() },
        VisualElement { id: 98, name: "Testimonial Slider".into(), category: "Special Elements".into(), description: "Visual quotes from users".into() },
        VisualElement { id: 99, name: "Comparison Table".into(), category: "Special Elements".into(), description: "Side-by-side feature lists".into() },
        VisualElement { id: 100, name: "Dashboard Widget".into(), category: "Special Elements".into(), description: "Small, modular window of data".into() },
    ]
}

/// Get all unique categories from logic components
pub fn get_logic_categories() -> Vec<String> {
    let mut categories: Vec<String> = get_logic_components()
        .into_iter()
        .map(|c| c.category)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    categories.sort();
    categories
}

/// Get all unique categories from visual elements
pub fn get_visual_categories() -> Vec<String> {
    let mut categories: Vec<String> = get_visual_elements()
        .into_iter()
        .map(|c| c.category)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    categories.sort();
    categories
}

/// Get all unique categories (combined)
pub fn get_all_categories() -> Vec<String> {
    let mut categories: Vec<String> = get_logic_components()
        .into_iter()
        .chain(get_visual_elements().into_iter())
        .map(|c| c.category)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    categories.sort();
    categories
}
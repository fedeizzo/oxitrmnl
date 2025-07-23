CREATE TABLE IF NOT EXISTS devices (
id INTEGER PRIMARY KEY AUTOINCREMENT,
name TEXT,
mac_address TEXT UNIQUE NOT NULL,
default_refresh_interval INTEGER DEFAULT 900,
friendly_id TEXT,
api_key TEXT,
last_rssi_level INTEGER,
last_battery_voltage REAL,
last_firmware_version TEXT,
current_screen_image TEXT,
user_id INTEGER,
created_at TEXT DEFAULT CURRENT_TIMESTAMP,
updated_at TEXT DEFAULT CURRENT_TIMESTAMP
) ;

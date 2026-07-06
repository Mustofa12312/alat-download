-- 1. settings
CREATE TABLE IF NOT EXISTS settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT NOT NULL UNIQUE,
    value TEXT NOT NULL,
    category TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 2. downloads
CREATE TABLE IF NOT EXISTS downloads (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    url TEXT NOT NULL,
    file_name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    total_size INTEGER DEFAULT 0,
    downloaded_size INTEGER DEFAULT 0,
    download_speed REAL DEFAULT 0.0,
    status INTEGER NOT NULL DEFAULT 0,
    priority INTEGER DEFAULT 1,
    category TEXT,
    checksum TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 3. download_segments
CREATE TABLE IF NOT EXISTS download_segments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    download_id INTEGER NOT NULL,
    segment_index INTEGER NOT NULL,
    start_byte INTEGER NOT NULL,
    end_byte INTEGER NOT NULL,
    downloaded_bytes INTEGER DEFAULT 0,
    status INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY(download_id) REFERENCES downloads(id) ON DELETE CASCADE
);
CREATE INDEX idx_segments_download_id ON download_segments(download_id);

-- 4. queue
CREATE TABLE IF NOT EXISTS queue (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    download_id INTEGER NOT NULL,
    queue_order INTEGER NOT NULL,
    priority INTEGER DEFAULT 1,
    auto_start BOOLEAN DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(download_id) REFERENCES downloads(id) ON DELETE CASCADE
);

-- 5. history
CREATE TABLE IF NOT EXISTS history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    download_id INTEGER NOT NULL,
    status INTEGER NOT NULL,
    completed_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    total_time INTEGER DEFAULT 0,
    average_speed REAL DEFAULT 0.0,
    file_size INTEGER DEFAULT 0,
    FOREIGN KEY(download_id) REFERENCES downloads(id) ON DELETE SET NULL
);

-- 6. scheduler
CREATE TABLE IF NOT EXISTS scheduler (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_name TEXT NOT NULL,
    trigger_type TEXT NOT NULL,
    scheduled_time DATETIME NOT NULL,
    action TEXT NOT NULL,
    enabled BOOLEAN DEFAULT 1
);

-- 7. notifications
CREATE TABLE IF NOT EXISTS notifications (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    type TEXT NOT NULL,
    priority INTEGER DEFAULT 1,
    is_read BOOLEAN DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 8. statistics
CREATE TABLE IF NOT EXISTS statistics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    total_downloads INTEGER DEFAULT 0,
    completed_downloads INTEGER DEFAULT 0,
    failed_downloads INTEGER DEFAULT 0,
    total_download_size INTEGER DEFAULT 0,
    total_download_time INTEGER DEFAULT 0,
    average_speed REAL DEFAULT 0.0,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 9. logs
CREATE TABLE IF NOT EXISTS logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    level TEXT NOT NULL,
    module TEXT NOT NULL,
    event TEXT NOT NULL,
    message TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 10. recovery
CREATE TABLE IF NOT EXISTS recovery (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    download_id INTEGER NOT NULL,
    resume_data BLOB,
    recovery_status INTEGER DEFAULT 0,
    last_position INTEGER DEFAULT 0,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(download_id) REFERENCES downloads(id) ON DELETE CASCADE
);

-- 11. torrent
CREATE TABLE IF NOT EXISTS torrent (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    download_id INTEGER NOT NULL,
    magnet_link TEXT,
    torrent_name TEXT,
    hash TEXT,
    tracker TEXT,
    peers INTEGER DEFAULT 0,
    seeds INTEGER DEFAULT 0,
    progress REAL DEFAULT 0.0,
    FOREIGN KEY(download_id) REFERENCES downloads(id) ON DELETE CASCADE
);

-- 12. video_metadata
CREATE TABLE IF NOT EXISTS video_metadata (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    download_id INTEGER NOT NULL,
    title TEXT,
    uploader TEXT,
    duration INTEGER,
    resolution TEXT,
    format TEXT,
    subtitle TEXT,
    thumbnail TEXT,
    upload_date TEXT,
    FOREIGN KEY(download_id) REFERENCES downloads(id) ON DELETE CASCADE
);

CREATE TABLE camera_calibrations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    camera_id TEXT NOT NULL,              -- Unique identifier for the camera
    calibration_date TEXT,                -- ISO 8601 date (e.g., "2025-04-20T13:36:00")
    fx REAL NOT NULL,                     -- Focal length x (pixels)
    fy REAL NOT NULL,                     -- Focal length y (pixels)
    cx REAL NOT NULL,                     -- Principal point x (pixels)
    cy REAL NOT NULL,                     -- Principal point y (pixels)
    k1 REAL,                              -- Radial distortion coefficient 1
    k2 REAL,                              -- Radial distortion coefficient 2
    p1 REAL,                              -- Tangential distortion coefficient 1
    p2 REAL,                              -- Tangential distortion coefficient 2
    k3 REAL,                              -- Radial distortion coefficient 3 (optional)
    rotation TEXT NOT NULL,               -- 3x3 rotation matrix (e.g., JSON or CSV)
    translation TEXT NOT NULL,            -- 3D translation vector (e.g., JSON or CSV)
    UNIQUE(camera_id, calibration_date)    -- Ensure unique camera/date pairs
);


-- Alternatively, if you prefer to store rotation and translation as separate columns:
--
-- CREATE TABLE camera_calibrations (
--     id INTEGER PRIMARY KEY AUTOINCREMENT,
--     camera_id TEXT NOT NULL,
--     calibration_date TEXT,
--     fx REAL NOT NULL,
--     fy REAL NOT NULL,
--     cx REAL NOT NULL,
--     cy REAL NOT NULL,
--     k1 REAL,
--     k2 REAL,
--     p1 REAL,
--     p2 REAL,
--     k3 REAL,
--     r11 REAL NOT NULL, -- Rotation matrix elements
--     r12 REAL NOT NULL,
--     r13 REAL NOT NULL,
--     r21 REAL NOT NULL,
--     r22 REAL NOT NULL,
--     r23 REAL NOT NULL,
--     r31 REAL NOT NULL,
--     r32 REAL NOT NULL,
--     r33 REAL NOT NULL,
--     tx REAL NOT NULL,  -- Translation vector elements
--     ty REAL NOT NULL,
--     tz REAL NOT NULL,
--     UNIQUE(camera_id, calibration_date)
-- );
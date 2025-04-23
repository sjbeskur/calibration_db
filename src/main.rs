use rusqlite::{Connection, Result};
use std::string::ToString;
use  rustdb_poc::CameraCalibration;

fn main() -> Result<()> {
    // Open or create a database file
    let conn = Connection::open("calibrations.db")?;

    // Create the table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS camera_calibrations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            camera_id TEXT NOT NULL,
            calibration_date TEXT,
            fx REAL NOT NULL,
            fy REAL NOT NULL,
            cx REAL NOT NULL,
            cy REAL NOT NULL,
            k1 REAL,
            k2 REAL,
            p1 REAL,
            p2 REAL,
            k3 REAL,
            rotation TEXT NOT NULL,
            translation TEXT NOT NULL,
            UNIQUE(camera_id, calibration_date)
        )",
        [],
    )?;

    // Sample calibration data
    let calibration = CameraCalibration {
        camera_id: "cam2".to_string(),
        calibration_date: "2025-04-20T13:36:00".to_string(),
        fx: 800.0,
        fy: 800.0,
        cx: 640.0,
        cy: 480.0,
        k1: Some(-0.1),
        k2: Some(0.01),
        p1: Some(0.0),
        p2: Some(0.0),
        k3: None,
        rotation: "1,0,0,0,1,0,0,0,1".to_string(), // Identity matrix
        translation: "0,0,0".to_string(), // No translation
    };

    // Insert data
    conn.execute(
        "INSERT INTO camera_calibrations (  
                    camera_id, calibration_date,          
                    fx,                        
                    fy,                           
                    cx,                        
                    cy,                        
                    k1,                        
                    k2,                        
                    p1,                        
                    p2,                        
                    k3,                           
                    rotation,                  
                    translation                
        ) VALUES (:camera_id, :calibration_date, :fx, :fy, :cx, :cy, :k1, :k2, :p1, :p2, :k3, :rotation, :translation)",
        rusqlite::named_params! {
            ":camera_id": &calibration.camera_id,
            ":calibration_date": &calibration.calibration_date,
            ":fx": &calibration.fx,
            ":fy": &calibration.fy,
            ":cx": &calibration.cx,
            ":cy": &calibration.cy,
            ":k1": &calibration.k1,
            ":k2": &calibration.k2,
            ":p1": &calibration.p1,
            ":p2": &calibration.p2,
            ":k3": &calibration.k3,
            ":rotation": &calibration.rotation,
            ":translation": &calibration.translation,
        },
    )?;

    // Read data back
    let mut stmt = conn.prepare("SELECT camera_id, calibration_date, fx, fy, cx, cy, k1, k2, p1, p2, k3, rotation, translation FROM camera_calibrations")?;
    let calibrations = stmt.query_map([], |row| {
        Ok(CameraCalibration {
            camera_id: row.get(0)?,
            calibration_date: row.get(1)?,
            fx: row.get(2)?,
            fy: row.get(3)?,
            cx: row.get(4)?,
            cy: row.get(5)?,
            k1: row.get(6)?,
            k2: row.get(7)?,
            p1: row.get(8)?,
            p2: row.get(9)?,
            k3: row.get(10)?,
            rotation: row.get(11)?,
            translation: row.get(12)?,
        })
    })?;

    for calibration in calibrations {
        let cal = calibration?;
        println!("{:?}", cal);
    }

    Ok(())
}
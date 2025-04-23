
// Struct to represent calibration data
#[derive(Debug)]
pub struct CameraCalibration {
    pub camera_id: String,
    pub calibration_date: String,
    pub fx: f64,
    pub fy: f64,
    pub cx: f64,
    pub cy: f64,
    pub k1: Option<f64>,
    pub k2: Option<f64>,
    pub p1: Option<f64>,
    pub p2: Option<f64>,
    pub k3: Option<f64>,
    pub rotation: String, // CSV: "r11,r12,r13,r21,r22,r23,r31,r32,r33"  // perhaps this should be json?
    pub translation: String, // CSV: "tx,ty,tz" // perhaps this should be json?
}
/// FIT message builders for workout recording.
///
/// Each function writes a complete definition + data message pair
/// to the encoder. Messages follow the FIT Activity file structure.

use super::encoder::FitEncoder;
use super::types::*;

/// File ID message — identifies this as an activity file.
/// Local message type: 0
pub fn write_file_id(encoder: &mut FitEncoder, timestamp: u32, serial: u32) {
    let fields = vec![
        FieldDef::new(0, 1, BASE_TYPE_ENUM),    // type
        FieldDef::new(1, 2, BASE_TYPE_UINT16),   // manufacturer
        FieldDef::new(2, 2, BASE_TYPE_UINT16),   // product
        FieldDef::new(3, 4, BASE_TYPE_UINT32),   // serial_number
        FieldDef::new(4, 4, BASE_TYPE_UINT32),   // time_created
    ];

    encoder.write_message_with_def(
        0,
        MESG_FILE_ID,
        &fields,
        &[
            &[FILE_TYPE_ACTIVITY],
            &MANUFACTURER_DEVELOPMENT.to_le_bytes(),
            &PRODUCT_NOTCHRIDER.to_le_bytes(),
            &serial.to_le_bytes(),
            &timestamp.to_le_bytes(),
        ],
    );
}

/// Event message — timer start or stop.
/// Local message type: 1
pub fn write_event(encoder: &mut FitEncoder, timestamp: u32, event_type: u8) {
    let fields = vec![
        FieldDef::new(253, 4, BASE_TYPE_UINT32),  // timestamp
        FieldDef::new(0, 1, BASE_TYPE_ENUM),      // event
        FieldDef::new(1, 1, BASE_TYPE_ENUM),      // event_type
    ];

    encoder.write_message_with_def(
        1,
        MESG_EVENT,
        &fields,
        &[
            &timestamp.to_le_bytes(),
            &[EVENT_TIMER],
            &[event_type],
        ],
    );
}

/// Record message — per-second data point.
/// Local message type: 2
pub fn write_record(
    encoder: &mut FitEncoder,
    timestamp: u32,
    power: u16,
    heart_rate: u8,
    cadence: u8,
    speed: u16,      // m/s * 1000
    distance: u32,   // meters * 100
    first: bool,     // if true, write definition first
) {
    if first {
        let fields = vec![
            FieldDef::new(253, 4, BASE_TYPE_UINT32),  // timestamp
            FieldDef::new(7, 2, BASE_TYPE_UINT16),    // power
            FieldDef::new(3, 1, BASE_TYPE_UINT8),     // heart_rate
            FieldDef::new(4, 1, BASE_TYPE_UINT8),     // cadence
            FieldDef::new(6, 2, BASE_TYPE_UINT16),    // speed (m/s * 1000)
            FieldDef::new(5, 4, BASE_TYPE_UINT32),    // distance (m * 100)
        ];
        encoder.write_definition(2, MESG_RECORD, &fields);
    }

    encoder.write_data(
        2,
        &[
            &timestamp.to_le_bytes(),
            &power.to_le_bytes(),
            &[heart_rate],
            &[cadence],
            &speed.to_le_bytes(),
            &distance.to_le_bytes(),
        ],
    );
}

/// Lap message — summary for one lap (entire workout = 1 lap).
/// Local message type: 3
pub fn write_lap(encoder: &mut FitEncoder, lap: &LapData) {
    let fields = vec![
        FieldDef::new(253, 4, BASE_TYPE_UINT32),  // timestamp
        FieldDef::new(2, 4, BASE_TYPE_UINT32),    // start_time
        FieldDef::new(7, 4, BASE_TYPE_UINT32),    // total_elapsed_time (ms)
        FieldDef::new(8, 4, BASE_TYPE_UINT32),    // total_timer_time (ms)
        FieldDef::new(9, 4, BASE_TYPE_UINT32),    // total_distance (m * 100)
        FieldDef::new(0, 1, BASE_TYPE_ENUM),      // event
        FieldDef::new(1, 1, BASE_TYPE_ENUM),      // event_type
        FieldDef::new(24, 1, BASE_TYPE_ENUM),     // lap_trigger
        FieldDef::new(25, 1, BASE_TYPE_ENUM),     // sport
        FieldDef::new(26, 1, BASE_TYPE_ENUM),     // sub_sport
        FieldDef::new(19, 2, BASE_TYPE_UINT16),   // avg_power
        FieldDef::new(20, 2, BASE_TYPE_UINT16),   // max_power
        FieldDef::new(15, 1, BASE_TYPE_UINT8),    // avg_heart_rate
        FieldDef::new(16, 1, BASE_TYPE_UINT8),    // max_heart_rate
        FieldDef::new(17, 1, BASE_TYPE_UINT8),    // avg_cadence
        FieldDef::new(18, 1, BASE_TYPE_UINT8),    // max_cadence
    ];

    let elapsed_ms = (lap.total_elapsed_time * 1000.0) as u32;
    let timer_ms = (lap.total_timer_time * 1000.0) as u32;
    let distance_cm = (lap.total_distance * 100.0) as u32;

    encoder.write_message_with_def(
        3,
        MESG_LAP,
        &fields,
        &[
            &lap.timestamp.to_le_bytes(),
            &lap.start_time.to_le_bytes(),
            &elapsed_ms.to_le_bytes(),
            &timer_ms.to_le_bytes(),
            &distance_cm.to_le_bytes(),
            &[EVENT_TIMER],
            &[EVENT_TYPE_STOP_ALL],
            &[LAP_TRIGGER_SESSION_END],
            &[SPORT_CYCLING],
            &[SUB_SPORT_INDOOR_CYCLING],
            &lap.avg_power.to_le_bytes(),
            &lap.max_power.to_le_bytes(),
            &[lap.avg_heart_rate],
            &[lap.max_heart_rate],
            &[lap.avg_cadence],
            &[lap.max_cadence],
        ],
    );
}

/// Session message — overall workout summary.
/// Local message type: 4
pub fn write_session(encoder: &mut FitEncoder, session: &SessionData) {
    let fields = vec![
        FieldDef::new(253, 4, BASE_TYPE_UINT32),  // timestamp
        FieldDef::new(2, 4, BASE_TYPE_UINT32),    // start_time
        FieldDef::new(7, 4, BASE_TYPE_UINT32),    // total_elapsed_time (ms)
        FieldDef::new(8, 4, BASE_TYPE_UINT32),    // total_timer_time (ms)
        FieldDef::new(9, 4, BASE_TYPE_UINT32),    // total_distance (m * 100)
        FieldDef::new(5, 1, BASE_TYPE_ENUM),      // sport
        FieldDef::new(6, 1, BASE_TYPE_ENUM),      // sub_sport
        FieldDef::new(20, 2, BASE_TYPE_UINT16),   // avg_power
        FieldDef::new(21, 2, BASE_TYPE_UINT16),   // max_power
        FieldDef::new(16, 1, BASE_TYPE_UINT8),    // avg_heart_rate
        FieldDef::new(17, 1, BASE_TYPE_UINT8),    // max_heart_rate
        FieldDef::new(18, 1, BASE_TYPE_UINT8),    // avg_cadence
        FieldDef::new(19, 1, BASE_TYPE_UINT8),    // max_cadence
        FieldDef::new(14, 2, BASE_TYPE_UINT16),   // avg_speed (m/s * 1000)
        FieldDef::new(15, 2, BASE_TYPE_UINT16),   // max_speed (m/s * 1000)
        FieldDef::new(0, 1, BASE_TYPE_ENUM),      // event
        FieldDef::new(1, 1, BASE_TYPE_ENUM),      // event_type
        FieldDef::new(25, 2, BASE_TYPE_UINT16),   // first_lap_index
        FieldDef::new(26, 2, BASE_TYPE_UINT16),   // num_laps
    ];

    let elapsed_ms = (session.total_elapsed_time * 1000.0) as u32;
    let timer_ms = (session.total_timer_time * 1000.0) as u32;
    let distance_cm = (session.total_distance * 100.0) as u32;

    encoder.write_message_with_def(
        4,
        MESG_SESSION,
        &fields,
        &[
            &session.timestamp.to_le_bytes(),
            &session.start_time.to_le_bytes(),
            &elapsed_ms.to_le_bytes(),
            &timer_ms.to_le_bytes(),
            &distance_cm.to_le_bytes(),
            &[SPORT_CYCLING],
            &[SUB_SPORT_INDOOR_CYCLING],
            &session.avg_power.to_le_bytes(),
            &session.max_power.to_le_bytes(),
            &[session.avg_heart_rate],
            &[session.max_heart_rate],
            &[session.avg_cadence],
            &[session.max_cadence],
            &session.avg_speed.to_le_bytes(),
            &session.max_speed.to_le_bytes(),
            &[EVENT_TIMER],
            &[EVENT_TYPE_STOP_ALL],
            &0u16.to_le_bytes(),   // first_lap_index = 0
            &1u16.to_le_bytes(),   // num_laps = 1
        ],
    );
}

/// Activity message — wraps everything.
/// Local message type: 5
pub fn write_activity(encoder: &mut FitEncoder, timestamp: u32, total_timer_time: f64) {
    let fields = vec![
        FieldDef::new(253, 4, BASE_TYPE_UINT32),  // timestamp
        FieldDef::new(0, 4, BASE_TYPE_UINT32),    // total_timer_time (sec * 1000 → hundredths? No: sec / 1)
        FieldDef::new(1, 2, BASE_TYPE_UINT16),    // num_sessions
        FieldDef::new(2, 1, BASE_TYPE_ENUM),      // type
        FieldDef::new(3, 1, BASE_TYPE_ENUM),      // event
        FieldDef::new(4, 1, BASE_TYPE_ENUM),      // event_type
    ];

    // Activity total_timer_time is in seconds (not ms) as uint32
    let timer_secs = total_timer_time as u32;

    encoder.write_message_with_def(
        5,
        MESG_ACTIVITY,
        &fields,
        &[
            &timestamp.to_le_bytes(),
            &timer_secs.to_le_bytes(),
            &1u16.to_le_bytes(),         // 1 session
            &[ACTIVITY_MANUAL],
            &[EVENT_TIMER],
            &[EVENT_TYPE_STOP_ALL],
        ],
    );
}

/// Data needed to write a Lap message.
pub struct LapData {
    pub timestamp: u32,
    pub start_time: u32,
    pub total_elapsed_time: f64,
    pub total_timer_time: f64,
    pub total_distance: f64,
    pub avg_power: u16,
    pub max_power: u16,
    pub avg_heart_rate: u8,
    pub max_heart_rate: u8,
    pub avg_cadence: u8,
    pub max_cadence: u8,
}

/// Data needed to write a Session message.
pub struct SessionData {
    pub timestamp: u32,
    pub start_time: u32,
    pub total_elapsed_time: f64,
    pub total_timer_time: f64,
    pub total_distance: f64,
    pub avg_power: u16,
    pub max_power: u16,
    pub avg_heart_rate: u8,
    pub max_heart_rate: u8,
    pub avg_cadence: u8,
    pub max_cadence: u8,
    pub avg_speed: u16,   // m/s * 1000
    pub max_speed: u16,   // m/s * 1000
}

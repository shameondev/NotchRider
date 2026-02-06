/// Workout recorder — captures trainer data samples and produces FIT files.

use crate::ant::TrainerData;
use crate::fit::encoder::FitEncoder;
use crate::fit::messages::{self, LapData, SessionData};
use crate::fit::types;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// A single recorded data sample.
struct Sample {
    timestamp: u32,        // FIT timestamp
    power: u16,
    heart_rate: u8,
    cadence: u8,
    speed_mps_1000: u16,  // speed in m/s * 1000
    distance_cm: u32,     // cumulative distance in cm
}

/// Records a workout and produces a FIT file.
pub struct WorkoutRecorder {
    start_time: u32,
    samples: Vec<Sample>,
    cumulative_distance: f64, // meters
    paused: bool,
    pause_duration: f64, // total seconds paused
}

/// Summary returned to the frontend after saving.
#[derive(Serialize, Clone)]
pub struct WorkoutSummary {
    pub duration_secs: u32,
    pub distance_km: f64,
    pub avg_power: u16,
    pub max_power: u16,
    pub avg_hr: u8,
    pub max_hr: u8,
    pub avg_cadence: u8,
    pub sample_count: usize,
    pub file_path: String,
}

impl WorkoutRecorder {
    pub fn new() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let fit_ts = types::unix_to_fit_timestamp(now);

        Self {
            start_time: fit_ts,
            samples: Vec::with_capacity(7200), // 2 hours at 1/sec
            cumulative_distance: 0.0,
            paused: false,
            pause_duration: 0.0,
        }
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    /// Add a data sample from the trainer. Called ~1/sec.
    pub fn add_sample(&mut self, data: &TrainerData) {
        if self.paused {
            self.pause_duration += 1.0;
            return;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let fit_ts = types::unix_to_fit_timestamp(now);

        // Accumulate distance from speed (km/h -> m/s * 1 sec)
        let speed_mps = data.speed as f64 / 3.6;
        self.cumulative_distance += speed_mps; // 1 second interval

        self.samples.push(Sample {
            timestamp: fit_ts,
            power: data.power,
            heart_rate: data.heart_rate,
            cadence: data.cadence,
            speed_mps_1000: (speed_mps * 1000.0) as u16,
            distance_cm: (self.cumulative_distance * 100.0) as u32,
        });
    }

    /// Encode all samples into a FIT file binary.
    fn encode_fit(&self) -> Vec<u8> {
        let mut encoder = FitEncoder::new();

        let end_time = self.samples.last().map(|s| s.timestamp).unwrap_or(self.start_time);
        let serial = self.start_time; // Use start time as serial number

        // 1. File ID
        messages::write_file_id(&mut encoder, self.start_time, serial);

        // 2. Event: timer start
        messages::write_event(&mut encoder, self.start_time, types::EVENT_TYPE_START);

        // 3. Records
        for (i, sample) in self.samples.iter().enumerate() {
            messages::write_record(
                &mut encoder,
                sample.timestamp,
                sample.power,
                sample.heart_rate,
                sample.cadence,
                sample.speed_mps_1000,
                sample.distance_cm,
                i == 0,
            );
        }

        // 4. Event: timer stop
        messages::write_event(&mut encoder, end_time, types::EVENT_TYPE_STOP_ALL);

        // Compute summary stats
        let stats = self.compute_stats();

        // 5. Lap
        messages::write_lap(&mut encoder, &LapData {
            timestamp: end_time,
            start_time: self.start_time,
            total_elapsed_time: stats.elapsed_time,
            total_timer_time: stats.timer_time,
            total_distance: self.cumulative_distance,
            avg_power: stats.avg_power,
            max_power: stats.max_power,
            avg_heart_rate: stats.avg_hr,
            max_heart_rate: stats.max_hr,
            avg_cadence: stats.avg_cadence,
            max_cadence: stats.max_cadence,
        });

        // 6. Session
        messages::write_session(&mut encoder, &SessionData {
            timestamp: end_time,
            start_time: self.start_time,
            total_elapsed_time: stats.elapsed_time,
            total_timer_time: stats.timer_time,
            total_distance: self.cumulative_distance,
            avg_power: stats.avg_power,
            max_power: stats.max_power,
            avg_heart_rate: stats.avg_hr,
            max_heart_rate: stats.max_hr,
            avg_cadence: stats.avg_cadence,
            max_cadence: stats.max_cadence,
            avg_speed: stats.avg_speed,
            max_speed: stats.max_speed,
        });

        // 7. Activity
        messages::write_activity(&mut encoder, end_time, stats.timer_time);

        encoder.finish()
    }

    fn compute_stats(&self) -> Stats {
        if self.samples.is_empty() {
            return Stats::default();
        }

        let n = self.samples.len() as u64;
        let mut total_power: u64 = 0;
        let mut max_power: u16 = 0;
        let mut total_hr: u64 = 0;
        let mut max_hr: u8 = 0;
        let mut total_cadence: u64 = 0;
        let mut max_cadence: u8 = 0;
        let mut total_speed: u64 = 0;
        let mut max_speed: u16 = 0;

        // For HR/cadence averages, only count non-zero values
        let mut hr_count: u64 = 0;
        let mut cadence_count: u64 = 0;

        for sample in &self.samples {
            total_power += sample.power as u64;
            if sample.power > max_power { max_power = sample.power; }

            if sample.heart_rate > 0 {
                total_hr += sample.heart_rate as u64;
                hr_count += 1;
            }
            if sample.heart_rate > max_hr { max_hr = sample.heart_rate; }

            if sample.cadence > 0 {
                total_cadence += sample.cadence as u64;
                cadence_count += 1;
            }
            if sample.cadence > max_cadence { max_cadence = sample.cadence; }

            total_speed += sample.speed_mps_1000 as u64;
            if sample.speed_mps_1000 > max_speed { max_speed = sample.speed_mps_1000; }
        }

        let elapsed_time = if self.samples.len() >= 2 {
            (self.samples.last().unwrap().timestamp - self.samples[0].timestamp) as f64
        } else {
            self.samples.len() as f64
        };

        Stats {
            elapsed_time: elapsed_time + self.pause_duration,
            timer_time: elapsed_time,
            avg_power: (total_power / n) as u16,
            max_power,
            avg_hr: if hr_count > 0 { (total_hr / hr_count) as u8 } else { 0 },
            max_hr,
            avg_cadence: if cadence_count > 0 { (total_cadence / cadence_count) as u8 } else { 0 },
            max_cadence,
            avg_speed: (total_speed / n) as u16,
            max_speed,
        }
    }

    /// Save the workout to disk and return a summary.
    pub fn save(&self) -> Result<WorkoutSummary, String> {
        if self.samples.is_empty() {
            return Err("No samples recorded".to_string());
        }

        let fit_bytes = self.encode_fit();

        // Build save path
        let dir = workout_dir()?;
        fs::create_dir_all(&dir).map_err(|e| format!("Failed to create directory: {}", e))?;

        let filename = format_filename();
        let file_path = dir.join(&filename);

        fs::write(&file_path, &fit_bytes)
            .map_err(|e| format!("Failed to write FIT file: {}", e))?;

        let stats = self.compute_stats();

        Ok(WorkoutSummary {
            duration_secs: stats.timer_time as u32,
            distance_km: self.cumulative_distance / 1000.0,
            avg_power: stats.avg_power,
            max_power: stats.max_power,
            avg_hr: stats.avg_hr,
            max_hr: stats.max_hr,
            avg_cadence: stats.avg_cadence,
            sample_count: self.samples.len(),
            file_path: file_path.to_string_lossy().to_string(),
        })
    }
}

#[derive(Default)]
struct Stats {
    elapsed_time: f64,
    timer_time: f64,
    avg_power: u16,
    max_power: u16,
    avg_hr: u8,
    max_hr: u8,
    avg_cadence: u8,
    max_cadence: u8,
    avg_speed: u16,
    max_speed: u16,
}

fn workout_dir() -> Result<PathBuf, String> {
    let home = std::env::var("HOME").map_err(|_| "HOME not set".to_string())?;
    Ok(PathBuf::from(home)
        .join("Library/Application Support/com.notchrider.app/workouts"))
}

fn format_filename() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Convert to date components manually (no chrono dependency)
    // We use a simple approach: format as Unix timestamp if we can't get local time
    // Actually, let's use the std approach with seconds math
    let secs_per_day: u64 = 86400;
    let days = now / secs_per_day;
    let time_of_day = now % secs_per_day;
    let hours = time_of_day / 3600;
    let minutes = (time_of_day % 3600) / 60;
    let seconds = time_of_day % 60;

    // Days since epoch to Y-M-D (simplified Gregorian calculation)
    let (year, month, day) = days_to_ymd(days);

    format!(
        "{:04}-{:02}-{:02}_{:02}-{:02}-{:02}.fit",
        year, month, day, hours, minutes, seconds
    )
}

/// Convert days since Unix epoch to (year, month, day).
fn days_to_ymd(days: u64) -> (u64, u64, u64) {
    // Algorithm from http://howardhinnant.github.io/date_algorithms.html
    let z = days + 719468;
    let era = z / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_days_to_ymd() {
        // 2024-01-01 = day 19723 since epoch
        let (y, m, d) = days_to_ymd(19723);
        assert_eq!((y, m, d), (2024, 1, 1));
    }

    #[test]
    fn test_recorder_basic() {
        let mut recorder = WorkoutRecorder::new();

        for _ in 0..10 {
            recorder.add_sample(&TrainerData::new(200, 30.0, 90, 140));
        }

        assert_eq!(recorder.samples.len(), 10);
        assert!(recorder.cumulative_distance > 0.0);
    }

    #[test]
    fn test_recorder_paused_skips_samples() {
        let mut recorder = WorkoutRecorder::new();

        recorder.add_sample(&TrainerData::new(200, 30.0, 90, 140));
        recorder.set_paused(true);
        recorder.add_sample(&TrainerData::new(200, 30.0, 90, 140));
        recorder.set_paused(false);
        recorder.add_sample(&TrainerData::new(200, 30.0, 90, 140));

        assert_eq!(recorder.samples.len(), 2);
    }

    #[test]
    fn test_encode_fit_produces_valid_file() {
        let mut recorder = WorkoutRecorder::new();

        for _ in 0..5 {
            recorder.add_sample(&TrainerData::new(200, 30.0, 90, 140));
        }

        let fit_bytes = recorder.encode_fit();

        // Check FIT header
        assert!(fit_bytes.len() > 16);
        assert_eq!(fit_bytes[0], 14); // header size
        assert_eq!(&fit_bytes[8..12], b".FIT");
    }
}

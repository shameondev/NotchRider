export interface TrainerData {
  power: number;       // Watts
  speed: number;       // km/h
  cadence: number;     // RPM
  heartRate: number;   // BPM (0 if no HRM)
  distance: number;    // meters
  elapsedTime: number; // seconds
  grade: number;       // percent (-20 to +20)
}

export interface TargetZone {
  min: number;
  max: number;
  metric: 'power' | 'heartRate';
}

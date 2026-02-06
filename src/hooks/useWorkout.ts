import { useRef, useCallback, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

export interface WorkoutSummary {
  duration_secs: number;
  distance_km: number;
  avg_power: number;
  max_power: number;
  avg_hr: number;
  max_hr: number;
  avg_cadence: number;
  sample_count: number;
  file_path: string;
}

interface UseWorkoutResult {
  isRecording: boolean;
  isPaused: boolean;
  duration: number;
  sampleCount: number;
  summary: WorkoutSummary | null;
  start: () => void;
  pause: () => void;
  resume: () => void;
  stop: () => Promise<WorkoutSummary | null>;
  clearSummary: () => void;
}

export function useWorkout(): UseWorkoutResult {
  const [isRecording, setIsRecording] = useState(false);
  const [isPaused, setIsPaused] = useState(false);
  const [duration, setDuration] = useState(0);
  const [sampleCount, setSampleCount] = useState(0);
  const [summary, setSummary] = useState<WorkoutSummary | null>(null);
  const intervalRef = useRef<number>(0);

  const start = useCallback(() => {
    invoke('start_workout').then(() => {
      setIsRecording(true);
      setIsPaused(false);
      setDuration(0);
      setSampleCount(0);
      setSummary(null);

      // Sample every 1 second
      intervalRef.current = window.setInterval(() => {
        invoke('add_workout_sample').catch(console.error);
        setSampleCount(prev => prev + 1);
        setDuration(prev => prev + 1);
      }, 1000);
    }).catch(console.error);
  }, []);

  const pause = useCallback(() => {
    invoke('pause_workout', { paused: true }).catch(console.error);
    setIsPaused(true);
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = 0;
    }
  }, []);

  const resume = useCallback(() => {
    invoke('pause_workout', { paused: false }).catch(console.error);
    setIsPaused(false);

    intervalRef.current = window.setInterval(() => {
      invoke('add_workout_sample').catch(console.error);
      setSampleCount(prev => prev + 1);
      setDuration(prev => prev + 1);
    }, 1000);
  }, []);

  const stop = useCallback(async (): Promise<WorkoutSummary | null> => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = 0;
    }

    try {
      const result = await invoke<WorkoutSummary>('stop_workout');
      setIsRecording(false);
      setIsPaused(false);
      setSummary(result);
      return result;
    } catch (e) {
      console.error('Failed to save workout:', e);
      setIsRecording(false);
      setIsPaused(false);
      return null;
    }
  }, []);

  const clearSummary = useCallback(() => {
    setSummary(null);
  }, []);

  return {
    isRecording,
    isPaused,
    duration,
    sampleCount,
    summary,
    start,
    pause,
    resume,
    stop,
    clearSummary,
  };
}

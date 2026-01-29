import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { TrainerData } from '../types/trainer';

interface UseTrainerResult {
  data: TrainerData;
  isConnected: boolean;
  error: string | null;
  findDevice: () => Promise<void>;
}

export function useTrainer(): UseTrainerResult {
  const [data, setData] = useState<TrainerData>({
    power: 0,
    speed: 0,
    cadence: 0,
    heartRate: 0,
    distance: 0,
    elapsedTime: 0,
    grade: 0,
  });
  const [isConnected, setIsConnected] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const findDevice = useCallback(async () => {
    try {
      const found = await invoke<boolean>('find_ant_device');
      setIsConnected(found);
      if (!found) {
        setError('ANT+ device not found. Please connect your USB dongle.');
      } else {
        setError(null);
      }
    } catch (e) {
      setError(`Error: ${e}`);
      setIsConnected(false);
    }
  }, []);

  // Poll for trainer data when connected
  useEffect(() => {
    if (!isConnected) return;

    const interval = setInterval(async () => {
      try {
        const trainerData = await invoke<{
          power: number;
          speed: number;
          cadence: number;
          heart_rate: number;
        }>('get_trainer_data');

        setData(prev => ({
          ...prev,
          power: trainerData.power,
          speed: trainerData.speed,
          cadence: trainerData.cadence,
          heartRate: trainerData.heart_rate,
        }));
      } catch (e) {
        console.error('Failed to get trainer data:', e);
      }
    }, 100); // 10 updates per second

    return () => clearInterval(interval);
  }, [isConnected]);

  return { data, isConnected, error, findDevice };
}

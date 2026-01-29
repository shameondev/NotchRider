import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { TrainerData } from '../types/trainer';

interface UseTrainerResult {
  data: TrainerData;
  isConnected: boolean;
  isSimulation: boolean;
  error: string | null;
  connect: () => Promise<void>;
  disconnect: () => Promise<void>;
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
  const [isSimulation, setIsSimulation] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Try to connect to real ANT+ device, fallback to simulation
  const connect = useCallback(async () => {
    try {
      // First check if device exists
      const found = await invoke<boolean>('find_ant_device');

      if (found) {
        // Try to connect and initialize
        await invoke<boolean>('connect_ant_device');
        setIsConnected(true);
        setIsSimulation(false);
        setError(null);
      } else {
        // No device found, use simulation mode
        console.log('No ANT+ device found, using simulation mode');
        setIsConnected(true);
        setIsSimulation(true);
        setError(null);
      }
    } catch (e) {
      // Connection failed, use simulation mode
      console.warn('ANT+ connection failed, using simulation mode:', e);
      setIsConnected(true);
      setIsSimulation(true);
      setError(null);
    }
  }, []);

  const disconnect = useCallback(async () => {
    try {
      if (!isSimulation) {
        await invoke('disconnect_ant_device');
      }
    } catch (e) {
      console.error('Disconnect error:', e);
    }
    setIsConnected(false);
    setIsSimulation(false);
  }, [isSimulation]);

  // Poll for trainer data when connected
  useEffect(() => {
    if (!isConnected) return;

    const interval = setInterval(async () => {
      try {
        if (isSimulation) {
          // Simulation mode: generate realistic cycling data
          setData(prev => {
            const time = Date.now() / 1000;
            // Simulate variations in power, cadence, speed
            const basePower = 150;
            const powerVariation = Math.sin(time * 0.5) * 30 + Math.random() * 10;
            const power = Math.round(basePower + powerVariation);

            const baseCadence = 85;
            const cadenceVariation = Math.sin(time * 0.3) * 10 + Math.random() * 5;
            const cadence = Math.round(baseCadence + cadenceVariation);

            // Speed roughly correlates with power
            const speed = Math.round((power / 10) * 10) / 10;

            // Heart rate increases with effort
            const baseHr = 120;
            const hrVariation = power / 10 + Math.random() * 5;
            const heartRate = Math.round(baseHr + hrVariation);

            // Accumulate distance based on speed
            const elapsedTime = prev.elapsedTime + 0.1; // 100ms intervals
            const distance = prev.distance + (speed / 3600) * 0.1; // km

            return {
              power,
              cadence,
              speed,
              heartRate,
              distance: Math.round(distance * 100) / 100,
              elapsedTime: Math.round(elapsedTime * 10) / 10,
              grade: prev.grade,
            };
          });
        } else {
          // Real hardware mode: poll from backend
          const trainerData = await invoke<{
            power: number;
            speed: number;
            cadence: number;
            heart_rate: number;
          } | null>('poll_trainer_data');

          if (trainerData) {
            setData(prev => ({
              ...prev,
              power: trainerData.power,
              speed: trainerData.speed,
              cadence: trainerData.cadence,
              heartRate: trainerData.heart_rate,
              // Accumulate distance and time
              distance: prev.distance + (trainerData.speed / 3600) * 0.1,
              elapsedTime: prev.elapsedTime + 0.1,
            }));
          }
        }
      } catch (e) {
        console.error('Failed to get trainer data:', e);
      }
    }, 100); // 10 updates per second

    return () => clearInterval(interval);
  }, [isConnected, isSimulation]);

  // Auto-connect on mount
  useEffect(() => {
    let mounted = true;

    const initConnection = async () => {
      if (mounted) {
        try {
          const found = await invoke<boolean>('find_ant_device');

          if (!mounted) return;

          if (found) {
            await invoke<boolean>('connect_ant_device');
            if (mounted) {
              setIsConnected(true);
              setIsSimulation(false);
            }
          } else {
            console.log('No ANT+ device found, using simulation mode');
            if (mounted) {
              setIsConnected(true);
              setIsSimulation(true);
            }
          }
        } catch (e) {
          console.warn('ANT+ connection failed, using simulation mode:', e);
          if (mounted) {
            setIsConnected(true);
            setIsSimulation(true);
          }
        }
      }
    };

    initConnection();

    return () => {
      mounted = false;
      // Cleanup: try to disconnect if we were connected
      invoke('disconnect_ant_device').catch(() => {
        // Ignore disconnect errors on cleanup
      });
    };
  }, []);

  return { data, isConnected, isSimulation, error, connect, disconnect };
}

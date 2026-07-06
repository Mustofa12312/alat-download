import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { useAppStore } from '../stores/useAppStore';

interface DownloadProgressPayload {
  id: number;
  speed: number;
  downloaded_bytes: number;
  total_bytes: number;
}

interface DownloadStatusPayload {
  id: number;
  status: number;
  error_message?: string;
}

export function useTauriEvents() {
  const { updateProgress, updateStatus } = useAppStore();

  useEffect(() => {
    const unlistenProgress = listen<DownloadProgressPayload>('vdm://download-progress', (event) => {
      const { id, speed, downloaded_bytes, total_bytes } = event.payload;
      updateProgress(id, downloaded_bytes, total_bytes, speed);
    });

    const unlistenStatus = listen<DownloadStatusPayload>('vdm://download-status', (event) => {
      const { id, status, error_message } = event.payload;
      updateStatus(id, status, error_message);
    });

    return () => {
      unlistenProgress.then((f) => f());
      unlistenStatus.then((f) => f());
    };
  }, [updateProgress, updateStatus]);
}

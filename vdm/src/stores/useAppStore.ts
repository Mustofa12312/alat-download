import { create } from 'zustand';

export interface DownloadState {
  id: number;
  speed: number;
  downloadedBytes: number;
  totalBytes: number;
  status: number;
  errorMessage?: string;
}

interface AppState {
  theme: 'dark' | 'light' | 'auto';
  setTheme: (theme: 'dark' | 'light' | 'auto') => void;
  
  activeDownloads: Record<number, DownloadState>;
  updateProgress: (id: number, downloadedBytes: number, totalBytes: number, speed: number) => void;
  updateStatus: (id: number, status: number, errorMessage?: string) => void;
}

export const useAppStore = create<AppState>((set) => ({
  theme: 'dark',
  setTheme: (theme) => set({ theme }),

  activeDownloads: {},
  updateProgress: (id, downloadedBytes, totalBytes, speed) => 
    set((state) => ({
      activeDownloads: {
        ...state.activeDownloads,
        [id]: {
          ...state.activeDownloads[id],
          id,
          downloadedBytes,
          totalBytes,
          speed,
        }
      }
    })),
    
  updateStatus: (id, status, errorMessage) =>
    set((state) => ({
      activeDownloads: {
        ...state.activeDownloads,
        [id]: {
          ...state.activeDownloads[id],
          id,
          status,
          errorMessage,
        }
      }
    })),
}));

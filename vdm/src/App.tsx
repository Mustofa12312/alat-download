import { BrowserRouter, Routes, Route, useNavigate } from 'react-router-dom';
import { useEffect } from 'react';
import { useAppStore } from './stores/useAppStore';
import { register } from '@tauri-apps/plugin-global-shortcut';
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';
import { getCurrentWindow } from '@tauri-apps/api/window';
import MainLayout from './layouts/MainLayout';
import Dashboard from './pages/Dashboard';
import Downloads from './pages/Downloads';
import VideoDownloader from './pages/VideoDownloader';
import Torrent from './pages/Torrent';
import History from './pages/History';
import Statistics from './pages/Statistics';
import Scheduler from './pages/Scheduler';
import Settings from './pages/Settings';
import About from './pages/About';

function AppRoutes() {
  const navigate = useNavigate();

  useEffect(() => {
    // 1. Local App Keyboard Shortcuts
    const handleKeyDown = (e: KeyboardEvent) => {
      // Ctrl+N or Cmd+N -> Go to Downloads (to add new)
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'n') {
        e.preventDefault();
        navigate('/downloads');
      }
      // Ctrl+J or Cmd+J -> Go to Downloads
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'j') {
        e.preventDefault();
        navigate('/downloads');
      }
    };
    window.addEventListener('keydown', handleKeyDown);

    // 2. Setup Global Shortcut & Notification Permission
    const setupSys = async () => {
      try {
        let permissionGranted = await isPermissionGranted();
        if (!permissionGranted) {
          const permission = await requestPermission();
          permissionGranted = permission === 'granted';
        }
        
        await register('CommandOrControl+Shift+D', async () => {
          const win = getCurrentWindow();
          await win.unminimize();
          await win.setFocus();
          
          if (permissionGranted) {
            sendNotification({ title: 'VDM', body: 'Velocity Download Manager is ready!' });
          }
        });
      } catch (err) {
        console.error('Failed to register global shortcut', err);
      }
    };
    setupSys();

    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [navigate]);

  return (
    <Routes>
      <Route path="/" element={<MainLayout />}>
        <Route index element={<Dashboard />} />
        <Route path="downloads" element={<Downloads />} />
        <Route path="video-downloader" element={<VideoDownloader />} />
        <Route path="torrent" element={<Torrent />} />
        <Route path="history" element={<History />} />
        <Route path="statistics" element={<Statistics />} />
        <Route path="scheduler" element={<Scheduler />} />
        <Route path="settings" element={<Settings />} />
        <Route path="about" element={<About />} />
      </Route>
    </Routes>
  );
}

export default function App() {
  const theme = useAppStore((state) => state.theme);

  useEffect(() => {
    const root = document.documentElement;
    if (theme === 'dark') {
      root.classList.add('dark');
    } else if (theme === 'light') {
      root.classList.remove('dark');
    } else {
      // Auto logic
      const systemDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      if (systemDark) root.classList.add('dark');
      else root.classList.remove('dark');
    }
  }, [theme]);

  return (
    <BrowserRouter>
      <AppRoutes />
    </BrowserRouter>
  );
}

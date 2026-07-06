import { BrowserRouter, Routes, Route } from 'react-router-dom';
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

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<MainLayout />}>
          <Route index element={<Dashboard />} />
          <Route path="downloads" element={<Downloads />} />
          <Route path="video" element={<VideoDownloader />} />
          <Route path="torrent" element={<Torrent />} />
          <Route path="history" element={<History />} />
          <Route path="statistics" element={<Statistics />} />
          <Route path="scheduler" element={<Scheduler />} />
          <Route path="settings" element={<Settings />} />
          <Route path="about" element={<About />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

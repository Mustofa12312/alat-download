import { useAppStore } from '../stores/useAppStore';
import { useTauriEvents } from '../hooks/useTauriEvents';

export default function Dashboard() {
  useTauriEvents(); // Initialize event listeners

  const activeDownloads = useAppStore((state) => state.activeDownloads);
  const downloadsList = Object.values(activeDownloads);
  
  const totalSpeed = downloadsList.reduce((acc, dl) => acc + (dl.speed || 0), 0);
  const activeCount = downloadsList.filter(dl => dl.status === 1).length;

  const formatSpeed = (bytesPerSec: number) => {
    if (bytesPerSec === 0) return '0 KB/s';
    const mb = bytesPerSec / (1024 * 1024);
    if (mb >= 1) return `${mb.toFixed(2)} MB/s`;
    return `${(bytesPerSec / 1024).toFixed(2)} KB/s`;
  };
        <h1 className="text-2xl font-bold text-textPrimary">Dashboard</h1>
        <p className="text-textSecondary mt-1">Welcome to Velocity Download Manager</p>
      </div>
      
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="bg-surface border border-border rounded-xl p-6 shadow-sm">
          <h3 className="text-textSecondary text-sm font-medium">Active Downloads</h3>
          <p className="text-3xl font-bold text-textPrimary mt-2">{activeCount}</p>
        </div>
        <div className="bg-surface border border-border rounded-xl p-6 shadow-sm">
          <h3 className="text-textSecondary text-sm font-medium">Total Speed</h3>
          <p className="text-3xl font-bold text-textPrimary mt-2">{formatSpeed(totalSpeed)}</p>
        </div>
        <div className="bg-surface border border-border rounded-xl p-6 shadow-sm">
          <h3 className="text-textSecondary text-sm font-medium">Completed Today</h3>
          <p className="text-3xl font-bold text-textPrimary mt-2">0</p>
        </div>
      </div>
    </div>
  );
}

import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useAppStore } from '../stores/useAppStore';
import { Play, Pause, X, Trash2, Plus } from 'lucide-react';

export default function Downloads() {
  const activeDownloads = useAppStore((state) => state.activeDownloads);
  const downloadsList = Object.values(activeDownloads);
  const [urlInput, setUrlInput] = useState('');

  const formatSize = (bytes: number) => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  const handleStartDownload = async () => {
    if (!urlInput) return;
    try {
      await invoke('start_download', { url: urlInput });
      setUrlInput('');
    } catch (e) {
      console.error(e);
    }
  };

  const handleAction = async (action: string, id: number) => {
    try {
      await invoke(action, { id });
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <div className="p-8 h-full flex flex-col">
      <div className="flex justify-between items-center mb-8">
        <div>
          <h1 className="text-2xl font-bold text-textPrimary">Downloads</h1>
          <p className="text-textSecondary mt-1">Manage your active and completed downloads</p>
        </div>
        <div className="flex gap-2">
          <input 
            type="text" 
            placeholder="Enter URL to download..." 
            className="bg-surface border border-border text-textPrimary rounded-lg px-4 py-2 w-64 focus:outline-none focus:border-primary"
            value={urlInput}
            onChange={(e) => setUrlInput(e.target.value)}
          />
          <button 
            onClick={handleStartDownload}
            className="bg-primary hover:bg-blue-600 text-white rounded-lg px-4 py-2 flex items-center gap-2 font-medium transition-colors"
          >
            <Plus size={18} /> Add
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto pr-2 space-y-4">
        {downloadsList.length === 0 ? (
          <div className="bg-surface border border-border rounded-xl h-64 flex flex-col items-center justify-center">
            <div className="text-textSecondary text-4xl mb-4">📦</div>
            <p className="text-textSecondary font-medium">No active downloads</p>
            <p className="text-textSecondary text-sm mt-1">Add a new URL to get started.</p>
          </div>
        ) : (
          downloadsList.map((dl) => {
            const percent = dl.totalBytes > 0 ? (dl.downloadedBytes / dl.totalBytes) * 100 : 0;
            
            return (
              <div key={dl.id} className="bg-surface border border-border rounded-xl p-5 hover:border-primary/50 transition-colors">
                <div className="flex justify-between items-start mb-3">
                  <div>
                    <h3 className="text-textPrimary font-semibold truncate max-w-lg" title={`Download #${dl.id}`}>
                      Download #{dl.id}
                    </h3>
                    <p className="text-textSecondary text-sm mt-1 flex gap-4">
                      <span>{formatSize(dl.downloadedBytes)} / {formatSize(dl.totalBytes)}</span>
                      <span className="text-primary">{formatSize(dl.speed)}/s</span>
                    </p>
                  </div>
                  <div className="flex gap-2 text-textSecondary">
                    <button onClick={() => handleAction('resume_download', dl.id)} className="p-2 hover:text-success hover:bg-success/10 rounded-lg transition-colors" title="Resume">
                      <Play size={18} />
                    </button>
                    <button onClick={() => handleAction('pause_download', dl.id)} className="p-2 hover:text-warning hover:bg-warning/10 rounded-lg transition-colors" title="Pause">
                      <Pause size={18} />
                    </button>
                    <button onClick={() => handleAction('cancel_download', dl.id)} className="p-2 hover:text-error hover:bg-error/10 rounded-lg transition-colors" title="Cancel">
                      <X size={18} />
                    </button>
                    <button onClick={() => handleAction('delete_download', dl.id)} className="p-2 hover:text-error hover:bg-error/10 rounded-lg transition-colors" title="Delete">
                      <Trash2 size={18} />
                    </button>
                  </div>
                </div>
                
                <div className="w-full bg-background rounded-full h-2.5 overflow-hidden">
                  <div 
                    className="bg-primary h-2.5 rounded-full transition-all duration-300 ease-out"
                    style={{ width: `${percent}%` }}
                  ></div>
                </div>
              </div>
            );
          })
        )}
      </div>
    </div>
  );
}

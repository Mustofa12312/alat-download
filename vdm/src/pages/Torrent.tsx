export default function Torrent() {
  return (
    <div className="p-8 h-full flex flex-col">
      <div className="mb-8">
        <h1 className="text-2xl font-bold text-textPrimary">Torrent</h1>
        <p className="text-textSecondary mt-1">Download from .torrent files and Magnet links</p>
      </div>
      <div className="bg-surface border border-border rounded-xl p-12 flex flex-col items-center justify-center border-dashed">
        <p className="text-textPrimary font-medium mb-2">Drop a .torrent file here or paste a Magnet link</p>
        <button className="bg-primary/10 text-primary px-6 py-2 rounded-lg font-medium hover:bg-primary/20 transition-colors mt-4">
          Browse File
        </button>
      </div>
    </div>
  );
}

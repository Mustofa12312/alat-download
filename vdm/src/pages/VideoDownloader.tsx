export default function VideoDownloader() {
  return (
    <div className="p-8 h-full flex flex-col">
      <div className="mb-8">
        <h1 className="text-2xl font-bold text-textPrimary">Video Downloader</h1>
        <p className="text-textSecondary mt-1">Download videos from YouTube, Vimeo, and more</p>
      </div>
      <div className="bg-surface border border-border rounded-xl p-6">
        <div className="flex gap-4">
          <input 
            type="text" 
            placeholder="Paste video URL here..." 
            className="flex-1 bg-background border border-border rounded-lg px-4 py-2 text-textPrimary focus:outline-none focus:border-primary"
          />
          <button className="bg-primary text-white px-6 py-2 rounded-lg font-medium hover:bg-primary/90 transition-colors">
            Analyze
          </button>
        </div>
      </div>
    </div>
  );
}

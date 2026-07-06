export default function Dashboard() {
  return (
    <div className="p-8 h-full flex flex-col">
      <div className="mb-8">
        <h1 className="text-2xl font-bold text-textPrimary">Dashboard</h1>
        <p className="text-textSecondary mt-1">Welcome to Velocity Download Manager</p>
      </div>
      
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="bg-surface border border-border rounded-xl p-6 shadow-sm">
          <h3 className="text-textSecondary text-sm font-medium">Active Downloads</h3>
          <p className="text-3xl font-bold text-textPrimary mt-2">0</p>
        </div>
        <div className="bg-surface border border-border rounded-xl p-6 shadow-sm">
          <h3 className="text-textSecondary text-sm font-medium">Total Speed</h3>
          <p className="text-3xl font-bold text-textPrimary mt-2">0 KB/s</p>
        </div>
        <div className="bg-surface border border-border rounded-xl p-6 shadow-sm">
          <h3 className="text-textSecondary text-sm font-medium">Completed Today</h3>
          <p className="text-3xl font-bold text-textPrimary mt-2">0</p>
        </div>
      </div>
    </div>
  );
}

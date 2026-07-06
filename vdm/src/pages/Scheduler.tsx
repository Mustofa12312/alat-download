export default function Scheduler() {
  return (
    <div className="p-8 h-full flex flex-col">
      <div className="mb-8">
        <h1 className="text-2xl font-bold text-textPrimary">Scheduler</h1>
        <p className="text-textSecondary mt-1">Schedule your downloads and automated tasks</p>
      </div>
      <div className="bg-surface border border-border rounded-xl flex-1 flex items-center justify-center">
        <p className="text-textSecondary">No scheduled tasks.</p>
      </div>
    </div>
  );
}

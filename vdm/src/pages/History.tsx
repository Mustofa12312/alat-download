export default function History() {
  return (
    <div className="p-8 h-full flex flex-col">
      <div className="mb-8">
        <h1 className="text-2xl font-bold text-textPrimary">History</h1>
        <p className="text-textSecondary mt-1">View your past downloads</p>
      </div>
      <div className="bg-surface border border-border rounded-xl flex-1 flex items-center justify-center">
        <p className="text-textSecondary">Download history is empty.</p>
      </div>
    </div>
  );
}

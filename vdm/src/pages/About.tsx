export default function About() {
  return (
    <div className="p-8 h-full flex flex-col items-center justify-center text-center">
      <div className="w-24 h-24 bg-primary/20 text-primary rounded-2xl flex items-center justify-center mb-6 text-3xl font-bold">
        VDM
      </div>
      <h1 className="text-2xl font-bold text-textPrimary mb-2">Velocity Download Manager</h1>
      <p className="text-textSecondary mb-8">Version 0.1.0</p>
      
      <div className="bg-surface border border-border rounded-xl p-6 w-full max-w-md text-left">
        <h3 className="text-sm font-semibold text-textSecondary mb-4 uppercase tracking-wider">Information</h3>
        <div className="space-y-3">
          <div className="flex justify-between">
            <span className="text-textSecondary">Developer</span>
            <span className="text-textPrimary font-medium">Mustofa & ChatGPT</span>
          </div>
          <div className="flex justify-between">
            <span className="text-textSecondary">License</span>
            <span className="text-textPrimary font-medium">Free</span>
          </div>
          <div className="flex justify-between">
            <span className="text-textSecondary">Architecture</span>
            <span className="text-textPrimary font-medium">Rust + Tauri</span>
          </div>
        </div>
      </div>
    </div>
  );
}

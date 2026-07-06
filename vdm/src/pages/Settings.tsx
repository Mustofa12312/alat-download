import { useAppStore } from '../stores/useAppStore';

export default function Settings() {
  const { theme, setTheme } = useAppStore();

  return (
    <div className="p-8 h-full flex flex-col">
      <div className="mb-8">
        <h1 className="text-2xl font-bold text-textPrimary">Settings</h1>
        <p className="text-textSecondary mt-1">Configure your download manager</p>
      </div>
      
      <div className="flex-1 overflow-y-auto">
        <div className="bg-surface border border-border rounded-xl p-6">
          <h3 className="text-lg font-medium text-textPrimary mb-4">Appearance</h3>
          <div className="flex items-center justify-between py-3 border-b border-border">
            <div>
              <p className="text-textPrimary font-medium">Theme</p>
              <p className="text-textSecondary text-sm">Select application color scheme</p>
            </div>
            <select 
              value={theme}
              onChange={(e) => setTheme(e.target.value as any)}
              className="bg-background border border-border text-textPrimary text-sm rounded-lg p-2.5 focus:ring-primary focus:border-primary outline-none"
            >
              <option value="dark">Dark</option>
              <option value="light">Light</option>
              <option value="auto">System (Auto)</option>
            </select>
          </div>
        </div>
      </div>
    </div>
  );
}

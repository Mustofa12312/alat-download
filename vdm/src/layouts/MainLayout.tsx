import { Outlet, NavLink } from 'react-router-dom';
import { LayoutDashboard, Download, Video, History, Settings, Info } from 'lucide-react';

export default function MainLayout() {
  const navItems = [
    { name: 'Dashboard', path: '/', icon: <LayoutDashboard size={18} /> },
    { name: 'Downloads', path: '/downloads', icon: <Download size={18} /> },
    { name: 'Video Downloader', path: '/video', icon: <Video size={18} /> },
    { name: 'History', path: '/history', icon: <History size={18} /> },
    { name: 'Settings', path: '/settings', icon: <Settings size={18} /> },
    { name: 'About', path: '/about', icon: <Info size={18} /> },
  ];

  return (
    <div className="flex flex-col h-screen overflow-hidden">
      {/* Title Bar Placeholder */}
      <div data-tauri-drag-region className="h-10 bg-surface border-b border-border flex items-center px-4 shrink-0 justify-center relative">
        <span className="text-xs font-semibold text-textSecondary pointer-events-none">Velocity Download Manager</span>
      </div>
      
      {/* Main Content Area */}
      <div className="flex flex-1 overflow-hidden">
        {/* Sidebar */}
        <div className="w-64 bg-surface border-r border-border p-4 flex flex-col shrink-0">
          <nav className="space-y-1 flex-1">
            {navItems.map((item) => (
              <NavLink
                key={item.name}
                to={item.path}
                className={({ isActive }) =>
                  `flex items-center gap-3 px-3 py-2 rounded-md font-medium transition-colors ${
                    isActive
                      ? 'bg-primary/10 text-primary'
                      : 'text-textSecondary hover:bg-white/5 hover:text-textPrimary'
                  }`
                }
              >
                {item.icon}
                {item.name}
              </NavLink>
            ))}
          </nav>
        </div>

        {/* Page Content */}
        <div className="flex-1 overflow-auto bg-background">
          <Outlet />
        </div>
      </div>
    </div>
  );
}

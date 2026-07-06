document.addEventListener('DOMContentLoaded', () => {
  const statusEl = document.getElementById('status');
  
  // Minimal placeholder logic to show status
  // In a real app, this would query the background service worker
  statusEl.textContent = 'Connected to VDM';
  statusEl.className = 'status connected';
});

// Video and Audio detector logic
document.addEventListener('play', (e) => {
  if (e.target && (e.target.tagName === 'VIDEO' || e.target.tagName === 'AUDIO')) {
    const mediaUrl = e.target.src || e.target.currentSrc;
    if (mediaUrl) {
      // Send detected media to background script
      chrome.runtime.sendMessage({
        type: 'MEDIA_DETECTED',
        url: mediaUrl,
        mediaType: e.target.tagName.toLowerCase()
      });
    }
  }
}, true);

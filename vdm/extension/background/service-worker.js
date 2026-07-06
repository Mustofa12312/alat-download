importScripts('../messaging/native-client.js');

chrome.runtime.onInstalled.addListener(() => {
  chrome.contextMenus.create({
    id: 'download-with-vdm',
    title: 'Download with VDM',
    contexts: ['link', 'image', 'video', 'audio']
  });
});

chrome.contextMenus.onClicked.addListener((info, tab) => {
  if (info.menuItemId === 'download-with-vdm') {
    const url = info.linkUrl || info.srcUrl;
    if (url) {
      sendToVDM({ command: 'StartDownload', payload: { url } });
    }
  }
});

chrome.downloads.onCreated.addListener((downloadItem) => {
  // Logic to intercept download and send to VDM
  chrome.downloads.cancel(downloadItem.id, () => {
    sendToVDM({
      command: 'StartDownload',
      payload: {
        url: downloadItem.url,
        file_name: downloadItem.filename,
        total_size: downloadItem.totalBytes,
        mime_type: downloadItem.mime
      }
    });
  });
});

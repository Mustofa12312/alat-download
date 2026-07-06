const NATIVE_APP_NAME = 'com.vdm.app';
let nativePort = null;

function connectToVDM() {
  nativePort = chrome.runtime.connectNative(NATIVE_APP_NAME);
  
  nativePort.onMessage.addListener((message) => {
    console.log('Received from VDM:', message);
  });

  nativePort.onDisconnect.addListener(() => {
    console.error('Disconnected from VDM:', chrome.runtime.lastError?.message);
    nativePort = null;
  });
}

function sendToVDM(message) {
  if (!nativePort) {
    connectToVDM();
  }
  
  if (nativePort) {
    message.requestId = Date.now().toString();
    message.timestamp = new Date().toISOString();
    nativePort.postMessage(message);
  } else {
    console.error('Could not connect to VDM Native Messaging Host');
  }
}

// Port Native Messaging pour OpenDM
let nativePort = null;

function connectOpenDM() {
  nativePort = chrome.runtime.connectNative("com.opendm.downloader");
  nativePort.onMessage.addListener((msg) => {
    console.log("[OpenDM Host]", msg);
  });
}

chrome.runtime.onMessage.addListener((msg, sender, sendResponse) => {
  if (msg.type === "TRIGGER_DOWNLOAD") {
    if (!nativePort) connectOpenDM();
    nativePort.postMessage({
      action: "download",
      url: msg.url,
      referer: sender.url,
      audioOnly: msg.audioOnly || false
    });
    sendResponse({ status: "dispatched_to_opendm" });
  }
});

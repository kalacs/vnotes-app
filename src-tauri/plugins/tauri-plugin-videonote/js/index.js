console.log("INJECT SCRIPT");
window.__INIT = 0;
window.__count = 0;
window.__videoPlayer = null;
window.__findVideoPlayer = findVideoPlayer;
window.__initVideoPlayerProxy = initVideoPlayerProxy;

if (!window.__INIT) {
  console.log("RUN INIT");
  initialize();
  window.__INIT = 1;
}

function initialize() {
  window.__findBody = setInterval(() => {
    const body = document.querySelector("body");

    if (body || window.__count < 2) {
      const button = document.createElement("button");
      const span = document.createElement("span");
      span.textContent = "Go Back";
      button.title = "Go back";
      button.id = "go-back";
      button.appendChild(span);
      button.style.position = "fixed";
      button.style.bottom = "20px";
      button.style.left = "20px";
      button.style.borderRadius = "45px";
      button.style.border = "1px solid transparent";
      button.style.padding = "22px 0";
      button.style.zIndex = 10000000;
      button.onclick = function () {
        window.__findVideoPlayer();
        window.invoke("plugin:videonote|switch_to_main");
      };
      document.querySelector("body").appendChild(button);
      clearInterval(window.__findBody);
    }
    window.__count++;
  }, 1000);

  window.uid = function uid() {
    return window.crypto.getRandomValues(new Uint32Array(1))[0];
  };

  window.transformCallback = function transformCallback(callback, once) {
    var identifier = window.uid();
    var prop = `_${identifier}`;

    Object.defineProperty(window, prop, {
      value: (result) => {
        if (once) {
          Reflect.deleteProperty(window, prop);
        }

        return callback && callback(result);
      },
      writable: false,
      configurable: true,
    });

    return identifier;
  };

  window.invoke = function invoke(cmd, args) {
    return new Promise((resolve, reject) => {
      const callback = window.transformCallback((e) => {
        resolve(e);
        Reflect.deleteProperty(window, error);
      }, true);
      const error = window.transformCallback((e) => {
        reject(e);
        Reflect.deleteProperty(window, callback);
      }, true);

      window.__TAURI_POST_MESSAGE__({
        cmd,
        callback,
        error,
        ...args,
      });
    });
  };

  window.invokeTauriCommand = function invokeTauriCommand(command) {
    return window.invoke("tauri", command);
  };

  window.emit = function emit(event, windowLabel, payload) {
    console.log("EMIT EVENT");
    window.invokeTauriCommand({
      __tauriModule: "Event",
      message: {
        cmd: "emit",
        event,
        windowLabel,
        payload:
          typeof payload === "string" ? payload : JSON.stringify(payload),
      },
    });
  };
}
function findVideoPlayer() {
  const pollingFunction = () => {
    if (window.__videoPlayer) {
      return;
    }
    console.log("SEARCH VIDEO");
    const videoTag = window.document.querySelector("video");
    if (videoTag) {
      window.__videoPlayer = videoTag;
      window.emit("videonotes://video-player-found", "main");
      window.initVideoPlayerProxy();
      clearInterval(window.__pollingVideo);
      window.__pollingVideo = null;
    }
  };
  pollingFunction();
  window.__pollingVideo = setInterval(pollingFunction, 1000);
}
function initVideoPlayerProxy() {
  window.__videoPlayer.addEventListener("play", forwardVideoEvent);
  window.__videoPlayer.addEventListener("playing", forwardVideoEvent);
  window.__videoPlayer.addEventListener("pause", forwardVideoEvent);
  window.__videoPlayer.addEventListener("seeked", forwardVideoEvent);
  window.__videoPlayer.addEventListener("seeking", forwardVideoEvent);
  window.__videoPlayer.addEventListener("abort", forwardVideoEvent);
  window.__videoPlayer.addEventListener("ended", forwardVideoEvent);
  window.__videoPlayer.addEventListener("timeupdate", forwardVideoEvent);
}
function forwardVideoEvent(event) {
  console.log("VIDEO EVENT: ", event);
  window.emit(`videonotes://video-player-event`, "main", {
    name: event.type,
    payload: {
      currentTime: event.target.currentTime,
    },
  });
}

let callback = null;

function prototype_to_ffi(prototype) {
  return Object.fromEntries(
    Object.entries(Object.getOwnPropertyDescriptors(prototype))
      .filter(([_key, value]) => value.value)
      .map(([key, value]) => {
        if (typeof value.value === "function")
          return [key, Function.prototype.call.bind(value.value)];
        return [key, () => value.value];
      })
  );
}

const ckLoaded = CanvasKitInit({
  locateFile: (file) => "assets/lib/canvaskit.wasm",
});
let canvasKit = null;
ckLoaded.then((_canvasKit) => {
  canvasKit = _canvasKit;
  const surface = canvasKit.MakeSWCanvasSurface("foo");
  const paint = new canvasKit.Paint();

  const send_to_mbt = (event_type, data) => {
    const json = JSON.stringify({
      type: event_type,
      data,
    });
    const bytes = new TextEncoder("utf-16").encode(json);
    const uint8array = new Uint8Array(bytes.length * 2);
    for (let i = 0; i < bytes.length; i++) {
      uint8array[i * 2] = bytes[i];
      uint8array[i * 2 + 1] = 0;
    }
    callback.h_rs();
    for (let i = 0; i < uint8array.length; i++) {
      callback.h_rd(uint8array[i]);
    }
    callback.h_re();
  };

  const [log, flush] = (() => {
    let buffer = [];
    function flush() {
      if (buffer.length > 0) {
        const str = new TextDecoder("utf-16")
          .decode(new Uint16Array(buffer).valueOf())
          .replace(/ /g, "");

        console.log(str);
        buffer = [];
      }
    }
    function log(ch) {
      if (ch === "\n".charCodeAt(0)) {
        flush();
      } else if (ch === "\r".charCodeAt(0)) {
        /* noop */
      } else {
        buffer.push(ch);
      }
    }
    return [log, flush];
  })();

  // data channel
  const [h_ss, h_sd, h_se] = (() => {
    let buffer = [];
    function h_ss(str) {
      buffer = [];
    }
    function h_sd(ch) {
      buffer.push(ch);
    }
    function h_se() {
      if (buffer.length > 0) {
        const str = new TextDecoder("utf-16")
          .decode(new Uint16Array(buffer).valueOf())
          .replace(/ /g, "");

        const json = JSON.parse(str);
        buffer = [];
        handleReceive(json);
      }
    }
    return [h_ss, h_sd, h_se];
  })();

  const importObject = {
    __h: {
      h_ss,
      h_sd,
      h_se,
    },
    math: prototype_to_ffi(Math),
    spectest: {
      print_char: log,
    },
  };

  function handleReceive(res) {
    // {
    //   "type": "log",
    //   "data": "Hello, world!"
    // }
    switch (res.type) {
      case "log":
        console.log(res.data);
        break;
      case "error":
        console.error(res.data);
        break;
      case "eval":
        // biome-ignore lint/security/noGlobalEval: <explanation>
        eval(res.data);
        break;
      default:
        console.warn(`Unknown message type: ${res.type}`);
        break;
    }
  }

  WebAssembly.instantiateStreaming(fetch("assets/app.wasm"), importObject).then(
    (obj) => {
      // environment initialization
      callback = obj.instance.exports;
      const { _start } = callback;
      _start();
    }
  );
});

window.addEventListener("resize", () => {
  document.getElementById("foo").width = window.innerWidth;
  document.getElementById("foo").height = window.innerHeight;
  if (canvasKit) {
    canvasKit.resizeCanvas("foo", window.innerWidth, window.innerHeight);
  }
});

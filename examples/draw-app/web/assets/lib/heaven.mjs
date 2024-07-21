import "./binding.mjs";

let _mbt_callbacks = null;
const _mbt_listeners = {};

export function defineEvent(type, callback) {
  _mbt_listeners[type] = callback;
}

export function bindObject(prefix, obj) {
  if (obj === null || typeof obj === "undefined" || typeof obj !== "object") {
    return;
  }
  const properties = Object.getOwnPropertyNames(Object.getPrototypeOf(obj));

  defineEvent(`${prefix}.addEventListener`, (type, listener) => {
    obj.addEventListener(type, (...args) => {
      function getCompleteObject(obj) {
        const result = {};
        const keys = [
          "altKey",
          "bubbles",
          "button",
          "buttons",
          "cancelable",
          "clientX",
          "clientY",
          "ctrlKey",
          "defaultPrevented",
          "detail",
          "eventPhase",
          "metaKey",
          "movementX",
          "movementY",
          "offsetX",
          "offsetY",
          "pageX",
          "pageY",
          "relatedTarget",
          "screenX",
          "screenY",
          "shiftKey",
          "type",
          "timeStamp",
          "isTrusted",
          "x",
          "y",
          ...Object.getOwnPropertyNames(obj),
        ];
        for (const key of keys) {
          result[key] = obj[key];
        }
        return result;
      }
      callFunction(listener, args.map(getCompleteObject));
    });
  });
  for (const methodName of properties) {
    const method = obj[methodName];

    if (typeof method === "function") {
      defineEvent(`${prefix}.${methodName}`, (...args) => {
        method.apply(obj, args);
      });
    } else if (typeof method === "object") {
      bindObject(`${prefix}.${methodName}`, method);
    } else {
      defineEvent(`${prefix}.${methodName}`, () => method);
      defineEvent(`${prefix}.${methodName}.set`, (value) => {
        obj[methodName] = value;
      });
    }
  }
}

export function callFunction(eventType, data) {
  const json = JSON.stringify({ type: eventType, data });
  const uint8array = new TextEncoder("utf-16")
    .encode(json)
    .reduce((acc, byte) => {
      acc.push(byte, 0);
      return acc;
    }, []);
  _mbt_callbacks.h_rs();
  for (let i = 0; i < uint8array.length; i++) {
    _mbt_callbacks.h_rd(uint8array[i]);
  }
  _mbt_callbacks.h_re();
}

window.addEventListener("DOMContentLoaded", () => {
  function prototypeToFFI(prototype) {
    return Object.fromEntries(
      Object.entries(Object.getOwnPropertyDescriptors(prototype))
        .filter(([_key, value]) => value.value)
        .map(([key, value]) => [
          key,
          typeof value.value === "function"
            ? Function.prototype.call.bind(value.value)
            : () => value.value,
        ])
    );
  }

  const [log, flush] = (() => {
    let buffer = [];
    return [
      (ch) => {
        if (ch === "\n".charCodeAt(0)) flush();
        else if (ch !== "\r".charCodeAt(0)) buffer.push(ch);
      },
      () => {
        if (buffer.length > 0) {
          console.log(
            new TextDecoder("utf-16")
              .decode(new Uint16Array(buffer))
              .replace(/\0/g, "")
          );
          buffer = [];
        }
      },
    ];
  })();

  const [h_ss, h_sd, h_se] = (() => {
    let buffer = [];
    return [
      () => {
        buffer = [];
      },
      (ch) => buffer.push(ch),
      () => {
        if (buffer.length > 0) {
          const str = new TextDecoder("utf-16")
            .decode(new Uint16Array(buffer))
            .replace(/\0/g, "");
          handleReceive(JSON.parse(str));
          buffer = [];
        }
      },
    ];
  })();

  const importObject = {
    __h: { h_ss, h_sd, h_se },
    math: prototypeToFFI(Math),
    spectest: { print_char: log },
  };

  function handleReceive(res) {
    switch (res.type) {
      case "log":
        console.log(res.data);
        break;
      case "error":
        console.error(res.data);
        break;
      default:
        if (res.type in _mbt_listeners) {
          if (Array.isArray(res.data)) {
            callFunction("result", _mbt_listeners[res.type](...res.data));
          } else {
            callFunction("result", _mbt_listeners[res.type](res.data));
          }
        }
    }
  }

  WebAssembly.instantiateStreaming(fetch("assets/app.wasm"), importObject).then(
    (obj) => {
      _mbt_callbacks = obj.instance.exports;
      _mbt_callbacks._start();
    }
  );
});

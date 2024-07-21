import { bindObject } from "./heaven.mjs";

document.addEventListener("DOMContentLoaded", () => {
  bindObject("window", window);
  bindObject("document", document);
  bindObject("location", location);
  bindObject("navigator", navigator);
  bindObject("history", history);
  bindObject("screen", screen);
  bindObject("performance", performance);
  bindObject("console", console);
  
  const canvas = document.getElementById("canvas");
  const ctx = canvas.getContext("2d");
  function resize() {
    const ratio = window.devicePixelRatio;
    canvas.width = window.innerWidth * ratio;
    canvas.height = window.innerHeight * ratio;
    ctx.scale(ratio, ratio);
  }
  document.addEventListener("resize", resize);
  resize();
  bindObject("canvas", ctx);
});

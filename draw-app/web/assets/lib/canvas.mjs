import { bindObject } from "./heaven.mjs";

document.addEventListener("DOMContentLoaded", () => {
  const canvas = document.getElementById("canvas");
  const ctx = canvas.getContext("2d");
  function resize() {
    const ratio = window.devicePixelRatio;
    canvas.width = window.innerWidth * ratio;
    canvas.height = window.innerHeight * ratio;
  }
  document.addEventListener("resize", resize);
  resize();
  bindObject("canvas", ctx);
});

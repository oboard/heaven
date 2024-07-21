import { bindObject } from "./heaven.mjs";

document.addEventListener("DOMContentLoaded", () => {
  const canvas = document.getElementById("canvas");
  const ctx = canvas.getContext("2d");
  bindObject("canvas", ctx);
});

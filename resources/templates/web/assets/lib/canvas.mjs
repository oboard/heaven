const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

defineEvent("canvas.clear", () => {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
});

defineEvent("canvas.fillStyle", (data) => {
  ctx.fillStyle = data;
});

defineEvent("canvas.fillText", (text, x, y) => {
  ctx.fillText(text, x, y);
});

defineEvent("canvas.font", (data) => {
  ctx.font = data;
});

defineEvent("canvas.lineCap", (data) => {
  ctx.lineCap = data;
});

defineEvent("canvas.lineJoin", (data) => {
  ctx.lineJoin = data;
});

defineEvent("canvas.lineWidth", (data) => {
  ctx.lineWidth = data;
});

defineEvent("canvas.strokeStyle", (data) => {
  ctx.strokeStyle = data;
});

defineEvent("canvas.strokeText", (text, x, y) => {
  ctx.strokeText(text, x, y);
});

defineEvent("canvas.rect", (x, y, width, height) => {
  ctx.rect(x, y, width, height);
});

defineEvent("canvas.fillRect", (x, y, width, height) => {
  ctx.fillRect(x, y, width, height);
});

defineEvent("canvas.strokeRect", (x, y, width, height) => {
  ctx.strokeRect(x, y, width, height);
});

defineEvent(
  "canvas.arc",
  (x, y, radius, startAngle, endAngle, anticlockwise) => {
    ctx.arc(x, y, radius, startAngle, endAngle, anticlockwise);
  }
);

defineEvent("canvas.beginPath", () => {
  ctx.beginPath();
});

defineEvent("canvas.closePath", () => {
  ctx.closePath();
});

defineEvent("canvas.moveTo", (x, y) => {
  ctx.moveTo(x, y);
});

defineEvent("canvas.lineTo", (x, y) => {
  ctx.lineTo(x, y);
});

defineEvent("canvas.quadraticCurveTo", (cpx, cpy, x, y) => {
  ctx.quadraticCurveTo(cpx, cpy, x, y);
});

defineEvent("canvas.bezierCurveTo", (cp1x, cp1y, cp2x, cp2y, x, y) => {
  ctx.bezierCurveTo(cp1x, cp1y, cp2x, cp2y, x, y);
});

defineEvent("canvas.arcTo", (x1, y1, x2, y2, radius) => {
  ctx.arcTo(x1, y1, x2, y2, radius);
});

defineEvent("canvas.save", () => {
  ctx.save();
});

defineEvent("canvas.restore", () => {
  ctx.restore();
});

defineEvent("canvas.clip", () => {
  ctx.clip();
});

defineEvent("canvas.rotate", (angle) => {
  ctx.rotate(angle);
});

defineEvent("canvas.scale", (x, y) => {
  ctx.scale(x, y);
});

defineEvent("canvas.translate", (x, y) => {
  ctx.translate(x, y);
});

defineEvent("canvas.transform", (a, b, c, d, e, f) => {
  ctx.transform(a, b, c, d, e, f);
});

defineEvent("canvas.setTransform", (a, b, c, d, e, f) => {
  ctx.setTransform(a, b, c, d, e, f);
});

defineEvent(
  "canvas.drawImage",
  (image, sx, sy, sWidth, sHeight, dx, dy, dWidth, dHeight) => {
    ctx.drawImage(image, sx, sy, sWidth, sHeight, dx, dy, dWidth, dHeight);
  }
);

defineEvent("canvas.createLinearGradient", (x0, y0, x1, y1) => {
  return ctx.createLinearGradient(x0, y0, x1, y1);
});

defineEvent("canvas.createRadialGradient", (x0, y0, r0, x1, y1, r1) => {
  return ctx.createRadialGradient(x0, y0, r0, x1, y1, r1);
});

defineEvent("canvas.createPattern", (image, repetition) => {
  return ctx.createPattern(image, repetition);
});

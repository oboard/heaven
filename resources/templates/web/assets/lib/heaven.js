function heaven_init() {
  WebAssembly.instantiateStreaming(
    fetch("assets/lib/app.wasm"),
    importObject
  ).then((obj) => {
    // 总是调用_start来初始化环境
    obj.instance.exports._start();
    // 将JS对象当作参数传递以绘制笑脸
    // obj.instance.exports["draw"](ctx);
    // 显示PI的值
    // obj.instance.exports["display_pi"]();
  });
}

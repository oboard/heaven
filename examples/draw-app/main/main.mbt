struct Offset {
  mut x : Double
  mut y : Double
}

let lastOffset : Offset = { x: 0, y: 0 }

fn main {
  let _ = send_event("log", @json.String("Hello, world!"))
  setFontSize(100)
  fillText("画图", 10, 100)
  addEventListener(
    "window",
    "mousemove",
    fn(events) {
      match events {
        @json.JsonValue::Array(
          [
            Object(
              { "buttons": Number(buttons), "x": Number(x), "y": Number(y) }
            ),
          ]
        ) => {
          if buttons == 1 {
            drawLine(lastOffset.x, lastOffset.y, x, y)
          }
          lastOffset.x = x
          lastOffset.y = y
        }
        _ => println(events)
      }
    },
  )
}

fn drawLine(x1 : Double, y1 : Double, x2 : Double, y2 : Double) -> Unit {
  ignore(send_event("canvas.beginPath", @json.Array([])))
  ignore(send_event("canvas.moveTo", @json.Array([Number(x1), Number(y1)])))
  ignore(send_event("canvas.lineTo", @json.Array([Number(x2), Number(y2)])))
  ignore(send_event("canvas.stroke", @json.Array([])))
}

fn setFontSize(size : Double) -> Unit {
  ignore(send_event("canvas.font.set", @json.String("\(size)px Arial")))
}

fn fillText(text : String, x : Double, y : Double) -> Unit {
  ignore(
    send_event(
      "canvas.fillText",
      @json.Array([String(text), Number(x), Number(y)]),
    ),
  )
}

// send_event("http.createServer", {  })
// send_event("http.listen", 4000)
// listen_event(
//   "http.request",
//   fn(req) {
//     send_event("http.writeHead", [200, { "Content-Type": "text/plain" }])
//     send_event("http.end", "Hello, World!")
//   },
// )

pub struct HeavenBinding {
  mut listen : (String, (@json.JsonValue) -> Unit) -> Unit
  mut send : (String, @json.JsonValue) -> Unit
}

pub struct HttpServer {
  port : Double
  mappings : Map[String, (@json.JsonValue, @json.JsonValue) -> String]
}

pub let binding : HeavenBinding = {
  listen: fn(_a, _b) { println("Please bind heaven.") },
  send: fn(_a, _b) { println("Please bind heaven.") },
}

pub fn listen(port : Double) -> HttpServer {
  let server = { port, mappings: of([]) }
  (binding.listen)(
    "http.request",
    fn(result) {
      match result {
        [req, res] =>
          match res {
            { "method": String(method), "url": String(url) } =>
              match server.mappings.get(method + " " + url) {
                Some(handler) => end(String(handler(req, res)))
                None => {
                  writeHead(404, { "Content-Type": "text/plain" })
                  end("Not Found")
                  // println("No handler found for " + method + " " + url)
                }
              }
            _ => println("Invalid request")
          }
        _ => println("Invalid request")
      }
    },
  )
  (binding.send)("http.createServer", {  })
  (binding.send)("http.listen", Number(server.port))
  server
}

pub fn get(
  self : HttpServer,
  mapping : String,
  handler : (@json.JsonValue, @json.JsonValue) -> String
) -> Unit {
  self.mappings["GET " + mapping] = handler
}

pub fn post(
  self : HttpServer,
  mapping : String,
  handler : (@json.JsonValue, @json.JsonValue) -> String
) -> Unit {
  self.mappings["POST " + mapping] = handler
}

pub fn put(
  self : HttpServer,
  mapping : String,
  handler : (@json.JsonValue, @json.JsonValue) -> String
) -> Unit {
  self.mappings["PUT " + mapping] = handler
}

pub fn delete(
  self : HttpServer,
  mapping : String,
  handler : (@json.JsonValue, @json.JsonValue) -> String
) -> Unit {
  self.mappings["DELETE " + mapping] = handler
}

pub fn all(
  self : HttpServer,
  mapping : String,
  handler : (@json.JsonValue, @json.JsonValue) -> String
) -> Unit {
  self.mappings["ALL " + mapping] = handler
}

pub fn writeHead(
  statusCode : @json.JsonValue,
  headers : @json.JsonValue
) -> Unit {
  (binding.send)("http.writeHead", [statusCode, headers])
}

pub fn end(data : @json.JsonValue) -> Unit {
  (binding.send)("http.end", data)
}

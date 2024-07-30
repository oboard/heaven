fn main {
  @mocket.binding.send = send_event
  @mocket.binding.listen = listen_event
  @mocket.listen(4000).get(
    "/",
    fn(_req, _res) {
      println("Received request for /")
      @mocket.writeHead(200, { "Content-Type": "text/plain" })
      "Hello, World!"
    },
  )
}

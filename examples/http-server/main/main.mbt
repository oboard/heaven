fn main {
  @mocket.listen(get_context(), 4000).get(
    "/",
    fn(_req, _res) {
      println("Received request for /")
      @mocket.writeHead(200, { "Content-Type": "text/plain" })
      "Hello, World!"
    },
  )
}

import http from "node:http";

export default class Mocket {
  init(heaven) {
    let response = undefined;
    let server = undefined;

    heaven.defineEvent("http.createServer", () => {
      console.log("Creating server");
      server = http.createServer((req, res) => {
        response = res;
        const callRequest = (data) => {
          heaven.callFunction("http.request", [
            {},
            {
              url: req.url,
              method: req.method,
              headers: req.headers,
              body: data,
            },
          ]);
        };

        // 检测请求方法是否为POST
        if (req.method === "POST") {
          let body = "";
          // 监听数据事件，将请求体数据累加到body变量
          req.on("data", (chunk) => {
            body += chunk.toString(); // 将Buffer转换为字符串
          });

          // 监听结束事件，完成body的接收
          req.on("end", () => {
            callRequest(body);
          });
        } else {
          callRequest();
        }
      });
    });

    heaven.defineEvent("http.listen", (port) => {
      if (!server) {
        throw new Error("Server not created");
      }
      server.listen(port, () => {
        console.log(`Server running on port ${port}`);
      });
    });

    heaven.defineEvent("http.writeHead", (statusCode, headers) => {
      if (!response) {
        throw new Error("Response not created");
      }
      response.writeHead(statusCode, headers);
    });

    heaven.defineEvent("http.end", (data) => {
      if (!response) {
        throw new Error("Response not created");
      }
      response.end(data);
    });
  }
}

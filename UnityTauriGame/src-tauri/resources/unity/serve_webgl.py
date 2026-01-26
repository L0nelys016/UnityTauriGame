# serve_webgl.py
import http.server
import socketserver
import os
from functools import partial

PORT = 8000
DIRECTORY = os.path.dirname(os.path.abspath(__file__))

class GzipHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    """
    HTTP сервер для Unity WebGL.
    Корректно отдает .gz файлы с Content-Encoding: gzip
    и правильными MIME типами.
    """

    def end_headers(self):
        if self.path.endswith(".gz"):
            self.send_header("Content-Encoding", "gzip")

            if self.path.endswith(".data.gz"):
                self.send_header("Content-Type", "application/octet-stream")

            elif self.path.endswith(".wasm.gz"):
                self.send_header("Content-Type", "application/wasm")

            elif (
                self.path.endswith(".framework.js.gz")
                or self.path.endswith(".loader.js")
                or self.path.endswith(".js.gz")
            ):
                self.send_header("Content-Type", "application/javascript")

        super().end_headers()


# ===== Запуск сервера =====
if __name__ == "__main__":
    handler = partial(GzipHTTPRequestHandler, directory=DIRECTORY)

    with socketserver.TCPServer(("localhost", PORT), handler) as httpd:
        print("===================================")
        print(" Unity WebGL local server started ")
        print("===================================")
        print(f"Directory : {DIRECTORY}")
        print(f"URL       : http://localhost:{PORT}/")
        print("Press Ctrl+C to stop")
        print("===================================")

        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\nServer stopped")
            httpd.server_close()

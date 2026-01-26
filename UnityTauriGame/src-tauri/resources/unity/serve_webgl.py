# serve_webgl.py
import http.server
import socketserver
import os
from functools import partial

PORT = 8000
DIRECTORY = os.path.dirname(os.path.abspath(__file__))

class GzipHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    """
    Обычный HTTP сервер, который правильно отдаёт .gz файлы с Content-Encoding: gzip
    для Unity WebGL сборки.
    """
    def end_headers(self):
        if self.path.endswith(".gz"):
            self.send_header("Content-Encoding", "gzip")
            # Unity WebGL ожидает правильный MIME тип
            if self.path.endswith(".data.gz"):
                self.send_header("Content-Type", "application/octet-stream")
            elif self.path.endswith(".wasm.gz"):
                self.send_header("Content-Type", "application/wasm")
            elif self.path.endswith(".framework.js.gz") or self.path.endswith(".loader.js"):
                self.send_header("Content-Type", "application/javascript")
        super().end_headers()

# Запуск сервера
handler = partial(GzipHTTPRequestHandler, directory=DIRECTORY)
with socketserver.TCPServer(("", PORT), handler) as httpd:
    print(f"Serving Unity WebGL from: {DIRECTORY}")
    print(f"Open: http://localhost:{PORT}/")
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("\nServer stopped")
        httpd.server_close()

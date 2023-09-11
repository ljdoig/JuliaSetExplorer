# -*- coding: utf-8 -*-
import http.server
import socketserver
import socket
import os
FILES_LOCATION='web_app'
PORT = 8080
web_dir = os.path.join(os.path.dirname(__file__), FILES_LOCATION)
os.chdir(web_dir)

class HttpRequestHandler(http.server.SimpleHTTPRequestHandler):
    extensions_map = {
        '': 'application/octet-stream',
        '.manifest': 'text/cache-manifest',
        '.html': 'text/html',
        '.png': 'image/png',
        '.jpg': 'image/jpg',
        '.svg': 'image/svg+xml',
        '.css': 'text/css',
        '.js': 'application/x-javascript',
        '.wasm': 'application/wasm',
        '.json': 'application/json',
        '.xml': 'application/xml',
    }

    def end_headers(self):
        """
        Overrides the SimpleHTTPRequestHandler to
        send extra headers.
        """
        self.send_safe_context_headers()
        super().end_headers()

    def send_safe_context_headers(self):
        """
        Allows SharedArrayBuffer to work from our demo server.
        """
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")

with socketserver.TCPServer(("localhost", PORT), HttpRequestHandler) as httpd:
    try:
        print(f"serving {str(web_dir)} at http://localhost:{PORT}")
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("[!] Keyboard Interrupted!")
        httpd.server_close()
        httpd.shutdown()


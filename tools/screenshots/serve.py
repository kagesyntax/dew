import http.server, os, sys
ROOT = sys.argv[1]; PORT = int(sys.argv[2])
class H(http.server.SimpleHTTPRequestHandler):
    def __init__(s, *a, **k): super().__init__(*a, directory=ROOT, **k)
    def send_head(s):
        p = s.translate_path(s.path)
        if not os.path.exists(p): s.path = '/index.html'
        return super().send_head()
    def log_message(s, *a): pass
H.extensions_map['.wasm'] = 'application/wasm'
http.server.ThreadingHTTPServer(('127.0.0.1', PORT), H).serve_forever()

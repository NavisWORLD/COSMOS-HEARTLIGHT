#!/usr/bin/env python3
"""Serve HEARTLIGHT locally with Python 3.10+.
Camera access on desktop works on localhost in modern browsers.
For phone installation, deploy app/ to an HTTPS host or package as a native shell.
"""
from http.server import ThreadingHTTPServer, SimpleHTTPRequestHandler
from pathlib import Path
import argparse, os, socket

ROOT = Path(__file__).resolve().parent / "app"

def free_port(start: int) -> int:
    for port in range(start, start + 50):
        with socket.socket() as s:
            try:
                s.bind(("127.0.0.1", port))
                return port
            except OSError:
                continue
    raise RuntimeError("No free local port found")

parser = argparse.ArgumentParser()
parser.add_argument("--port", type=int, default=8877)
args = parser.parse_args()
port = free_port(args.port)
os.chdir(ROOT)
server = ThreadingHTTPServer(("127.0.0.1", port), SimpleHTTPRequestHandler)
print(f"COSMOS HEARTLIGHT -> http://127.0.0.1:{port}")
try:
    server.serve_forever()
except KeyboardInterrupt:
    pass
finally:
    server.server_close()

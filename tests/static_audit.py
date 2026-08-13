from pathlib import Path
import json, re, sys

ROOT = Path(__file__).resolve().parents[1]
html = (ROOT/'app/index.html').read_text()
manifest = json.loads((ROOT/'app/manifest.webmanifest').read_text())

required = [
    'What do you need?', 'Teacher / aide observation', 'Ambient color / light sensor',
    'CST 12D support-state', 'Erase all HEARTLIGHT data', 'getUserMedia'
]
for text in required:
    assert text in html, f'missing: {text}'

# Privacy-sensitive APIs should not be present in the reference UI.
for forbidden in ['MediaRecorder(', 'SpeechRecognition(', 'geolocation.', 'RTCPeerConnection(', 'WebSocket(']:
    assert forbidden not in html, f'forbidden API present: {forbidden}'

# Core page does not send data via fetch/XHR/beacon.
assert 'fetch(' not in html
assert 'XMLHttpRequest' not in html
assert 'sendBeacon' not in html

assert manifest['display'] == 'standalone'
assert manifest['start_url'] == './index.html'

print('HEARTLIGHT static privacy/function audit: PASS')

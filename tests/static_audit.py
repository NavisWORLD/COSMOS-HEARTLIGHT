from pathlib import Path
import json

ROOT = Path(__file__).resolve().parents[1]
html = (ROOT/'app/index.html').read_text()
i18n = (ROOT/'app/i18n.js').read_text()
manifest = json.loads((ROOT/'app/manifest.webmanifest').read_text())

required = [
    'What do you need?', 'Teacher / aide observation', 'Ambient color / light sensor',
    'CST 12D support-state', 'Erase all HEARTLIGHT data', 'getUserMedia',
    '<script src="i18n.js"></script>'
]
for text in required:
    assert text in html, f'missing: {text}'

for text in [
    'Español', '¿Qué necesitas?', 'Guardar localmente', 'Sensor de color / luz ambiental',
    'Borrar todos los datos de HEARTLIGHT', 'heartlight.language'
]:
    assert text in i18n, f'missing Spanish/offline localization feature: {text}'

# Privacy-sensitive APIs should not be present in the reference UI.
for forbidden in ['MediaRecorder(', 'SpeechRecognition(', 'geolocation.', 'RTCPeerConnection(', 'WebSocket(']:
    assert forbidden not in html, f'forbidden API present: {forbidden}'
    assert forbidden not in i18n, f'forbidden API present in localization layer: {forbidden}'

# Core page and localization layer do not send data via fetch/XHR/beacon.
for source in [html, i18n]:
    assert 'fetch(' not in source
    assert 'XMLHttpRequest' not in source
    assert 'sendBeacon' not in source

assert manifest['display'] == 'standalone'
assert manifest['start_url'] == './index.html'

print('HEARTLIGHT static privacy/function/i18n audit: PASS')

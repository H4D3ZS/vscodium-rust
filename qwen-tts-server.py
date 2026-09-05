#!/usr/bin/env python3
"""
Qwen3-TTS HTTP Server
Simple HTTP API for text-to-speech using Qwen3-TTS
"""

import http.server
import socketserver
import json
import io
import wave
import struct
import sys
from pathlib import Path
from typing import Optional

# Windows defaults to cp1252; box-drawing / emoji in prints raise UnicodeEncodeError.
if sys.platform == "win32":
    try:
        sys.stdout.reconfigure(encoding="utf-8", errors="replace")
        sys.stderr.reconfigure(encoding="utf-8", errors="replace")
    except Exception:
        pass

# Import Qwen3-TTS
try:
    from qwen_tts import QwenTTS
    HAS_QWEN = True
except ImportError:
    print(" Qwen3-TTS not installed. Using mock TTS.")
    HAS_QWEN = False

PORT = 8081  # Changed from 8080 (conflict with PostgreSQL/Redis)

class QwenTTSHandler(http.server.SimpleHTTPRequestHandler):
    tts_engine = None

    def end_headers(self):
        # Add CORS headers
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        super().end_headers()

    def do_OPTIONS(self):
        # Handle CORS preflight
        self.send_response(200)
        self.end_headers()

    def __init__(self, *args, **kwargs):
        if not QwenTTSHandler.tts_engine and HAS_QWEN:
            QwenTTSHandler.tts_engine = QwenTTS()
        super().__init__(*args, **kwargs)
    
    def do_GET(self):
        if self.path == '/health':
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()
            response = json.dumps({
                'status': 'healthy',
                'has_qwen': HAS_QWEN,
                'port': PORT
            })
            self.wfile.write(response.encode())
        else:
            self.send_response(404)
            self.end_headers()
    
    def do_POST(self):
        if self.path == '/tts':
            content_length = int(self.headers['Content-Length'])
            post_data = self.rfile.read(content_length)
            
            try:
                data = json.loads(post_data.decode('utf-8'))
                text = data.get('text', '')
                emotion = data.get('emotion', 'neutral')
                speed = data.get('speed', 1.0)
                pitch = data.get('pitch', 1.0)
                
                print(f" TTS Request: '{text[:50]}...' (emotion={emotion})")
                
                # Generate speech
                if HAS_QWEN and QwenTTSHandler.tts_engine:
                    # Use Qwen3-TTS
                    audio_data = QwenTTSHandler.tts_engine.synthesize(
                        text=text,
                        emotion=emotion,
                        speed=speed,
                        pitch=pitch
                    )
                else:
                    # Mock audio (silence) for testing
                    print(" Using mock TTS (Qwen3-TTS not available)")
                    audio_data = self.generate_mock_audio()
                
                # Send audio response
                self.send_response(200)
                self.send_header('Content-type', 'audio/wav')
                self.end_headers()
                
                # Write WAV file
                with io.BytesIO() as wav_buffer:
                    with wave.open(wav_buffer, 'wb') as wav_file:
                        wav_file.setnchannels(1)
                        wav_file.setsampwidth(2)
                        wav_file.setframerate(22050)
                        wav_file.writeframes(audio_data)
                    
                    self.wfile.write(wav_buffer.getvalue())
                
                print(f" Sent {len(audio_data)} bytes of audio")
                
            except Exception as e:
                print(f" TTS Error: {e}")
                self.send_response(500)
                self.send_header('Content-type', 'application/json')
                self.end_headers()
                error = json.dumps({'error': str(e)})
                self.wfile.write(error.encode())
        else:
            self.send_response(404)
            self.end_headers()
    
    def generate_mock_audio(self, duration_sec: float = 1.0) -> bytes:
        """Generate silent audio for testing"""
        sample_rate = 22050
        num_samples = int(sample_rate * duration_sec)
        return struct.pack('<' + 'h' * num_samples, *[0] * num_samples)
    
    def log_message(self, format, *args):
        # Suppress default logging
        pass

def run_server():
    print(f"╔══════════════════════════════════════════════════════════╗")
    print(f"║         Qwen3-TTS HTTP Server                            ║")
    print(f"╚══════════════════════════════════════════════════════════╝")
    print(f"")
    print(f" Port: {PORT}")
    print(f" URL: http://localhost:{PORT}")
    print(f" Qwen3-TTS: {' Available' if HAS_QWEN else ' Not installed'}")
    print(f"")
    print(f"Endpoints:")
    print(f"  GET  /tts   - Generate speech (POST with JSON body)")
    print(f"  GET  /health - Health check")
    print(f"")
    print(f"Example usage:")
    print(f'  curl -X POST http://localhost:{PORT}/tts \\')
    print(f'    -H "Content-Type: application/json" \\')
    print(f'    -d \'{{"text": "Hello, I am AIRI"}}\'')
    print(f"")
    print(f"Press Ctrl+C to stop")
    print(f"")
    
    with socketserver.TCPServer(("", PORT), QwenTTSHandler) as httpd:
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\n\n Shutting down server...")
            httpd.shutdown()

if __name__== "__main__":
    run_server()

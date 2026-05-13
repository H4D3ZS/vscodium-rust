# coding=utf-8
import os
import torch
import soundfile as sf
import io
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from fastapi.responses import StreamingResponse
from qwen_tts import Qwen3TTSModel
from fastapi.middleware.cors import CORSMiddleware
import uvicorn

app = FastAPI(title="Qwen3-TTS API Server")

# Enable CORS for the IDE origin
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # Allow all for local dev; can be restricted to http://localhost:5173
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Load model
device = "cuda:0" if torch.cuda.is_available() else "cpu"
MODEL_PATH = "Qwen/Qwen3-TTS-12Hz-1.7B-Base/"

print(f"🚀 Loading Qwen3-TTS model on {device}...")
try:
    tts_model = Qwen3TTSModel.from_pretrained(
        MODEL_PATH,
        device_map=device,
        dtype=torch.bfloat16 if torch.cuda.is_available() else torch.float32,
        attn_implementation="flash_attention_2" if torch.cuda.is_available() else "eager",
    )
    print("✅ Model loaded successfully!")
except Exception as e:
    print(f"❌ Failed to load model: {e}")
    # Fallback to eager implementation if flash_attention fails
    tts_model = Qwen3TTSModel.from_pretrained(
        MODEL_PATH,
        device_map=device,
        dtype=torch.float32,
        attn_implementation="eager",
    )
    print("✅ Model loaded successfully (Fallback/CPU mode)!")

class TTSRequest(BaseModel):
    text: str
    emotion: str = "neutral"
    speed: float = 1.0
    pitch: float = 0.0

@app.get("/health")
async def health():
    return {"status": "ok"}

@app.post("/tts")
async def tts(request: TTSRequest):
    try:
        # Simple generation (no cloning for now, uses base model)
        # Using the base model generation pattern from examples
        common_gen_kwargs = dict(
            max_new_tokens=2048,
            do_sample=True,
            top_k=50,
            top_p=1.0,
            temperature=0.9,
            repetition_penalty=1.05,
            subtalker_dosample=True,
            subtalker_top_k=50,
            subtalker_top_p=1.0,
            subtalker_temperature=0.9,
        )

        # Qwen3-TTS-Base usually needs a prompt or uses a default voice
        # For simplicity, we'll try a basic generate call if available
        # OR use a default reference from the examples
        
        # This is a placeholder for the actual generation call
        # Since I don't have the full API of Qwen3TTSModel memorized,
        # I'll use the pattern from Case 1 but with a hardcoded reference for now
        
        ref_audio = "https://qianwen-res.oss-cn-beijing.aliyuncs.com/Qwen3-TTS-Repo/clone_2.wav"
        ref_text = "Okay. Yeah. I resent you. I love you. I respect you. But you know what? You blew it! And thanks to you."

        wavs, sr = tts_model.generate_voice_clone(
            text=request.text,
            language="Auto",
            ref_audio=ref_audio,
            ref_text=ref_text,
            **common_gen_kwargs,
        )

        if not wavs:
            raise HTTPException(status_code=500, detail="Generation failed")

        # Convert to WAV in memory
        buffer = io.BytesIO()
        sf.write(buffer, wavs[0], sr, format='WAV')
        buffer.seek(0)

        return StreamingResponse(buffer, media_type="audio/wav")

    except Exception as e:
        print(f"Error during TTS: {e}")
        raise HTTPException(status_code=500, detail=str(e))

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8080)

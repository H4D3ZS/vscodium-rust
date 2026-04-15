# Custom AI Avatar Guide

You can now customize the visual appearance of AIRI (your AI assistant) with your own avatar images and 3D models!

## Features

✅ **Select from pre-loaded characters** - Choose from 8 built-in avatar personalities  
✅ **Custom 2D URL avatars** - Use any PNG image URL as your avatar  
✅ **3D VRM Avatars** - Load custom 3D anime models in the AIRI panel  
✅ **Persistent storage** - Your custom avatars persist across reloads  
✅ **Sticker + Wallpaper** - Configure both avatar overlay and background  

## Types of Avatars

### 1. 2D Avatar (Background Sticker/Wallpaper)
This is the anime character overlay that appears as a background in the IDE.

**How to Configure:**
1. Open Settings → Click the gear icon in the sidebar
2. Find "AI Character" section
3. Scroll to "Custom Avatar URLs"
4. Enable "Enable Custom" checkbox
5. Enter URLs:
   - **Sticker URL**: Your avatar image (PNG with transparency recommended)
   - **Wallpaper URL**: Background image (optional)
6. Click "Save Custom Avatar"

### 2. 3D VRM Avatar (Airi Panel)
This is the interactive 3D anime avatar that appears in the AIRI panel (localhost:5174).

**How to Configure:**
1. Open Settings → Click the gear icon
2. Find "AI Character" section
3. Scroll to "3D VRM Avatar (Airi Panel)"
4. Choose one of these options:

   **A. Pre-loaded Models:**
   - Click any of the built-in models (AIRI, Sage, Nova, etc.)
   
   **B. Custom VRM URL:**
   - Enter a direct URL to a .vrm file
   - Must be publicly accessible or from a local server
   
   **C. Saved Custom Models:**
   - Add custom models with name and URL
   - Switch between them easily
   - Delete models you no longer need

5. The 3D avatar updates immediately in the AIRI panel

## Pre-loaded 3D Models (Already Cached!)

The following models are **already downloaded and cached** in your AIRI installation - ready to use immediately:

| Model ID | Name | Type | Description |
|----------|------|------|-------------|
| `hiyori_pro` | Hiyori Pro | Live2D | Professional version with full expressions |
| `hiyori_free` | Hiyori Free | Live2D | Free version of Hiyori model |
| `avatar_a` | Avatar Sample A | VRM | Sample 3D VRM model (Type A) |
| `avatar_b` | Avatar Sample B | VRM | Sample 3D VRM model (Type B) |
| `airi` | AIRI Default | VRM | Primary avatar - energetic anime AI |
| `sage` | Sage | VRM | Mature assistant - calm & wise |

**✅ No download required!** These models are stored locally at:
```
C:\Users\HADES\Desktop\vscodium-rust\airi\packages\stage-ui\src\assets\
```

## Recommended Image/Model Specifications

### 2D Sticker (Avatar Overlay)
- **Format**: PNG with transparency
- **Size**: 512x512px (or square aspect ratio)
- **Position**: Bottom-right corner overlay
- **Example**: Character portrait with transparent background

### 2D Wallpaper (Background)
- **Format**: PNG or JPG
- **Size**: 1920x1080px or similar
- **Position**: Full background
- **Example**: Scenic anime background, gradient, pattern

### 3D VRM Model
- **Format**: .vrm (VRM 0.x or 1.0)
- **Source**: 
  - [VRM Hub](https://hub.vrm.dev/)
  - [Booth.pm](https://booth.pm/) (search "VRM")
  - Custom made with Blender + VRM exporter
- **Hosting**: Must be served from a URL (GitHub Pages, Imgur for VRM, local server)

## Example URLs to Try

### 2D Avatar URLs
```
https://example.com/your-character.png
https://example.com/background.png
```

### 3D VRM Model URLs
```
https://example.com/models/my-avatar.vrm
http://localhost:8080/models/custom.vrm
```

## How to Host VRM Files

### Option 1: GitHub Pages
1. Create a GitHub repository
2. Upload your .vrm file
3. Enable GitHub Pages
4. Use the raw URL: `https://username.github.io/repo/model.vrm`

### Option 2: Local Server
```bash
# Serve your VRM files locally
cd /path/to/vrm-files
python -m http.server 8080
# Then use: http://localhost:8080/model.vrm
```

### Option 3: Cloud Storage
- Upload to Google Drive, Dropbox, etc.
- Get a direct download link
- Ensure CORS is enabled

## Troubleshooting

**3D Avatar not showing?**
- Verify the AIRI panel is running (localhost:5174)
- Check that the VRM URL is accessible (opens in browser)
- Ensure CORS headers allow cross-origin requests
- Try reloading the window (`Ctrl+Shift+P` → "Reload Window")

**2D Avatar not showing?**
- Check that the URL is accessible (opens in browser)
- Ensure it's a direct image link (ends in .png/.jpg)
- Verify "Enable Custom" is checked
- Try reloading the window

**Want to revert to default?**
- For 2D: Uncheck "Enable Custom" in Custom Avatar URLs
- For 3D: Select a pre-loaded model from the grid

## Advanced Tips

### Multiple Avatar Profiles
You can save multiple 3D model configurations and switch between them:
1. Add multiple custom models in the 3D VRM section
2. Click "Use" on any saved model to activate it
3. Changes apply immediately to the AIRI panel

### Layering Avatars
- **2D Avatar** appears as a background overlay in the IDE
- **3D Avatar** appears in the interactive AIRI panel
- Both can be configured independently!

### Integration with Voice
Your avatar works independently from voice selection:
- **Avatar** = Visual appearance (2D or 3D)
- **Voice** = Audio output (ElevenLabs voice selection)
- Configure both separately in Settings for a complete personalized experience!

## 🎤 ElevenLabs Voice Features

### Natural, Human-Like Speech

The TTS system now uses **advanced sentence splitting** for smooth, natural speech:

✅ **Intelligent text processing**:
- Removes markdown formatting automatically
- Handles abbreviations (Mr, Dr, etc.) without awkward pauses
- Splits on natural sentence boundaries
- Preserves proper grammar and phrasing
- Filters overly long segments for better flow

✅ **ElevenLabs streaming**:
- Real-time audio streaming for low latency
- Sentence-by-sentence playback for natural rhythm
- Optimized for conversational speech patterns

### How to Select & Save Your Voice

1. **Open Settings** → Scroll to "Voice & TTS (AIRI Speech)"
2. **Find "ElevenLabs Voices"** section
3. **Browse available voices** - filtered by category (premade, cloned, generated)
4. **Preview voices** - Click the ▶️ play icon to hear a sample
5. **Select a voice** - Click on the voice name to preview it
6. **Click "Save"** button next to your chosen voice
   - Button shows: **Save** → **Saving...** → **Saved ✓**
   - Voice is now marked as **✓ SELECTED**
   - Saved to persistent storage
7. **Done!** AI will now speak with your selected voice

### Voice Selection UI Features

- **Visual feedback**: Selected voice highlighted in green with checkmark
- **Save confirmation**: Button shows saving progress and confirmation
- **Persistent storage**: Voice saved across reloads
- **Preview playback**: Listen to voice samples before selecting
- **Filter categories**: Browse premade, cloned, or generated voices
- **Metadata tags**: See voice characteristics (age, accent, gender)

### Popular ElevenLabs Voices

| Voice Name | Style | Best For |
|------------|-------|----------|
| **Jessica** | Warm, friendly | Conversational AI, narration |
| **Adam** | Deep, authoritative | Professional content |
| **Rachel** | Clear, neutral | General purpose |
| **Domi** | Energetic, young | Casual, friendly tone |
| **Bella** | Soft, gentle | Calm, soothing content |
| **Antoni** | Confident, clear | Explanations, tutorials |

### Troubleshooting Voice Issues

**Voice sounds robotic/static:**
- Check console logs for `provider=elevenlabs`
- Verify API key is valid (starts with `sk_`)
- Ensure voice ID is saved correctly

**Multiple voices talking:**
- Check logs for duplicate `speak()` calls
- May indicate both AiriPanel and RightSidebar triggering TTS

**Voice not saving:**
- Click the **Save** button explicitly (don't just click voice name)
- Wait for "Saved ✓" confirmation
- Check `api_keys.json` for `elevenlabs_voice_id` field

---

**Enjoy your personalized AI companion with natural, human-like speech! 🎨✨🎮🎤**

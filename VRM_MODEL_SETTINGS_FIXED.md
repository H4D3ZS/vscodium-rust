# ✅ VRM Model Selection - Now in Main Settings!

## What Was Fixed

**Problem**: VRM model selection was only accessible inside the AIRI iframe/preview UI, not from the main IDE settings.

**Solution**: Added model selection controls directly to the main IDE Settings panel with an "Apply Model" button.

---

## How to Use

### 1. Open Settings
Click the **gear icon** in the activity bar (left sidebar)

### 2. Scroll to "3D VRM Avatar Configuration"
You'll see:
- 12 preset VRM models in a 4-column grid
- Custom VRM URL input field
- **"Apply Model"** button (purple gradient)

### 3. Select a Model
Click any of the 12 preset models:
- **Hiyori Pro/Free** - Professional Live2D
- **Avatar A/B** - VRM samples
- **AIRI Default** - Default avatar
- **Sage** - Mature assistant
- **Nova** - Energetic & futuristic
- **Kawaii** - Cute & adorable
- **Sentinel** - Security-focused
- **Oracle** - All-knowing
- **Phantom** - Mysterious
- **Titan** - Powerful & strong

### 4. Click "Apply Model"
The AIRI panel will reload with your selected model!

---

## Files Modified

### 1. `src/components/AgentSettingsView.tsx`
**Added**:
- "Apply Model" button in VRM configuration section
- Event dispatch to notify AiriPanel of model change
- Visual feedback with purple gradient button

**Code**:
```typescript
<button
    onClick={() => {
        window.dispatchEvent(new CustomEvent('airi-vrm-model-change', { 
            detail: { modelId: vrmModelId, modelUrl: vrmModelUrl } 
        }));
    }}
    style={{
        background: 'linear-gradient(135deg, #c084fc 0%, #a855f7 100%)',
        // ...styling
    }}
>
    Apply Model
</button>
```

### 2. `src/components/AiriPanel.tsx`
**Added**:
- Event listener for `airi-vrm-model-change`
- Iframe reload mechanism with timestamp to force refresh
- Loading state management during model switch

**Code**:
```typescript
const handleModelChange = (e: any) => {
    console.log('[AiriPanel] 🎭 Model change requested:', e.detail);
    setAiriLoading(true);
    const iframe = iframeRef.current;
    if (iframe) {
        const newUrl = url + `&t=${Date.now()}`;
        iframe.src = newUrl;
    }
};

window.addEventListener('airi-vrm-model-change', handleModelChange);
```

---

## How It Works

```
User clicks model in Settings
    ↓
Settings dispatches 'airi-vrm-model-change' event
    ↓
AiriPanel listens for event
    ↓
AiriPanel reloads iframe with new model URL
    ↓
New VRM model loads in AIRI panel
```

---

## Features

### ✅ Easy Access
- No need to open AIRI's internal settings
- All controls in main IDE settings
- One-click model switching

### ✅ 12 Preset Models
- All models cached locally
- No download required
- Instant switching

### ✅ Custom Models
- Enter custom VRM URL
- Supports VRM 0.x and 1.0
- Must be publicly accessible URL

### ✅ Visual Feedback
- Selected model highlighted with purple border
- "Apply Model" button with gradient
- Loading indicator during model switch

---

## Testing

1. **Open Settings** (gear icon)
2. **Scroll to "3D VRM Avatar Configuration"**
3. **Click "Sage"** (or any model)
4. **Click "Apply Model"**
5. **Watch AIRI panel** - model should switch!

**Expected Console Output**:
```
[VRM] Model change signal sent: {modelId: 'sage', modelUrl: undefined}
[AiriPanel] 🎭 Model change requested: {modelId: 'sage', modelUrl: undefined}
```

---

## Status

| Feature | Status |
|---------|--------|
| Settings UI | ✅ Complete |
| Model Grid (12 models) | ✅ Complete |
| Custom URL Input | ✅ Complete |
| Apply Button | ✅ Complete |
| Iframe Reload | ✅ Complete |
| Loading State | ✅ Complete |

**VRM model selection is now fully accessible from main IDE settings!** 🎉

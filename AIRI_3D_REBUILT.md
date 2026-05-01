# ✅ AIRI 3D App Rebuilt with URL Model Switching!

## What Was Added

**Feature**: URL-based character/model switching for external integration

**Modified File**: `airi/apps/stage-web/src/main.ts`

### How It Works

When the AIRI 3D app loads, it now checks URL parameters:
- `char` - Character/model ID to load
- `modelUrl` - Custom VRM model URL

### Supported Character IDs

| ID | Maps To | Description |
|----|---------|-------------|
| `hiyori_pro` | `preset-live2d-1` | Hiyori Pro (Live2D) |
| `hiyori_free` | `preset-live2d-2` | Hiyori Free (Live2D) |
| `avatar_a` | `preset-vrm-1` | Avatar Sample A (VRM) |
| `avatar_b` | `preset-vrm-2` | Avatar Sample B (VRM) |
| `airi` | `preset-live2d-1` | AIRI Default |
| `sage` | `preset-live2d-3` | Sage |
| `nova` | `preset-live2d-4` | Nova |
| `kawaii` | `preset-live2d-5` | Kawaii |
| `sentinel` | `preset-live2d-6` | Sentinel |
| `oracle` | `preset-live2d-7` | Oracle |
| `phantom` | `preset-live2d-8` | Phantom |
| `titan` | `preset-live2d-9` | Titan |

---

## How to Use

### From VSCodium-Rust IDE

1. Open Settings (gear icon)
2. Scroll to "3D VRM Avatar (Airi Panel)"
3. Select a model (e.g., "Sage", "Nova", "Kawaii")
4. Click "APPLY MODEL"
5. AIRI panel reloads with new model!

### Direct URL Access

Open in browser:
```
http://localhost:5174/?headless=true&char=sage
```

Or with custom model:
```
http://localhost:5174/?headless=true&modelUrl=https://example.com/model.vrm
```

---

## Implementation Details

### Code Added to `main.ts`

```typescript
// Check URL parameters for model/character selection
const urlParams = new URLSearchParams(window.location.search)
const charParam = urlParams.get('char')
const modelUrlParam = urlParams.get('modelUrl')

if (charParam || modelUrlParam) {
  console.log('[Main] 🎭 URL model params detected:', { 
    char: charParam, 
    modelUrl: modelUrlParam 
  })
  
  // Wait for pinia to be ready, then set the model
  setTimeout(() => {
    import('@proj-airi/stage-ui/stores/settings/stage-model')
      .then(({ useSettingsStageModel }) => {
        const store = useSettingsStageModel(pinia)
        
        if (charParam) {
          const mappedId = charMap[charParam] || charParam
          store.stageModelSelected = mappedId
          console.log('[Main] ✅ Character set to:', mappedId)
        }
        
        if (modelUrlParam) {
          store.replaceStageModelUrl(modelUrlParam)
          console.log('[Main] ✅ Model URL set to:', modelUrlParam)
        }
      })
  }, 500)
}
```

### Integration Flow

```
VSCodium-Rust Settings
    ↓
User clicks "Apply Model"
    ↓
AiriPanel.tsx sends event
    ↓
Iframe reloads with new URL params
    ↓
AIRI 3D App reads `char` param
    ↓
Maps to preset ID (e.g., "sage" → "preset-live2d-3")
    ↓
Sets store.stageModelSelected
    ↓
AIRI loads new model
    ↓
Avatar changes! ✅
```

---

## Testing

### Test in Browser

1. Open: `http://localhost:5174/?headless=true&char=hiyori_free`
2. Should see Hiyori Free model
3. Try: `http://localhost:5174/?headless=true&char=avatar_a`
4. Should see Avatar Sample A (VRM)

### Test from IDE

1. Open VSCodium-Rust
2. Open Settings (gear icon)
3. Scroll to bottom right
4. Click "Sage" model
5. Click "APPLY MODEL"
6. Watch AIRI panel switch to Sage model!

---

## Console Output

When model changes successfully:
```
[AiriPanel] 🎭 Model change requested: {modelId: 'sage', modelUrl: undefined}
[AiriPanel] Setting character: sage
[AiriPanel] 🔄 Reloading iframe with new model: http://localhost:5174/?headless=true&transparent=true&char=sage&t=1777289181252
[AiriPanel] ✅ Sent postMessage to AIRI app

// In AIRI app console:
[Main] 🎭 URL model params detected: {char: 'sage', modelUrl: null}
[Main] ✅ Character set to: preset-live2d-3
```

---

## Files Modified

1. **`airi/apps/stage-web/src/main.ts`**
   - Added URL parameter parsing
   - Added character ID mapping
   - Added dynamic store import
   - Sets model on app initialization

2. **`src/components/AiriPanel.tsx`** (previous fix)
   - Added model change event listener
   - Reloads iframe with new parameters

3. **`src/components/AgentSettingsView.tsx`** (previous fix)
   - Added "Apply Model" button
   - Dispatches model change event

---

## Build Status

✅ **AIRI 3D App**: Built successfully  
✅ **Bundle Size**: 146MB (includes all models)  
✅ **PWA**: 576 entries precached  
✅ **Build Time**: 27.98s

---

## Next Steps

The model switching is now **fully functional**!

To use:
1. **In IDE**: Settings → Select model → Apply Model
2. **In Browser**: Add `?char=modelname` to URL
3. **Programmatically**: Send postMessage to iframe

**All 12 character models are now switchable from the IDE settings!** 🎉

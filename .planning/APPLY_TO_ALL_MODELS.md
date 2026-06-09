# "Apply to All Specialist Engines" Feature

## Overview
Added a checkbox in the Model Selector panel that allows users to apply a selected model to ALL APEX specialist engines at once, instead of configuring them individually.

## Changes Made

### Backend (Rust)

**1. New Command: `apply_model_to_all_engines`** (`src-tauri/src/model_commands.rs`)
```rust
pub async fn apply_model_to_all_engines(
    state: State<'_, EditorState>,
    model_name: String,
) -> Result<Value, String>
```

- Verifies the model exists in Ollama
- Applies to all 7 specialist engines:
  - architect
  - threat
  - perf
  - self_improve
  - explainer
  - multi_system
  - predictor
- Also sets as the current model
- Returns success JSON with engine count

**2. Command Registration** (`src-tauri/src/lib.rs`)
- Added `model_commands::apply_model_to_all_engines` to the command handler list

### Frontend (React)

**1. Updated ModelSelectorPanel.tsx**
- Added `applyToAll` state variable (checkbox state)
- Added checkbox UI with label:
  ```
  ☑ Apply to all specialist engines (architect, threat, perf, etc.)
  ```
- Modified `handleSelectModel()` to:
  - Call `apply_model_to_all_engines` when checkbox is checked
  - Call `set_current_model` when checkbox is unchecked (individual model)

## Usage

### Before
1. User selects a model (e.g., qwen3.5:12b)
2. Each APEX engine still uses its default model assignment
3. User had to manually configure each engine separately

### After
1. User checks: "Apply to all specialist engines"
2. User selects a model (e.g., qwen3.5:12b)
3. **All 7 specialist engines automatically use that model**
4. Consistency across all AI operations

## Benefits

✅ **Simplicity**: One click instead of configuring 7 engines separately  
✅ **Consistency**: All specialist engines use the same model  
✅ **Performance**: Optimized for 12b and below models on 8GB RAM  
✅ **Offline**: No external API calls needed  

## User Workflow

```
Settings → Model Selection
  ☑ Apply to all specialist engines
  Select: qwen3.5:12b
  → architect, threat, perf, self_improve, explainer, multi_system, predictor all use qwen3.5:12b
```

## Backward Compatibility

- Checkbox defaults to **unchecked** (individual model mode)
- Users can still configure individual engines by leaving checkbox unchecked
- Existing model overrides are preserved

## Testing Checklist

- [ ] Model selection panel loads without errors
- [ ] Checkbox toggles correctly (visual feedback)
- [ ] With checkbox **unchecked**: selecting model sets only current model
- [ ] With checkbox **checked**: selecting model applies to all engines
- [ ] Multiple model selections work correctly
- [ ] Auto-detect button also respects the checkbox
- [ ] Refresh page: setting persists or resets appropriately

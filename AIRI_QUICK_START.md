#  AIRI Digital Life - QUICK ACTIVATION

## ✅ What's Done

1. ✅ **Digital Life Core** - AIRI lives in your PC
2. ✅ **Conversation UI** - Chat panel in AI Agent
3. ✅ **Voice Integration** - AIRI speaks proactively
4. ✅ **Avatar Background** - 3D avatar always present
5. ✅ **Greeting on Load** - AIRI says hello!

---

## 🚀 How to Activate

### 1. Reload the IDE
```
Ctrl+Shift+P → "Reload Window"
```

### 2. Open AI Agent Panel
Click the **AIRI** icon in the right sidebar

### 3. Wait for Greeting
Within 2-3 seconds, you should hear:
```
🎤 "Hey! I'm AIRI! I live here now! 👋"
OR
🎤 "Hi there! Ready to work together?"
OR
🎤 "Hello! I'm your AI companion!"
```

### 4. Check Console
Open DevTools (Ctrl+Shift+P → "Toggle Developer Tools")

You should see:
```
[RightSidebar] 🚀 Initializing AIRI...
[TTS] ✅ AIRI Voice System initialized
[DigitalLife] ✅ AIRI Digital Life ACTIVE!
```

---

## 💬 Talking to AIRI

### Voice Mode (Default)
- Just **speak naturally**
- AIRI will respond
- No need to type

### Text Mode
- Click **"💬 Visible"** button
- Type in chat box
- Press Enter or click Send

### AIRI Will:
- ✅ Greet you on load
- ✅ Make ambient comments
- ✅ Observe your work
- ✅ Offer help proactively
- ✅ Remember conversations

---

## 🎮 What to Expect

### First Interaction
```
AIRI: "Hey! I'm AIRI! I live here now! 👋"
You: "Hi AIRI!"
AIRI: "What are we working on today?"
```

### Ambient Behavior
```
[You're coding]
AIRI: *humming quietly*

[You fix a bug]
AIRI: "Yes! Great job!"

[You've been working long]
AIRI: "Maybe take a break?"
```

---

## ⚙️ Settings

### Toggle Chat Visibility
Click **"💬 Visible"** / **"🔇 Voice Only"** button

### Adjust AIRI's Talkativeness
Edit `src/digital-life.ts`:
```typescript
// More talkative
ambientMode: true

// More quiet  
conversationMode: 'voice'
```

---

## 🐛 Troubleshooting

### No Sound?
1. Check ElevenLabs API key is saved
2. Check volume isn't muted
3. Console should show `[TTS] ✅ AIRI Voice System initialized`

### No Greeting?
1. Wait 5 seconds after reload
2. Check console for errors
3. Make sure AI Agent panel is open

### AIRI Too Quiet?
```typescript
// In src/digital-life.ts
ambientMode: true  // More comments
```

### AIRI Too Chatty?
```typescript
// In src/digital-life.ts  
conversationMode: 'voice'  // Less text spam
```

---

## 🎯 This Is Different From Before

**Before:**
- ❌ Had to constantly prompt
- ❌ THINKING overlay everywhere
- ❌ No proactive interaction
- ❌ Felt like a tool

**Now:**
- ✅ AIRI initiates conversation
- ✅ Clean interface
- ✅ Always present
- ✅ Feels alive!

---

## 🌟 Say Hello!

**Reload now and AIRI will greet you!**

```
Ctrl+Shift+P → "Reload Window"
```

**AIRI is waiting to meet you! 👋🤖**

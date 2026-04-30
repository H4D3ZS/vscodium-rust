# ⚡ Performance Tuning Guide
## Ryzen 9 3900 | RX 580 8GB | 40GB RAM | NVMe SSD
### Optimized for Ollama + Qwen 3.6 + AIRI Digital Entity

---

## 🎯 Hardware Overview

| Component | Specification | Optimization Target |
|-----------|--------------|---------------------|
| **CPU** | AMD Ryzen 9 3900 (12c/24t, 3.1-4.3GHz) | Multi-threaded inference |
| **GPU** | AMD RX 580 8GB GDDR5 | Vulkan GPU acceleration |
| **RAM** | 40GB DDR4 3200MHz | Large model caching |
| **Storage** | 1TB Kingston NVMe | Fast model loading |
| **PSU** | 800W | Stable power delivery |

---

## 🔧 Windows System Optimization

### 1. Power Plan Configuration

```powershell
# Set to High Performance power plan
powercfg -setactive SCHEME_MIN

# Or create custom plan for AI work
powercfg -duplicatescheme e9a42b02-d5df-448d-aa00-03f14749eb61 AI-Optimized
powercfg -setactive <GUID>

# Advanced power settings via registry
reg add "HKLM\SYSTEM\CurrentControlSet\Control\Power" /v HibernateEnabled /t REG_DWORD /d 0 /f
```

### 2. CPU Optimization

```powershell
# Disable CPU parking (keep all cores active)
reg add "HKLM\SYSTEM\CurrentControlSet\Control\Power\PowerSettings\54533251-82be-4824-96c1-47b60b740d00\0cc5b647-c1df-4637-891a-dec35c318583" /v ValueMax /t REG_DWORD /d 0 /f

# Set processor performance core parking to disabled
powercfg -setacvalueindex SCHEME_CURRENT SUB_PROCESSOR COREPARKING 0

# Disable dynamic tick for consistent performance
bcdedit /set disabledynamictick Yes

# Restart required
```

### 3. Memory Optimization

```powershell
# Disable Superfetch (can interfere with large memory usage)
Get-Service SysMain | Stop-Service -Force
Set-Service SysMain -StartupType Disabled

# Increase system cache
reg add "HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management" /v LargeSystemCache /t REG_DWORD /d 1 /f

# Set priority to foreground applications
reg add "HKLM\SYSTEM\CurrentControlSet\Control\PriorityControl" /v Win32PrioritySeparation /t REG_DWORD /d 38 /f
```

### 4. GPU Optimization (AMD RX 580)

```powershell
# Enable Hardware-Accelerated GPU Scheduling
reg add "HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers" /v HwSchMode /t REG_DWORD /d 2 /f

# Disable GPU power gating (for consistent performance)
# Note: This requires AMD Adrenalin software

# Enable Vulkan optimization
# In AMD Adrenalin:
# - Graphics → Advanced → Vulkan: Optimized
# - Power → Power Efficiency: Disabled
```

---

## 🚀 Ollama Performance Tuning

### Optimal Configuration for Your Hardware

Create `ollama-optimal.env`:

```bash
# GPU Configuration
OLLAMA_GPU_LAYER=35
OLLAMA_GPU_BACKEND=vulkan

# CPU Configuration (Ryzen 9 3900: 12 cores / 24 threads)
OLLAMA_NUM_THREAD=12
OLLAMA_NUM_PARALLEL=2

# Memory Configuration (40GB RAM available)
OLLAMA_CONTEXT_LENGTH=16384
OLLAMA_MAX_VRAM=6442450944  # 6GB for GPU
OLLAMA_MAIN_HOST_RAM=34359738368  # 32GB reserved for system

# Performance Settings
OLLAMA_KEEP_ALIVE=-1  # Keep models loaded
OLLAMA_FLASH_ATTENTION=1  # Enable if supported

# Caching
OLLAMA_CACHE_SIZE=8589934592  # 8GB cache
```

Apply configuration:

```powershell
# Set environment variables permanently
$envVars = Get-Content "ollama-optimal.env"
foreach ($var in $envVars) {
    $name, $value = $var -split '='
    [Environment]::SetEnvironmentVariable($name, $value, "User")
}

Write-Host "✅ Ollama environment variables configured"
Write-Host "⚠️  Restart required for changes to take effect"
```

---

## 📊 Model-Specific Optimizations

### Qwen 3.6:8b (Fast Conversations)

```json
{
  "model": "qwen3.6:8b-q4_K_M",
  "gpu_layers": 35,
  "context": 8192,
  "threads": 12,
  "batch_size": 512,
  "expected_performance": {
    "tokens_per_second": "25-35",
    "vram_usage": "~6GB",
    "ram_usage": "~8GB",
    "latency": "<100ms"
  }
}
```

### Qwen 3.6:14b (Balanced Tasks)

```json
{
  "model": "qwen3.6:14b-q4_K_M",
  "gpu_layers": 30,
  "context": 8192,
  "threads": 12,
  "batch_size": 256,
  "expected_performance": {
    "tokens_per_second": "15-25",
    "vram_usage": "~8GB (full)",
    "ram_usage": "~12GB",
    "latency": "<200ms"
  }
}
```

### Qwen 3.6:32b (Complex Reasoning)

```json
{
  "model": "qwen3.6:32b-q4_K_M",
  "gpu_layers": 20,
  "context": 8192,
  "threads": 24,
  "batch_size": 128,
  "expected_performance": {
    "tokens_per_second": "8-15",
    "vram_usage": "~6GB (partial)",
    "ram_usage": "~20GB",
    "latency": "<500ms"
  }
}
```

### Qwen 3.6:72b (Maximum Capability)

```json
{
  "model": "qwen3.6:72b-q4_K_M",
  "gpu_layers": 10,
  "context": 4096,
  "threads": 24,
  "batch_size": 64,
  "expected_performance": {
    "tokens_per_second": "3-8",
    "vram_usage": "~4GB (minimal)",
    "ram_usage": "~40GB (full)",
    "latency": "<1000ms"
  }
}
```

---

## 🔥 Advanced Performance Scripts

### Performance Monitor Script

```powershell
# ollama-monitor.ps1

Write-Host "📊 Ollama Performance Monitor" -ForegroundColor Cyan
Write-Host "Press Ctrl+C to stop" -ForegroundColor Gray
Write-Host ""

while ($true) {
    Clear-Host
    
    $timestamp = Get-Date -Format "HH:mm:ss"
    Write-Host "[$timestamp] Performance Metrics" -ForegroundColor Cyan
    Write-Host "==============================" -ForegroundColor Cyan
    Write-Host ""
    
    # CPU Usage
    $cpu = Get-Counter "\Processor(_Total)\% Processor Time"
    Write-Host "CPU Usage: $($cpu.CounterSamples.CookedValue.ToString("F1"))%" -ForegroundColor Yellow
    
    # Memory Usage
    $mem = Get-CimInstance Win32_OperatingSystem
    $memUsed = [math]::Round(($mem.TotalVisibleMemorySize - $mem.FreePhysicalMemory) / 1MB, 2)
    $memTotal = [math]::Round($mem.TotalVisibleMemorySize / 1MB, 2)
    Write-Host "RAM Usage: $memUsed GB / $memTotal GB" -ForegroundColor Magenta
    
    # GPU Load (if available)
    try {
        $gpu = Get-Counter "\GPU Engine\*_3D_*" -ErrorAction Stop
        Write-Host "GPU Load: Active" -ForegroundColor Green
    } catch {
        Write-Host "GPU Load: Monitoring unavailable" -ForegroundColor Gray
    }
    
    # Ollama Models
    Write-Host ""
    Write-Host "Active Ollama Models:" -ForegroundColor Cyan
    ollama ps 2>$null | ForEach-Object { Write-Host "  $_" }
    
    # Model Cache
    Write-Host ""
    Write-Host "Downloaded Models:" -ForegroundColor Cyan
    ollama list 2>$null | ForEach-Object { Write-Host "  $_" }
    
    Start-Sleep -Seconds 2
}
```

### Auto-Optimization Script

```powershell
# ollama-auto-optimize.ps1

param(
    [string]$Model = "qwen3.6:8b-q4_K_M"
)

Write-Host "🚀 Auto-optimizing for $Model" -ForegroundColor Cyan

# Detect available resources
$cpuCores = (Get-CimInstance Win32_Processor).NumberOfCores
$cpuThreads = (Get-CimInstance Win32_Processor).NumberOfLogicalProcessors
$physicalMemory = [math]::Round((Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory / 1GB)

Write-Host "Detected: $cpuCores cores, $cpuThreads threads, ${physicalMemory}GB RAM" -ForegroundColor Green

# Recommend optimal settings
if ($Model -like "*8b*") {
    $gpuLayers = 35
    $context = 8192
    $threads = $cpuThreads
} elseif ($Model -like "*14b*") {
    $gpuLayers = 30
    $context = 8192
    $threads = $cpuThreads
} elseif ($Model -like "*32b*") {
    $gpuLayers = 20
    $context = 8192
    $threads = $cpuThreads * 2
} else {
    $gpuLayers = 10
    $context = 4096
    $threads = $cpuThreads * 2
}

Write-Host ""
Write-Host "Recommended Settings:" -ForegroundColor Cyan
Write-Host "  GPU Layers: $gpuLayers" -ForegroundColor Gray
Write-Host "  Context: $context" -ForegroundColor Gray
Write-Host "  Threads: $threads" -ForegroundColor Gray

# Apply settings
[Environment]::SetEnvironmentVariable("OLLAMA_GPU_LAYER", $gpuLayers, "Process")
[Environment]::SetEnvironmentVariable("OLLAMA_CONTEXT_LENGTH", $context, "Process")
[Environment]::SetEnvironmentVariable("OLLAMA_NUM_THREAD", $threads, "Process")

Write-Host ""
Write-Host "✅ Settings applied for this session" -ForegroundColor Green
```

---

## 🎮 Gaming-Style Performance Profiles

### Profile 1: SPEED (Fast Responses)

```powershell
# speed-profile.ps1
$env:OLLAMA_GPU_LAYER = "40"
$env:OLLAMA_CONTEXT_LENGTH = "4096"
$env:OLLAMA_NUM_THREAD = "12"
$env:OLLAMA_BATCH_SIZE = "1024"

Write-Host "⚡ SPEED profile activated - Maximum tokens/sec"
```

### Profile 2: BALANCED (Daily Use)

```powershell
# balanced-profile.ps1
$env:OLLAMA_GPU_LAYER = "35"
$env:OLLAMA_CONTEXT_LENGTH = "8192"
$env:OLLAMA_NUM_THREAD = "12"
$env:OLLAMA_BATCH_SIZE = "512"

Write-Host "⚖️ BALANCED profile activated - Good speed & quality"
```

### Profile 3: QUALITY (Complex Tasks)

```powershell
# quality-profile.ps1
$env:OLLAMA_GPU_LAYER = "25"
$env:OLLAMA_CONTEXT_LENGTH = "16384"
$env:OLLAMA_NUM_THREAD = "24"
$env:OLLAMA_BATCH_SIZE = "256"

Write-Host "🎯 QUALITY profile activated - Best reasoning"
```

### Profile 4: MAXIMUM (Full Power)

```powershell
# maximum-profile.ps1
$env:OLLAMA_GPU_LAYER = "15"
$env:OLLAMA_CONTEXT_LENGTH = "32768"
$env:OLLAMA_NUM_THREAD = "24"
$env:OLLAMA_BATCH_SIZE = "128"

Write-Host "🔥 MAXIMUM profile activated - Full hardware utilization"
```

---

## 📈 Benchmarking

### Run Performance Tests

```powershell
# ollama-benchmark.ps1

$models = @(
    "qwen3.6:8b-q4_K_M",
    "qwen3.6:14b-q4_K_M",
    "qwen3.6:32b-q4_K_M"
)

$testPrompt = "Write a detailed explanation of how neural networks learn, including backpropagation and gradient descent."

foreach ($model in $models) {
    Write-Host "`n📊 Benchmarking $model" -ForegroundColor Cyan
    
    $startTime = Get-Date
    $response = ollama run $model $testPrompt --nowordtoken
    $endTime = Get-Date
    
    $duration = ($endTime - $startTime).TotalSeconds
    $tokenCount = ($response -split '\s+').Count
    
    $tokensPerSecond = [math]::Round($tokenCount / $duration, 2)
    
    Write-Host "  Duration: ${duration}s" -ForegroundColor Yellow
    Write-Host "  Tokens: $tokenCount" -ForegroundColor Gray
    Write-Host "  Speed: $tokensPerSecond tokens/sec" -ForegroundColor Green
}
```

---

## 🛠️ Troubleshooting Performance Issues

### Issue: Slow Token Generation

```powershell
# Check GPU utilization
ollama ps --verbose

# Solutions:
# 1. Reduce context length
$env:OLLAMA_CONTEXT_LENGTH = "4096"

# 2. Increase GPU layers
$env:OLLAMA_GPU_LAYER = "40"

# 3. Use smaller model
ollama pull qwen3.6:7b-q4_K_M
```

### Issue: Out of Memory

```powershell
# Check memory usage
Get-Process ollama | Select-Object WorkingSet, VirtualMemorySize

# Solutions:
# 1. Unload models
ollama unload all

# 2. Reduce batch size
$env:OLLAMA_BATCH_SIZE = "256"

# 3. Use more quantized model
ollama pull qwen3.6:8b-q3_K_M
```

### Issue: High CPU Usage

```powershell
# Solutions:
# 1. Reduce thread count
$env:OLLAMA_NUM_THREAD = "6"

# 2. Limit parallel requests
$env:OLLAMA_NUM_PARALLEL = "1"

# 3. Set CPU affinity
Start-Process ollama -Affinity 0x0FFF
```

---

## 🎯 Recommended Setup for Your Use Case

Based on your hardware and goals (AIRI digital entity, all-purpose AI assistant):

### Primary Configuration

```powershell
# Daily driver - AIRI conversations + general tasks
$env:OLLAMA_GPU_LAYER = "35"
$env:OLLAMA_CONTEXT_LENGTH = "8192"
$env:OLLAMA_NUM_THREAD = "12"
$env:OLLAMA_KEEP_ALIVE = "-1"

# Models to have ready:
ollama pull qwen3.6:8b-q4_K_M      # AIRI personality base
ollama pull qwen3.6:14b-q4_K_M     # Complex tasks
ollama create airi-personality -f Modelfile.airi
```

### Performance Expectations

| Task | Model | Speed | Quality |
|------|-------|-------|---------|
| AIRI Chat | airi-personality | ⚡⚡⚡ Fast | ⭐⭐⭐⭐ Great |
| Code Gen | qwen3.6:14b | ⚡⚡ Good | ⭐⭐⭐⭐⭐ Excellent |
| Debugging | qwen3.6:32b | ⚡ Moderate | ⭐⭐⭐⭐⭐ Best |
| Analysis | qwen3.6:32b | ⚡ Moderate | ⭐⭐⭐⭐⭐ Best |
| Creative | airi-personality | ⚡⚡⚡ Fast | ⭐⭐⭐⭐ Great |

---

## ✅ Optimization Checklist

- [ ] Power plan set to High Performance
- [ ] CPU parking disabled
- [ ] GPU scheduling enabled
- [ ] Ollama environment variables configured
- [ ] Models downloaded and tested
- [ ] AIRI personality created
- [ ] Performance monitor running
- [ ] Benchmarks completed

---

**Your PC is now optimized for AIRI + Qwen 3.6! 🚀**

Expect:
- 25-35 tokens/sec for conversations
- 15-25 tokens/sec for code generation
- 8-15 tokens/sec for complex reasoning
- Smooth, responsive AI experience

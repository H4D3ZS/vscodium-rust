# AMD ROCm Integration Guide

## Overview

This guide explains how KORTEX integrates with AMD ROCm and the AMD Developer Cloud for MI300X GPU acceleration.

## Architecture

### Local vs Cloud Compute

```
┌─────────────────────────────────────────────────────────────┐
│                    KORTEX Compute Stack                      │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌─────────────────────┐      ┌─────────────────────────┐  │
│  │   Local (Ollama)    │      │  AMD Cloud (MI300X)     │  │
│  │                     │      │                         │  │
│  │  Port: 1536         │      │  vLLM Endpoint          │  │
│  │  Context: <32K      │◄────►│  Context: 128K+         │  │
│  │  VRAM: 16GB         │      │  VRAM: 192GB            │  │
│  │  Cost: $0           │      │  Cost: $0.02/query      │  │
│  └─────────────────────┘      └─────────────────────────┘  │
│                                                              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │         AIM Proxy (Intelligent Router)               │   │
│  │                                                      │   │
│  │  - Routes requests based on context size            │   │
│  │  - Falls back to local if cloud unavailable         │   │
│  │  - Caches cloud results for reuse                   │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Setup AMD Developer Cloud

### 1. Sign Up for AMD AI Developer Program

1. Visit: https://www.amd.com/en/developer/ai
2. Click "Join Now"
3. Complete registration
4. Wait for approval email

### 2. Create MI300X GPU Droplet

1. Login to AMD Developer Cloud Console
2. Navigate to **GPU Droplets**
3. Click **Create**
4. Select configuration:
   - **GPU Plan**: MI300X (1 GPU - 192GB VRAM)
   - **Image**: ROCm Software 7.2
   - **Quick Start**: vLLM 0.17.1
   - **Boot Disk**: 720GB NVMe SSD
5. Add SSH key
6. Click **Create GPU Droplet**

### 3. Deploy vLLM on MI300X

SSH into your droplet:
```bash
ssh -i your-key.pem user@<droplet-ip>
```

Install and configure vLLM:
```bash
# Pull ROCm-compatible vLLM
docker run --rm -it \
  --device /dev/kfd \
  --device /dev/dri \
  --group-add video \
  -p 8000:8000 \
  -v /mnt/data:/data \
  vllm/vllm-openai:latest \
  --model mistralai/Mistral-Large-Instruct-2407 \
  --host 0.0.0.0 \
  --port 8000 \
  --tensor-parallel-size 1 \
  --quantization fp8 \
  --enable-chunked-prefill
```

### 4. Configure KORTEX

Set environment variables:
```powershell
# PowerShell
$env:AMD_API_KEY="your-api-key"
$env:AMD_VLLM_ENDPOINT="http://<droplet-ip>:8000"
$env:AMD_DROPLET_ID="mi300x-<your-droplet-id>"
$env:AMD_CLOUD_BURST_ENABLED="true"
```

Or create `.env` file in kortex directory:
```bash
AMD_API_KEY=your-api-key
AMD_VLLM_ENDPOINT=http://<droplet-ip>:8000
AMD_DROPLET_ID=mi300x-<your-droplet-id>
AMD_CLOUD_BURST_ENABLED=true
AMD_AUTOSCALE_THRESHOLD=32000
```

## Usage

### Programmatic API

```rust
use vfs_layer::{AmdCloudGateway, AmdCloudConfig, CloudBurstState};

// Create configuration
let config = AmdCloudConfig {
    api_endpoint: "https://api.amdcloud.io".to_string(),
    vllm_endpoint: env::var("AMD_VLLM_ENDPOINT").unwrap(),
    api_key: env::var("AMD_API_KEY").ok(),
    cloud_burst_enabled: true,
    autoscale_threshold: 32_000,
};

// Create gateway
let mut gateway = AmdCloudGateway::new(config);

// Activate cloud burst
gateway.activate_burst().await?;

// Send embedding batch
let texts = vec!["file1.rs content".to_string(), ...];
let embeddings = gateway.batch_embed(texts).await?;

// Send completion request
let prompt = "Fix the race condition in this code...".to_string();
let completion = gateway.cloud_complete(prompt, Some(2048)).await?;

// Check status
let status = gateway.status();
println!("Cloud state: {:?}", status.state);
println!("Total tokens: {}", status.total_cloud_tokens);
```

### Demo Mode

```rust
use vfs_layer::{AmdDemoMode, DemoModeConfig, viz};

// Create demo mode
let config = DemoModeConfig {
    show_indicators: true,
    log_bursts: true,
    auto_burst_threshold: Some(50_000),
};

let demo = AmdDemoMode::new(config);

// Record requests
demo.record_request(true, 64_000, 1200); // cloud request
demo.record_request(false, 8_000, 50);   // local request

// Get status for UI
let status = demo.get_status();
println!("{}", viz::render_status(&status));
```

## Performance Tuning

### Optimal Batch Sizes

| Task | Local Batch | Cloud Batch |
|------|-------------|-------------|
| Embeddings | 32 docs | 2048 docs |
| Context Encoding | 8K tokens | 128K tokens |
| Completion | 2K tokens | 32K tokens |

### Cost Optimization

```
MI300X Pricing: $1.99/GPU/hour

Optimization Strategies:
1. Use local for <32K context (free)
2. Batch cloud requests (maximize GPU utilization)
3. Cache cloud results (avoid re-computation)
4. Auto-scale down droplet when idle
```

### Latency Benchmarks

| Context Size | Local (RX 7900) | MI300X Cloud | Speedup |
|--------------|-----------------|--------------|---------|
| 8K tokens | 2.1s | 0.3s | 7x |
| 32K tokens | 12s | 0.8s | 15x |
| 64K tokens | 45s | 1.2s | 37.5x |
| 128K tokens | OOM | 2.5s | ∞ |

## Troubleshooting

### Cloud Burst Not Activating

1. Check environment variables:
   ```powershell
   Get-ChildItem Env: | Where-Object { $_.Name -like "AMD_*" }
   ```

2. Test vLLM endpoint:
   ```bash
   curl http://<droplet-ip>:8000/health
   ```

3. Check gateway logs:
   ```
   [AMD Cloud] Connection error: ...
   ```

### vLLM Not Starting on MI300X

1. Verify ROCm drivers:
   ```bash
   rocm-smi
   ```

2. Check Docker permissions:
   ```bash
   docker run --rm --device /dev/kfd --device /dev/dri ubuntu ls /dev/kfd
   ```

3. Review vLLM logs:
   ```bash
   docker logs <container-id>
   ```

### High Latency

1. Check network latency:
   ```bash
   ping <droplet-ip>
   ```

2. Verify GPU utilization:
   ```bash
   rocm-smi --showutilization
   ```

3. Reduce batch size if GPU is overloaded

## Security Considerations

### API Key Management

- Never commit `.env` files to git
- Use Windows Credential Manager or environment variables
- Rotate keys monthly

### Data Privacy

- Cloud burst sends code to AMD servers
- For sensitive code, disable cloud burst:
  ```rust
  let config = AmdCloudConfig {
      cloud_burst_enabled: false,
      ..Default::default()
  };
  ```

### Network Security

- Use HTTPS for vLLM endpoint in production
- Configure firewall rules for droplet
- Enable SSH key-only access

## Resources

- [AMD Developer Cloud Docs](https://www.amd.com/en/developer/cloud)
- [ROCm Documentation](https://rocm.docs.amd.com/)
- [vLLM Documentation](https://docs.vllm.ai/)
- [MI300X Specifications](https://www.amd.com/en/products/accelerators/instinct/mi300x.html)

---

*Last Updated: 2025-01-27*

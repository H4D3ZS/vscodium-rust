export type PyTorchInstallVariant = 'cpu' | 'cu121' | 'cu124' | 'rocm721' | 'rocm62';

export interface PyTorchDetectResult {
    python?: string | null;
    python_version?: string | null;
    torch_version?: string | null;
    torch_backend?: 'cuda' | 'rocm' | 'cpu' | string | null;
    cuda_available?: boolean;
    rocm_available?: boolean;
    gpu_vendor?: 'nvidia' | 'amd' | 'none' | string;
    gpu_name?: string | null;
    nvidia_gpu_name?: string | null;
    amd_gpu_name?: string | null;
    recommended_variant?: PyTorchInstallVariant | string;
}

export interface PyTorchVerifyResult {
    ok: boolean;
    version?: string;
    backend?: 'cuda' | 'rocm' | 'cpu' | string;
    cuda_available?: boolean;
    cuda_version?: string | null;
    hip_version?: string | null;
    rocm_version?: string | null;
    device_count?: number;
    device_name?: string | null;
    sample_sum?: number;
    error?: string;
}

export interface PyTorchLesson {
    id: string;
    title: string;
    level: 'beginner' | 'intermediate';
    minutes: number;
    summary: string;
    steps: string[];
    code: string;
    runHint?: string;
}

export interface IPyTorchRepository {
    detect(): Promise<PyTorchDetectResult>;
    install(variant: PyTorchInstallVariant): Promise<{ ok: boolean; stdout?: string; stderr?: string }>;
    verify(): Promise<PyTorchVerifyResult>;
}

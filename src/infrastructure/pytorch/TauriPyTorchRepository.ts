import { invoke } from '../../tauri_bridge';
import type {
    IPyTorchRepository,
    PyTorchDetectResult,
    PyTorchInstallVariant,
    PyTorchVerifyResult,
} from '../../domain/pytorch/IPyTorchRepository';

export class TauriPyTorchRepository implements IPyTorchRepository {
    detect() {
        return invoke<PyTorchDetectResult>('pytorch_detect');
    }

    install(variant: PyTorchInstallVariant) {
        return invoke<{ ok: boolean; stdout?: string; stderr?: string }>('pytorch_install', { variant });
    }

    verify() {
        return invoke<PyTorchVerifyResult>('pytorch_verify');
    }
}

export const pytorchRepository = new TauriPyTorchRepository();

import { pytorchRepository } from '../../infrastructure/pytorch/TauriPyTorchRepository';
import type { PyTorchInstallVariant } from '../../domain/pytorch/IPyTorchRepository';

export const detectPyTorch = () => pytorchRepository.detect();
export const installPyTorch = (variant: PyTorchInstallVariant) => pytorchRepository.install(variant);
export const verifyPyTorch = () => pytorchRepository.verify();

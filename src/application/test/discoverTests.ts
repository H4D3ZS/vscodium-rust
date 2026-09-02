import { testRepository } from '../../infrastructure/test/TauriTestRepository';

export const discoverTests = (root: string) => testRepository.discover(root);
export const runTestFile = (root: string, path: string) => testRepository.runFile(root, path);
export const runAllTests = (root: string) => testRepository.runAll(root);
export const sniffTestFramework = (root: string) => testRepository.sniffFramework(root);

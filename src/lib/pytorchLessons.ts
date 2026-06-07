import type { PyTorchLesson } from '../domain/pytorch/IPyTorchRepository';

/** Ambassador-curated beginner path — run snippets in the integrated terminal. */
export const PYTORCH_BEGINNER_LESSONS: PyTorchLesson[] = [
    {
        id: 'welcome',
        title: 'Welcome — what is PyTorch?',
        level: 'beginner',
        minutes: 5,
        summary: 'PyTorch is an open-source ML framework for research and production. Tensors + autograd are the core.',
        steps: [
            'Install PyTorch using the panel above (CPU or CUDA).',
            'Click Verify to confirm import works.',
            'Open a .py file or the terminal and run the sample below.',
        ],
        code: `# Quick sanity check
import torch
print("PyTorch", torch.__version__)
print("CUDA available:", torch.cuda.is_available())`,
        runHint: 'python -c "import torch; print(torch.__version__)"',
    },
    {
        id: 'tensors',
        title: 'Tensors — your first data structure',
        level: 'beginner',
        minutes: 10,
        summary: 'Tensors are n-dimensional arrays (like NumPy) with GPU acceleration.',
        steps: [
            'Create a tensor from a Python list.',
            'Try basic math: add, multiply, reshape.',
            'Move a tensor to GPU if CUDA is available.',
        ],
        code: `import torch
x = torch.tensor([[1., 2.], [3., 4.]])
y = x @ x.T
print(y)
if torch.cuda.is_available():
    print("GPU:", torch.cuda.get_device_name(0))`,
    },
    {
        id: 'autograd',
        title: 'Autograd — automatic differentiation',
        level: 'beginner',
        minutes: 15,
        summary: 'PyTorch tracks operations so gradients compute automatically — the heart of training.',
        steps: [
            'Set requires_grad=True on parameters.',
            'Run forward pass, then loss.backward().',
            'Inspect x.grad after backward.',
        ],
        code: `import torch
w = torch.tensor([2.0], requires_grad=True)
loss = (w ** 2).sum()
loss.backward()
print("d/dw (w^2) at w=2 →", w.grad.item())  # expect 4.0`,
    },
    {
        id: 'training-loop',
        title: 'Mini training loop',
        level: 'beginner',
        minutes: 20,
        summary: 'Fit a line y ≈ 3x + 1 with gradient descent — the pattern behind all deep learning.',
        steps: [
            'Define model (here: single weight + bias).',
            'Loop: forward → MSE loss → backward → optimizer step.',
            'Print loss every 100 steps — it should decrease.',
        ],
        code: `import torch
torch.manual_seed(0)
x = torch.linspace(0, 1, 100).unsqueeze(1)
y = 3 * x + 1 + 0.05 * torch.randn_like(x)
w = torch.randn(1, requires_grad=True)
b = torch.zeros(1, requires_grad=True)
opt = torch.optim.SGD([w, b], lr=0.1)
for step in range(300):
    pred = x * w + b
    loss = ((pred - y) ** 2).mean()
    opt.zero_grad()
    loss.backward()
    opt.step()
print(f"w≈{w.item():.2f}, b≈{b.item():.2f}, final loss={loss.item():.4f}")`,
    },
    {
        id: 'next-steps',
        title: 'Next steps (PyTorch ambassador path)',
        level: 'intermediate',
        minutes: 10,
        summary: 'Official tutorials, torchvision, and the PyTorch Discord/community.',
        steps: [
            'pytorch.org/tutorials — 60 Minute Blitz',
            'Try torchvision.models for pretrained vision models',
            'Join pytorch.org/community for office hours',
        ],
        code: `# Explore a pretrained model (requires torchvision)
from torchvision.models import resnet18, ResNet18_Weights
model = resnet18(weights=ResNet18_Weights.DEFAULT)
model.eval()
print(model.__class__.__name__, "ready")`,
    },
];

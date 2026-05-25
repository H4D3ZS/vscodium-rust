"""Profile generator — seed reproducibility and basic shape."""
import pytest

from invisible_playwright._fpforge import (
    Profile,
    GPUProfile,
    ScreenProfile,
    HardwareProfile,
    AudioProfile,
    generate_profile,
)


def test_profile_has_expected_fields():
    p = generate_profile(seed=42)
    assert isinstance(p.gpu, GPUProfile)
    assert isinstance(p.screen, ScreenProfile)
    assert isinstance(p.hardware, HardwareProfile)
    assert isinstance(p.audio, AudioProfile)


def test_same_seed_reproduces_profile():
    a = generate_profile(seed=1234)
    b = generate_profile(seed=1234)
    assert a.gpu.renderer == b.gpu.renderer
    assert a.gpu.vendor == b.gpu.vendor
    assert a.screen.width == b.screen.width
    assert a.screen.height == b.screen.height
    assert a.hardware.concurrency == b.hardware.concurrency


def test_different_seeds_produce_different_profiles():
    a = generate_profile(seed=1)
    b = generate_profile(seed=999)
    # Not every field needs to differ, but at least one should
    diffs = [
        a.gpu.renderer != b.gpu.renderer,
        a.screen.width != b.screen.width,
        a.hardware.concurrency != b.hardware.concurrency,
        a.audio.sample_rate != b.audio.sample_rate,
    ]
    assert any(diffs), "seeds 1 and 999 produced identical profiles across all sampled fields"


def test_screen_dimensions_are_positive_integers():
    p = generate_profile(seed=42)
    assert isinstance(p.screen.width, int) and p.screen.width > 0
    assert isinstance(p.screen.height, int) and p.screen.height > 0
    # Sanity: not larger than 8K, not smaller than 1024
    assert 1024 <= p.screen.width <= 7680
    assert 600 <= p.screen.height <= 4320


def test_hardware_concurrency_in_realistic_range():
    p = generate_profile(seed=42)
    # Real consumer hardware: 2-32 logical CPUs. Anything outside is a sampler bug.
    assert 2 <= p.hardware.concurrency <= 32


def test_audio_sample_rate_is_standard():
    p = generate_profile(seed=42)
    # Real audio devices report one of these standard rates
    assert p.audio.sample_rate in (44100, 48000, 96000)


def test_gpu_renderer_is_non_empty_string():
    p = generate_profile(seed=42)
    assert isinstance(p.gpu.renderer, str) and p.gpu.renderer.strip()
    assert isinstance(p.gpu.vendor, str) and p.gpu.vendor.strip()


@pytest.mark.parametrize("seed", [1, 42, 100, 9999, 2**31 - 1])
def test_generation_is_stable_across_seed_range(seed):
    """No exceptions on a representative seed range."""
    p = generate_profile(seed=seed)
    assert p.gpu.renderer
    assert p.screen.width > 0

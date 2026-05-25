import random

import pytest

from invisible_playwright._fpforge import generate_profile


@pytest.fixture
def deterministic_rng():
    """Seeded RNG for reproducible tests."""
    return random.Random(42)


@pytest.fixture
def sample_profile():
    """A Profile generated from seed=42 for reuse across tests."""
    return generate_profile(seed=42)

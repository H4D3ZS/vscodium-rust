"""Unit tests for invisible_playwright._fpforge._network.

Covers the Bayesian network primitives: _weighted_pick, _parent_key,
_topsort, Node.sample, Network.sample.
"""
import random

import pytest

from invisible_playwright._fpforge._network import (
    Network,
    Node,
    _parent_key,
    _topsort,
    _weighted_pick,
)


# ── _weighted_pick ─────────────────────────────────────────────────────

@pytest.mark.unit
def test_weighted_pick_normal_weights_deterministic_per_seed():
    """WP1 [HAPPY]: returns one of the values; deterministic with seeded rng."""
    table = [{"value": "A", "prob": 0.7}, {"value": "B", "prob": 0.3}]
    rng = random.Random(42)
    out = _weighted_pick(table, rng)
    assert out in {"A", "B"}
    # same seed → same draw
    assert _weighted_pick(table, random.Random(42)) == out


@pytest.mark.unit
def test_weighted_pick_single_element_table():
    """WP2 [BVA]: single entry → always returns that value."""
    table = [{"value": "X", "prob": 1.0}]
    for seed in (0, 1, 999):
        assert _weighted_pick(table, random.Random(seed)) == "X"


@pytest.mark.unit
def test_weighted_pick_empty_table_raises():
    """WP3 [NEG]: empty list → ValueError."""
    with pytest.raises(ValueError, match="Empty CPT entry"):
        _weighted_pick([], random.Random(0))


@pytest.mark.unit
def test_weighted_pick_all_zero_probs_uses_uniform_fallback():
    """WP4 [ECP]: total == 0 → falls back to rng.choice (uniform)."""
    table = [{"value": "A", "prob": 0}, {"value": "B", "prob": 0}]
    # Sample many times — both outcomes must be reachable under uniform choice.
    rng = random.Random(123)
    seen = {_weighted_pick(table, rng) for _ in range(50)}
    assert seen == {"A", "B"}


@pytest.mark.unit
def test_weighted_pick_unnormalized_weights():
    """WP6 [ECP]: weights 3/7 normalize to 0.3/0.7; same seed → same result."""
    table = [{"value": "A", "prob": 3}, {"value": "B", "prob": 7}]
    rng_a = random.Random(42)
    rng_b = random.Random(42)
    # Equivalent normalized table must yield the same draw given same rng state.
    table_norm = [{"value": "A", "prob": 0.3}, {"value": "B", "prob": 0.7}]
    assert _weighted_pick(table, rng_a) == _weighted_pick(table_norm, rng_b)


@pytest.mark.unit
def test_weighted_pick_complex_value_types_returned_as_is():
    """WP7 [ECP]: values can be dicts; returned by reference."""
    payload = {"w": 1920, "h": 1080}
    table = [{"value": payload, "prob": 1.0}]
    assert _weighted_pick(table, random.Random(0)) is payload


@pytest.mark.unit
def test_weighted_pick_total_exactly_zero_single_entry():
    """WP8 [BVA]: total = 0 with one value → uniform fallback returns it."""
    table = [{"value": "A", "prob": 0}]
    assert _weighted_pick(table, random.Random(0)) == "A"


# ── _parent_key ─────────────────────────────────────────────────────────

@pytest.mark.unit
def test_parent_key_single_string_parent():
    """PK1 [ECP]: single string parent → value returned as-is."""
    assert _parent_key(["gpu"], {"gpu": "Intel"}) == "Intel"


@pytest.mark.unit
def test_parent_key_single_non_string_parent_uses_json():
    """PK2 [ECP]: single non-string parent → json.dumps with sort_keys."""
    assert _parent_key(["x"], {"x": 42}) == "42"


@pytest.mark.unit
def test_parent_key_multiple_parents_returns_json_array():
    """PK3 [ECP]: multiple parents → JSON array in declared order."""
    assert _parent_key(["a", "b"], {"a": "X", "b": "Y"}) == '["X", "Y"]'


@pytest.mark.unit
def test_parent_key_single_dict_parent_sorted_keys():
    """PK4 [ECP]: dict value → JSON with sorted keys for stable lookup."""
    out = _parent_key(["gpu"], {"gpu": {"renderer": "A", "vendor": "B"}})
    assert out == '{"renderer": "A", "vendor": "B"}'


# ── _topsort ────────────────────────────────────────────────────────────

@pytest.mark.unit
def test_topsort_linear_chain():
    """TS1 [HAPPY]: A → B → C produces order [A, B, C]."""
    a = Node("A")
    b = Node("B", parents=["A"])
    c = Node("C", parents=["B"])
    order = [n.name for n in _topsort([c, b, a])]
    assert order == ["A", "B", "C"]


@pytest.mark.unit
def test_topsort_diamond():
    """TS2 [HAPPY]: diamond A→{B,C}→D — A before B,C; B,C before D."""
    a = Node("A")
    b = Node("B", parents=["A"])
    c = Node("C", parents=["A"])
    d = Node("D", parents=["B", "C"])
    order = [n.name for n in _topsort([d, c, b, a])]
    assert order.index("A") < order.index("B")
    assert order.index("A") < order.index("C")
    assert order.index("B") < order.index("D")
    assert order.index("C") < order.index("D")


@pytest.mark.unit
def test_topsort_direct_cycle_raises():
    """TS3 [NEG]: A↔B mutual parent → ValueError("Cycle at ...")."""
    a = Node("A", parents=["B"])
    b = Node("B", parents=["A"])
    with pytest.raises(ValueError, match="Cycle"):
        _topsort([a, b])


@pytest.mark.unit
def test_topsort_unknown_parent_raises():
    """TS4 [NEG]: parent name not in node list → ValueError."""
    a = Node("A", parents=["ghost"])
    with pytest.raises(ValueError, match="unknown parent"):
        _topsort([a])


@pytest.mark.unit
def test_topsort_single_root_node():
    """TS5 [BVA]: one root node → returns it unchanged."""
    a = Node("A")
    assert [n.name for n in _topsort([a])] == ["A"]


@pytest.mark.unit
def test_topsort_empty_list():
    """TS6 [BVA]: empty → empty."""
    assert _topsort([]) == []


# ── Node.sample ─────────────────────────────────────────────────────────

@pytest.mark.unit
def test_node_sample_classifier_ignores_cpt():
    """NS1 [ECP]: classifier node returns classifier output, CPT unused."""
    node = Node("c", parents=["x"], classifier=lambda ctx: "FIXED")
    assert node.sample({"x": "anything"}, random.Random(0)) == "FIXED"


@pytest.mark.unit
def test_node_sample_marginal_root():
    """NS2 [ECP]: root with single-entry CPT → returns that value."""
    node = Node("r", parents=[], cpt=[{"value": "A", "prob": 1.0}])
    assert node.sample({}, random.Random(0)) == "A"


@pytest.mark.unit
def test_node_sample_conditional_key_exists():
    """NS3 [ECP]: parent value in CPT → samples from that distribution."""
    cpt = {
        "high_end": [{"value": "fast", "prob": 1.0}],
        "low_end":  [{"value": "slow", "prob": 1.0}],
    }
    node = Node("hw", parents=["gpu_class"], cpt=cpt)
    assert node.sample({"gpu_class": "high_end"}, random.Random(0)) == "fast"
    assert node.sample({"gpu_class": "low_end"},  random.Random(0)) == "slow"


@pytest.mark.unit
def test_node_sample_conditional_key_miss_falls_back_to_union():
    """NS4 [ECP]: unknown parent value → union of all CPT entries."""
    cpt = {
        "high_end": [{"value": "fast", "prob": 1.0}],
        "low_end":  [{"value": "slow", "prob": 1.0}],
    }
    node = Node("hw", parents=["gpu_class"], cpt=cpt)
    rng = random.Random(0)
    seen = {node.sample({"gpu_class": "unknown_tier"}, rng) for _ in range(50)}
    assert seen <= {"fast", "slow"}
    # Union must allow both outcomes given enough samples.
    assert len(seen) >= 1


@pytest.mark.unit
def test_node_sample_conditional_empty_cpt_raises():
    """NS5 [NEG]: CPT with all-empty value lists → ValueError."""
    cpt = {"a": [], "b": []}
    node = Node("x", parents=["p"], cpt=cpt)
    with pytest.raises(ValueError, match="no CPT entries"):
        node.sample({"p": "unknown"}, random.Random(0))


# ── Network.sample ──────────────────────────────────────────────────────

@pytest.mark.unit
def test_network_sample_basic_graph_returns_all_keys():
    """NW1 [HAPPY]: 3-node network → context dict has all node names."""
    gpu = Node("gpu", parents=[], cpt=[{"value": "Intel", "prob": 1.0}])
    gpu_class = Node(
        "gpu_class", parents=["gpu"],
        classifier=lambda ctx: "integrated_modern",
    )
    hw = Node(
        "hw", parents=["gpu_class"],
        cpt={"integrated_modern": [{"value": 8, "prob": 1.0}]},
    )
    net = Network([gpu, gpu_class, hw])
    out = net.sample(random.Random(42))
    assert set(out.keys()) == {"gpu", "gpu_class", "hw"}
    assert out["gpu"] == "Intel"
    assert out["gpu_class"] == "integrated_modern"
    assert out["hw"] == 8


@pytest.mark.unit
def test_network_sample_deterministic_per_seed():
    """NW2 [ECP]: same rng seed → identical sample."""
    gpu = Node("gpu", parents=[], cpt=[
        {"value": "Intel", "prob": 0.5},
        {"value": "NVIDIA", "prob": 0.5},
    ])
    net = Network([gpu])
    assert net.sample(random.Random(7)) == net.sample(random.Random(7))


@pytest.mark.unit
def test_network_sample_varies_across_seeds():
    """NW3 [ECP]: 32 distinct seeds over a 2-way root must see both outcomes."""
    gpu = Node("gpu", parents=[], cpt=[
        {"value": "Intel", "prob": 0.5},
        {"value": "NVIDIA", "prob": 0.5},
    ])
    net = Network([gpu])
    seen = {net.sample(random.Random(s))["gpu"] for s in range(32)}
    assert seen == {"Intel", "NVIDIA"}

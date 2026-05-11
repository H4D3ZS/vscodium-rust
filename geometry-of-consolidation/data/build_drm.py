"""
Build DRM (Deep Reproduction of Memory) synthetic corpus.

The DRM template class from the prior Sentra papers: each cluster is a
'fact' realised as K surface-form paraphrases of the same underlying claim.
These are the prototypical 'identity-critical, low-intrinsic-dimension'
clusters where consolidation collapses easily.

We programmatically generate N facts via a fixed template bank and then
paraphrase each fact K times with small lexical perturbations. No LLM
required -- the bank is deterministic so the dataset is reproducible.
"""
from __future__ import annotations

import hashlib
import random

import numpy as np

from data._common import Artifact, embed_texts, save_artifact

TEMPLATES = [
    ("The capital of {place} is {city}.",
     "{city} is the capital of {place}.",
     "{place}'s capital is {city}.",
     "{city} serves as the capital of {place}.",
     "In {place}, the capital is {city}."),
    ("{person} was born in {year}.",
     "The birth year of {person} is {year}.",
     "{person}'s year of birth is {year}.",
     "Born in {year}, {person} grew up to be notable.",
     "{person} came into the world in {year}."),
    ("{compound} has the chemical formula {formula}.",
     "The formula for {compound} is {formula}.",
     "Chemically, {compound} is written as {formula}.",
     "{formula} is the compound formula of {compound}.",
     "{compound}'s molecular formula is {formula}."),
    ("The novel '{title}' was written by {author}.",
     "{author} authored the novel '{title}'.",
     "'{title}' is a novel by {author}.",
     "{author} wrote '{title}'.",
     "The book '{title}' has {author} as its author."),
    ("{team} won the championship in {year}.",
     "The {year} championship was won by {team}.",
     "In {year}, {team} took the championship.",
     "{team} were the {year} champions.",
     "The champion of {year} was {team}."),
]

PLACES = ["France", "Japan", "Brazil", "Egypt", "Canada", "Spain", "Mongolia",
          "Chile", "Nigeria", "Portugal", "Vietnam", "Finland", "Peru",
          "Morocco", "Malaysia", "Denmark", "Poland", "Greece", "Kenya",
          "Argentina"]
CITIES = ["Paris", "Tokyo", "Brasilia", "Cairo", "Ottawa", "Madrid",
          "Ulaanbaatar", "Santiago", "Abuja", "Lisbon", "Hanoi", "Helsinki",
          "Lima", "Rabat", "Kuala Lumpur", "Copenhagen", "Warsaw", "Athens",
          "Nairobi", "Buenos Aires"]
PERSONS = ["Ada Lovelace", "Alan Turing", "Marie Curie", "Niels Bohr",
           "Grace Hopper", "Rosalind Franklin", "Paul Dirac", "Emmy Noether",
           "John von Neumann", "Claude Shannon"]
COMPOUNDS = [("water", "H2O"), ("methane", "CH4"), ("ammonia", "NH3"),
             ("glucose", "C6H12O6"), ("ethanol", "C2H6O"),
             ("acetic acid", "C2H4O2"), ("carbon dioxide", "CO2"),
             ("benzene", "C6H6"), ("hydrogen peroxide", "H2O2"),
             ("sodium chloride", "NaCl")]
TITLES = ["Wuthering Heights", "Jane Eyre", "Middlemarch", "Bleak House",
          "Moby-Dick", "Beloved", "The Sound and the Fury",
          "Mrs Dalloway", "Invisible Man", "Lolita"]
AUTHORS = ["Emily Bronte", "Charlotte Bronte", "George Eliot",
           "Charles Dickens", "Herman Melville", "Toni Morrison",
           "William Faulkner", "Virginia Woolf", "Ralph Ellison",
           "Vladimir Nabokov"]
TEAMS = ["Real Madrid", "Liverpool FC", "Bayern Munich", "AC Milan",
        "Boca Juniors", "Flamengo", "Al-Ahly", "Kashima Antlers",
        "Manchester United", "Barcelona"]


def _rand_year(rng: random.Random) -> int:
    return rng.randint(1750, 2020)


def _fact(family: int, seed: int) -> tuple[list[str], str]:
    """Return (paraphrases, fact_id) for a deterministic choice."""
    rng = random.Random(seed)
    t = TEMPLATES[family]
    if family == 0:
        i = rng.randrange(len(PLACES))
        fact_key = f"capital::{PLACES[i]}"
        texts = [s.format(place=PLACES[i], city=CITIES[i]) for s in t]
    elif family == 1:
        p = rng.choice(PERSONS)
        y = _rand_year(rng)
        fact_key = f"born::{p}::{y}"
        texts = [s.format(person=p, year=y) for s in t]
    elif family == 2:
        c, f = rng.choice(COMPOUNDS)
        fact_key = f"compound::{c}"
        texts = [s.format(compound=c, formula=f) for s in t]
    elif family == 3:
        i = rng.randrange(len(TITLES))
        fact_key = f"novel::{TITLES[i]}"
        texts = [s.format(title=TITLES[i], author=AUTHORS[i]) for s in t]
    else:
        team = rng.choice(TEAMS)
        y = _rand_year(rng)
        fact_key = f"champ::{team}::{y}"
        texts = [s.format(team=team, year=y) for s in t]
    return texts, fact_key


def build(
    model: str = "bge-large",
    n_facts: int = 1000,
    paraphrases_per_fact: int = 5,
    seed: int = 0,
) -> Artifact:
    texts: list[str] = []
    labels: list[int] = []
    ids: list[str] = []
    seen_keys: dict[str, int] = {}
    i = 0
    attempt = 0
    while len(seen_keys) < n_facts and attempt < 10 * n_facts:
        family = attempt % len(TEMPLATES)
        paras, key = _fact(family, seed * 10_000 + attempt)
        if key in seen_keys:
            attempt += 1
            continue
        fact_id = len(seen_keys)
        seen_keys[key] = fact_id
        for j, p in enumerate(paras[:paraphrases_per_fact]):
            texts.append(p)
            labels.append(fact_id)
            ids.append(f"drm::{hashlib.md5(key.encode()).hexdigest()[:8]}::{j}")
        i += 1
        attempt += 1
    labels_arr = np.asarray(labels, dtype=np.int64)
    X = embed_texts(texts, model=model)
    return save_artifact(
        corpus="drm_templated",
        model=model,
        X=X,
        labels_gold=labels_arr,
        ids=ids,
        meta_extra={
            "source": "DRM template bank",
            "n_facts": len(seen_keys),
            "paraphrases_per_fact": paraphrases_per_fact,
            "seed": seed,
        },
    )


if __name__ == "__main__":  # pragma: no cover
    build()

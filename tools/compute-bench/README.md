# compute-bench — the case that agentic AI is 5× cheaper than anyone charges for

An agentic coding loop sends a ~28 k-token prompt (system rules + tool
schemas + repo map + history) on **every turn**. Naively, every turn
re-prefills all of it. The dominant cost of a local agent isn't the model
size and it isn't decode — it's re-processing a prompt that barely changed.

`model.py` counts that, turn by turn, under two policies:

- **naive** — re-prefill the whole prompt each turn (what most stacks do),
- **kortex** — keep the prefix byte-stable so llama.cpp's KV cache reuses
  it, compress the tool schemas, bound the history tail.

```
$ python model.py
agentic session: 30 turns  |  prefill 120 tok/s  |  harness=True retrieval=True budget=12000

turn  naive prefill  kortex prefill   ratio
--------------------------------------------
   1         26,670          26,670    1.0x  <- turn 1 cold
   2         29,070           5,270    5.5x
  ...
  30         96,270          12,470    7.7x
--------------------------------------------
TOTAL      1,844,100         373,900    4.9x

  prefill wall time :  15367.5 s  ->   3115.8 s   (12252 s saved)
  prefill energy    :    46.10 Wh ->     9.35 Wh   (36.75 Wh saved)
```

**~5× less prefill compute for the same session, same model, same outputs.**
Break the prefix (a live file count in the system prompt — the bug fixed in
`prefix_cache.rs`) and it drops to 1.7×:

```
$ python model.py --broken-prefix
TOTAL      1,844,100       1,064,100    1.7x
```

## Why this matters beyond one IDE

The mechanisms here — stable prefix, KV-slot reuse, tool-schema
compression, a hard context budget — are **not model-specific and not
vendor-specific**. Any inference stack can do them. Frontier providers
mostly don't, because they bill per input token and a 5× cheaper agent
loop is 5× less revenue on the same work.

An open reference implementation that demonstrably runs the *same task* on
*the same model* for a fifth of the compute changes the conversation:
"prompt caching" stops being a premium API feature and becomes the
expected default. The pressure is the number, reproduced on a laptop.

This is a **model**, calibrated to what this IDE assembles (see the `Sizes`
class and the design docs it cites). It is not a live measurement — for
that, `real_run.md`.

## Reproduce / tune

```bash
python model.py --turns 60                    # longer session, gap widens
python model.py --no-harness                  # without tool-schema compression
python model.py --no-retrieval --budget 8000  # tighter tail
python model.py --json                        # machine-readable, for CI
```

Adjust `--prefill-tps` to your hardware (ROCmFPX 35B on an RX 9060 XT
prefills ≈ 120 tok/s) and `Sizes` to your prompt.

## Files

| file | what |
|---|---|
| `model.py` | the cost model + CLI |
| `real_run.md` | how to pull the *actual* numbers from a live IDE session (`throughput.ts` samples + `kortex_kvcache` `RoutingTrace`) |

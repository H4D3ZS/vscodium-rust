# Commercial / hosted services

The code in this repository is MIT-licensed — see [LICENSE](LICENSE). That
covers the IDE itself: the editor, the agent loop, indexing, the local
inference client, and everything else you can build and run entirely on your
own machine.

A small number of **hosted services**, run on Cyber-Ifrit infrastructure, are
not part of that grant:

- **Cloud AI routing** — proxying requests to hosted models for users who
  don't want to run a local backend.
- **Neural VFS compression** — the hosted variant of the workspace-memory
  compression pipeline.
- **Subscription management** — account, billing, and entitlement handling
  for the above.

None of the source for these services ships in this repository — they're
backend infrastructure the client talks to over the network, the same
relationship any IDE has with an optional cloud add-on. Nothing here
restricts building, forking, modifying, or redistributing the IDE itself;
it only means "log in and use Cyber-Ifrit's hosted AI" is a separate,
optional product from "run this IDE."

Using the hosted services requires a subscription — see
[cyberifrit.xyz/pricing](https://cyberifrit.xyz/pricing) for current plans
and the terms that govern them.

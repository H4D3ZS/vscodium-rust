# bench.bin — binary interchange format

Little-endian. One file holds the corpus, the queries, and (on write-back)
the results, so the Mojo binary and `bench.py` agree byte-for-byte.

```
offset  type        field
------  ----------  ---------------------------------------------------------
0       u32         magic  = 0x4B4E4E31   ("KNN1")
4       u32         n      = number of corpus vectors
8       u32         dim    = vector dimension
12      u32         nq     = number of query vectors
16      u32         k      = neighbours to return per query
20      u32         flags  = 0 (reserved)
24      f32[n*dim]  corpus, row-major, each row L2-normalised
...     f32[nq*dim] queries, row-major, each row L2-normalised
...     i64[nq*k]   out_ids     — written by the kernel; 0-filled on input
...     f32[nq*k]   out_scores  — written by the kernel; 0-filled on input
```

`out_ids[q*k + j]` is the row index (0..n) of the j-th nearest corpus
vector to query `q`, ordered by descending cosine; `out_scores[q*k + j]`
the cosine itself (== dot, since rows are unit vectors).

The kernel opens the file `mmap`-style, computes in place over the two
output regions, and exits 0. `bench.py` then reads those regions back.

---
description: Build retrieval-augmented generation pipelines
---

# RAG Pipeline

I will help you build a retrieval-augmented generation (RAG) pipeline.

## Guardrails
- Start simple, optimize later
- Measure retrieval quality
- Handle empty results gracefully
- Consider cost and latency

## Steps

### 1. Understand Requirements
Ask clarifying questions:
- What data sources need indexing?
- What types of queries will users make?
- Real-time or batch processing?
- Any existing vector store setup?

### 2. Design Pipeline
Components to set up:
- **Document Loader**: Ingest sources
- **Text Splitter**: Chunk documents
- **Embeddings**: Generate vectors
- **Vector Store**: Store and query
- **Retriever**: Find relevant chunks
- **Generator**: Create responses

### 3. Set Up Components
Configure each piece:
- Choose embedding model
- Select vector database (Pinecone, Chroma, etc.)
- Set chunk size and overlap
- Configure retrieval parameters

### 4. Implement Pipeline
Build the flow:
- Load and process documents
- Generate embeddings
- Store in vector database
- Create retrieval chain
- Connect to LLM for generation

### 5. Optimize
Improve quality:
- Tune chunk size
- Adjust retrieval k
- Add reranking
- Implement hybrid search

### 6. Verify
- Test with sample queries
- Check retrieval relevance
- Verify generated responses

## Principles
- Quality of retrieval = quality of output
- Start with small dataset
- Monitor and iterate

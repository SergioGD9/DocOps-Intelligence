from qdrant_client import QdrantClient
from qdrant_client.models import VectorParams, Distance, PointStruct
from typing import List, Dict, Any
from uuid import UUID

from .config import settings
from .embeddings import embed_dim

def get_client() -> QdrantClient:
    return QdrantClient(url=settings.QDRANT_URL)

def ensure_collection(client: QdrantClient) -> None:
    collection = settings.QDRANT_COLLECTION
    existing = [c.name for c in client.get_collections().collections]
    if collection not in existing:
        client.create_collection(
            collection_name=collection,
            vectors_config=VectorParams(size=embed_dim(), distance=Distance.COSINE),
        )

def upsert_chunks(
    client: QdrantClient,
    document_id: UUID,
    chunks: List[Dict[str, Any]],
    vectors: List[List[float]],
) -> None:
    collection = settings.QDRANT_COLLECTION

    points = []
    for ch, vec in zip(chunks, vectors):
        payload = {
            "document_id": str(document_id),
            "chunk_id": ch["chunk_id"],
            "text": ch["text"],
            "meta": ch.get("meta", {}),
        }
        points.append(PointStruct(
            id=ch["chunk_id"],  # string id ok
            vector=vec,
            payload=payload
        ))

    client.upsert(collection_name=collection, points=points)

def semantic_search(client: QdrantClient, query_vec: List[float], top_k: int = 5):
    collection = settings.QDRANT_COLLECTION
    return client.search(
        collection_name=collection,
        query_vector=query_vec,
        limit=top_k
    )

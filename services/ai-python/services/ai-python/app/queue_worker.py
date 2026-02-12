import json
import time
from uuid import UUID
import requests
import redis

from .config import settings
from .parser_stub import make_chunks_from_document
from .embeddings import generate_embedding
from .qdrant_client import get_client, ensure_collection, upsert_chunks

def _set_status(document_id: UUID, status: str) -> None:
    # MVP conceptual:
    # Ideal: Rust API expone endpoint PATCH /documents/{id}/status
    # Para no depender de eso ahora, dejamos solo log.
    # Si luego lo implementas en Rust, aquí lo llamas.
    try:
        # Ejemplo conceptual (si lo implementas):
        # requests.patch(f"{settings.DOCOPS_API_URL}/documents/{document_id}/status", json={"status": status}, timeout=5)
        pass
    except Exception:
        pass

def run_worker() -> None:
    r = redis.Redis.from_url(settings.REDIS_URL, decode_responses=True)
    qc = get_client()
    ensure_collection(qc)

    print(f"[worker] running. queue={settings.REDIS_QUEUE_NAME}")

    while True:
        # BLPOP: espera jobs
        item = r.blpop(settings.REDIS_QUEUE_NAME, timeout=5)
        if not item:
            continue

        _, payload = item
        try:
            job = json.loads(payload)
            if job.get("job_type") != "process_document":
                continue

            document_id = UUID(job["document_id"])
            print(f"[worker] process_document: {document_id}")

            _set_status(document_id, "processing")

            # MVP: generamos chunks simulados
            chunks = make_chunks_from_document(str(document_id))

            # Embeddings (dummy)
            vectors = [generate_embedding(ch["text"]) for ch in chunks]

            # Indexado en Qdrant
            upsert_chunks(qc, document_id, chunks, vectors)

            _set_status(document_id, "indexed")
            print(f"[worker] indexed: {document_id}")

        except Exception as e:
            print(f"[worker] error: {e}")
            # _set_status(document_id, "failed")  # si document_id existe
            time.sleep(1)

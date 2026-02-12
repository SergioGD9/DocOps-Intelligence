from fastapi import FastAPI
from .schemas import AskRequest, AskResponse, SearchRequest, SearchResponse, SearchHit, WorkerStatus
from .config import settings
from .embeddings import generate_embedding
from .qdrant_client import get_client, ensure_collection, semantic_search

app = FastAPI(title="DocOps AI Service", version="0.1.0")

@app.on_event("startup")
def startup():
    qc = get_client()
    ensure_collection(qc)

@app.get("/health")
def health():
    return {"ok": True, "service": "docops-ai"}

@app.post("/search", response_model=SearchResponse)
def search(req: SearchRequest):
    qc = get_client()
    query_vec = generate_embedding(req.query)
    results = semantic_search(qc, query_vec, top_k=req.top_k)

    hits = []
    for r in results:
        p = r.payload or {}
        hits.append(SearchHit(
            document_id=p.get("document_id"),
            chunk_id=p.get("chunk_id", ""),
            score=float(r.score),
            text=p.get("text", ""),
            meta=p.get("meta", {}),
        ))
    return SearchResponse(hits=hits)

@app.post("/ask", response_model=AskResponse)
def ask(req: AskRequest):
    # MVP: "RAG" simplificado:
    # 1) búsqueda semántica por la pregunta
    # 2) construir respuesta conceptual usando top chunks
    qc = get_client()
    query_vec = generate_embedding(req.question)
    results = semantic_search(qc, query_vec, top_k=3)

    citations = []
    context = []
    for r in results:
        p = r.payload or {}
        # filtrado por document_id si viene
        if p.get("document_id") != str(req.document_id):
            continue
        context.append(p.get("text", ""))
        citations.append({
            "chunk_id": p.get("chunk_id"),
            "meta": p.get("meta", {}),
            "score": float(r.score),
        })

    if not context:
        return AskResponse(
            answer="No encuentro contexto indexado para ese documento todavía. Prueba a esperar a que termine el procesado o sube el documento de nuevo.",
            citations=[]
        )

    # Respuesta conceptual (luego lo conectas a LLM)
    answer = (
        "Según el contenido indexado, la respuesta está relacionada con estos puntos:\n"
        f"- {context[0]}\n"
        + (f"- {context[1]}\n" if len(context) > 1 else "")
        + (f"- {context[2]}\n" if len(context) > 2 else "")
    )

    return AskResponse(answer=answer, citations=citations)

@app.get("/worker/status", response_model=WorkerStatus)
def worker_status():
    # simple: la API no controla el proceso del worker aquí
    return WorkerStatus(running=True, queue_name=settings.REDIS_QUEUE_NAME)

from pydantic import BaseModel, Field
from typing import List, Optional, Dict, Any
from uuid import UUID

class AskRequest(BaseModel):
    document_id: UUID
    question: str = Field(..., min_length=3, max_length=500)

class AskResponse(BaseModel):
    answer: str
    citations: List[Dict[str, Any]] = []

class SearchRequest(BaseModel):
    query: str = Field(..., min_length=2, max_length=300)
    top_k: int = 5

class SearchHit(BaseModel):
    document_id: UUID
    chunk_id: str
    score: float
    text: str
    meta: Dict[str, Any]

class SearchResponse(BaseModel):
    hits: List[SearchHit]

class WorkerStatus(BaseModel):
    running: bool
    queue_name: str

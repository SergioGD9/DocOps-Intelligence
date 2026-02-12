import hashlib
from typing import List

# MVP: embeddings "dummy" deterministas (NO IA real)
# Así todo funciona end-to-end sin depender de modelos.
# Luego cambias generate_embedding() por embeddings reales (OpenAI / HF / local).

EMBED_DIM = 64

def _hash_to_floats(text: str, dim: int = EMBED_DIM) -> List[float]:
    h = hashlib.sha256(text.encode("utf-8")).digest()
    # Expandimos bytes a floats repetibles
    vals = []
    for i in range(dim):
        b = h[i % len(h)]
        vals.append((b / 255.0) * 2.0 - 1.0)  # [-1, 1]
    return vals

def generate_embedding(text: str) -> List[float]:
    text = text.strip().lower()
    return _hash_to_floats(text)

def embed_dim() -> int:
    return EMBED_DIM

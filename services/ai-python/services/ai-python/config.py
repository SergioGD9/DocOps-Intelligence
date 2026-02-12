import os
from dotenv import load_dotenv

load_dotenv()

class Settings:
    AI_HOST: str = os.getenv("AI_HOST", "0.0.0.0")
    AI_PORT: int = int(os.getenv("AI_PORT", "8000"))

    DOCOPS_API_URL: str = os.getenv("DOCOPS_API_URL", "http://localhost:8080")

    REDIS_URL: str = os.getenv("REDIS_URL", "redis://localhost:6379")
    REDIS_QUEUE_NAME: str = os.getenv("REDIS_QUEUE_NAME", "docops:jobs")

    QDRANT_URL: str = os.getenv("QDRANT_URL", "http://localhost:6333")
    QDRANT_COLLECTION: str = os.getenv("QDRANT_COLLECTION", "docops_chunks")

    # Conceptual (si luego descargas el documento desde S3/MinIO)
    S3_ENDPOINT: str = os.getenv("S3_ENDPOINT", "http://localhost:9000")
    S3_BUCKET: str = os.getenv("S3_BUCKET", "docops")
    S3_ACCESS_KEY: str = os.getenv("S3_ACCESS_KEY", "minio")
    S3_SECRET_KEY: str = os.getenv("S3_SECRET_KEY", "minio123456")

settings = Settings()

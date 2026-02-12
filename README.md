# 🧠 DocOps Intelligence — Rust + Python (AI Document Platform)

Plataforma **end-to-end** para subir documentos (**PDF/DOCX/XLSX**) y convertirlos en información consultable:
- ✅ Ingesta segura y rápida (API en **Rust**)
- ✅ Procesado asíncrono (cola **Redis** + **worker**)
- ✅ Indexado semántico en vector DB (**Qdrant**)
- ✅ Servicio de IA en **Python** (FastAPI): **/search** + **/ask**
- ✅ Arquitectura lista para escalar (S3 compatible, observabilidad “pro” en roadmap)

> Este repo está pensado como proyecto portfolio “startup-like”: **arquitectura real**, decisiones justificadas y foco en producto.

---

## ✨ Features (MVP)
- 📤 **Upload** de documentos via API (multipart)
- 🧾 Registro en **PostgreSQL** con metadatos
- 🗂️ Almacenamiento en **S3 compatible** (MinIO/AWS S3)
- 🧵 Pipeline asíncrono:
  - Redis queue `docops:jobs`
  - Worker Python procesa e indexa chunks en Qdrant
- 🔎 **Búsqueda semántica**: `POST /search`
- 💬 **Ask your document** (demo RAG): `POST /ask` (con citas)

> Nota: este MVP usa embeddings **dummy deterministas** para funcionar sin modelos externos.
> En la sección “Upgrade IA real” se indica dónde conectar embeddings reales.

---

## 🧱 Arquitectura (visión)
**Rust (API Core)**
- Ingesta + validación + streaming de archivos
- Postgres (metadatos)
- S3 (objeto documento)
- Encola job de procesado (Redis)

**Python (AI Service + Worker)**
- Worker consume jobs → genera chunks → embeddings → indexa en Qdrant
- API FastAPI expone:
  - búsqueda semántica
  - “ask” demo con contexto

**Infra**
- Postgres
- Redis
- MinIO (S3 compatible)
- Qdrant (vector DB)

---

## 🗂️ Estructura del repo
```bash
docops-intelligence/
├─ infra/
│  ├─ docker-compose.yml
│  └─ .env.e

from typing import List, Dict, Any

# MVP: parser "stub"
# En producción:
# - PDF: PyMuPDF / pdfplumber
# - DOCX: python-docx
# - XLSX: openpyxl
#
# Aquí simulamos "chunks" para indexado.

def make_chunks_from_document(document_id: str) -> List[Dict[str, Any]]:
    # Simulación: 5 chunks fijos por documento
    base = [
        "Este documento describe términos y condiciones generales.",
        "Se mencionan fechas clave, importes y obligaciones de las partes.",
        "Incluye cláusulas sobre renovación, penalizaciones y cancelación.",
        "Contiene un resumen ejecutivo y apartados técnicos.",
        "Aplica recomendaciones y conclusiones finales.",
    ]
    chunks = []
    for i, t in enumerate(base, start=1):
        chunks.append({
            "chunk_id": f"{document_id}-{i}",
            "text": t,
            "meta": {"page": i, "section": f"Section {i}"}
        })
    return chunks

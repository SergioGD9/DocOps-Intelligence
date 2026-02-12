FROM python:3.11-slim

WORKDIR /app

# deps
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# code
COPY app ./app

EXPOSE 8000

# por defecto arrancamos API; el worker se lanza con otro servicio en compose
CMD ["uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "8000"]

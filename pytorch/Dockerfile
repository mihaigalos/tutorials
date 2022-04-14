FROM python:3.9.12-slim-buster

ENV VIRTUAL_ENV=/opt/venv
RUN python3 -m venv $VIRTUAL_ENV
ENV PATH="$VIRTUAL_ENV/bin:$PATH"

COPY . .
RUN pip install \
    --no-cache-dir \
    --timeout=300 \
    --requirement \
        requirements.txt

RUN python3 main.py \
    --epochs 15 \
    --save-model

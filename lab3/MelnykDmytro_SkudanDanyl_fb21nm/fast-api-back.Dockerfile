FROM python:3.11.4-alpine

ENV PYTHONDONTWRITEBYTECODE=1
ENV PYTHONUNBUFFERED=1
RUN apk update && apk add gcc python3-dev musl-dev openssl

WORKDIR /fast-api
RUN python -m venv venv
ENV PATH venv/bin:$PATH
RUN pip install --upgrade pip

COPY /fast-api /fast-api

RUN pip install -r back-service.txt

EXPOSE 8000/TCP
ENTRYPOINT ["./entrypoint.sh"]

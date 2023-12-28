FROM golang:1.18-alpine3.15 AS builder

WORKDIR /app
COPY . .

RUN go mod download
RUN set -x; apk add --no-cache \
    && CGO_ENABLED=0 go build -gcflags="all=-N -l" -a -installsuffix cgo -o ./bin/app cmd/main.go

FROM golang:1.18-alpine3.15

WORKDIR /

COPY --from=builder /app/bin .
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/config.yml .

RUN chmod +x ./app

ENTRYPOINT ["/app"]
FROM docker.io/alpine:3.22 AS builder

ARG SNAPCTL_VERSION
ARG TARGETARCH

RUN apk add --update --no-cache curl && \
    case ${TARGETARCH} in \
        "amd64") export ARCH_SUFFIX="x86_64-unknown-linux-musl" ;; \
        "arm64") export ARCH_SUFFIX="aarch64-unknown-linux-musl" ;; \
        *) echo "Unsupported architecture: ${TARGETARCH}" >&2; exit 1 ;; \
    esac && \
    curl \
        --silent \
        --location \
        --fail \
        --request GET "https://github.com/open-sori/snapctl/releases/download/${SNAPCTL_VERSION}/snapctl-${ARCH_SUFFIX}" \
        --output /tmp/snapctl && \
    chmod +x /tmp/snapctl

FROM scratch

ARG SNAPCTL_VERSION
ARG CREATED_DATE

# Ref from https://github.com/opencontainers/image-spec/blob/main/annotations.md
LABEL org.opencontainers.image.created="$CREATED_DATE"
LABEL org.opencontainers.image.authors="thibault@open-sori.dev"
LABEL org.opencontainers.image.url="https://github.com/orgs/open-sori/packages/container/package/snapctl"
LABEL org.opencontainers.image.documentation="https://snapctl.open-sori.dev"
LABEL org.opencontainers.image.source="https://github.com/open-sori/snapctl"
LABEL org.opencontainers.image.version="$SNAPCTL_VERSION"
LABEL org.opencontainers.image.revision="$SNAPCTL_VERSION"
LABEL org.opencontainers.image.vendor="open-sori"
LABEL org.opencontainers.image.licenses="GPL-3.0-or-later"
LABEL org.opencontainers.image.ref.name="snapctl"
LABEL org.opencontainers.image.title="snapctl"
LABEL org.opencontainers.image.description="snapcast ctl binary docker image"
LABEL org.opencontainers.image.base.name="ghcr.io/open-sori/snapctl:${SNAPCTL_VERSION}"

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

COPY --from=builder /tmp/snapctl /bin/snapctl

ENV SNAPSERVER_HOST="127.0.0.1"
ENV SNAPSERVER_PORT="1780"

ENTRYPOINT ["/bin/snapctl"]
# syntax=docker/dockerfile:1.4
FROM alpine

# Use secret during build
RUN --mount=type=secret,id=my_secret \
    cat /run/secrets/my_secret

# ================ ARNICA SECURITY ANNOTATION BLOCK START ================
LABEL org.opencontainers.image.source="https://github.com/Jaswanti-gitgoat/gh-1a"
LABEL org.opencontainers.image.path="1/Dockerfile"
# These automated labels, added by the security team, enhance traceability and security.
# For more details, see: https://docs.arnica.io/arnica-documentation/developers/adding-oci-tags-to-docker-images.
# To exclude this file, please replace this change with: #arnica-ignore followed by the dismissal reason.
# ================ ARNICA SECURITY ANNOTATION BLOCK END ================

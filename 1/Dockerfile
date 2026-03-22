# syntax=docker/dockerfile:1.4
FROM alpine

# Use secret during build
RUN --mount=type=secret,id=my_secret \
    cat /run/secrets/my_secret

#!/bin/sh

generate_signature() {
    echo "Generating RSA key pair..."
    openssl genpkey -algorithm RSA -out private_key.pem
    openssl rsa -pubout -in private_key.pem -out public_key.pem
    echo "Hello, World!" > data.txt
    openssl dgst -sha256 -sign private_key.pem -out signature.txt data.txt
    echo "Signature generated."
}

verify_signature() {
    echo "Verifying signature..."
    openssl dgst -sha256 -verify public_key.pem -signature $2 $3
    if [ $? -eq 0 ]; then
        echo "Signature is valid."
    else
        echo "Signature is invalid."
    fi
}

case "$1" in
    "generate")
        generate_signature
        ;;
    "verify")
        verify_signature
        ;;
    *)
        echo "Usage: entrypoint.sh [generate|verify]"
        exit 1
        ;;
esac

exit 0

from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric import rsa, padding
from cryptography.hazmat.primitives import hashes
from cryptography.exceptions import InvalidSignature

app = FastAPI()

BITS = 2048
PRIVATE_KEY_PATH = "private_key.pem"
PUBLIC_KEY_PATH = "public_key.pem"

class Message(BaseModel):
    message: str


class Signature(BaseModel):
    message: str
    signature: str


def load_private_key(private_key_path):
    with open(private_key_path, "rb") as key_file:
        private_key = serialization.load_pem_private_key(
            key_file.read(),
            password=None
        )
    return private_key


def load_public_key(public_key_path):
    with open(public_key_path, "rb") as key_file:
        public_key = serialization.load_pem_public_key(
            key_file.read()
        )
    return public_key


# Generate RSA keys and save to files
@app.post("/generate_keys/")
def generate_keys():
    private_key = rsa.generate_private_key(public_exponent=65537, key_size=BITS)
    public_key = private_key.public_key()

    with open(PRIVATE_KEY_PATH, "wb") as f:
        f.write(private_key.private_bytes(
            encoding=serialization.Encoding.PEM,
            format=serialization.PrivateFormat.PKCS8,
            encryption_algorithm=serialization.NoEncryption()
        ))

    with open(PUBLIC_KEY_PATH, "wb") as f:
        f.write(public_key.public_bytes(
            encoding=serialization.Encoding.PEM,
            format=serialization.PublicFormat.SubjectPublicKeyInfo
        ))

    return {
        "private_key_path": PRIVATE_KEY_PATH,
        "public_key_path": PUBLIC_KEY_PATH
    }


# Sign a message
@app.post("/sign/")
def sign_message(message: Message):
    private_key = load_private_key(PRIVATE_KEY_PATH)
    message_bytes = message.message.encode()
    signature = private_key.sign(
        message_bytes,
        padding.PSS(
            mgf=padding.MGF1(hashes.SHA256()),
            salt_length=padding.PSS.MAX_LENGTH
        ),
        hashes.SHA256()
    )
    return {"signature": signature.hex()}


# Verify a signature
@app.post("/verify/")
def verify_signature(signature: Signature):
    public_key = load_public_key(PUBLIC_KEY_PATH)
    message_bytes = signature.message.encode()
    signature_bytes = bytes.fromhex(signature.signature)
    try:
        public_key.verify(
            signature_bytes,
            message_bytes,
            padding.PSS(
                mgf=padding.MGF1(hashes.SHA256()),
                salt_length=padding.PSS.MAX_LENGTH
            ),
            hashes.SHA256()
        )
        return {"valid": True}
    except InvalidSignature:
        return {"valid": False}
    except Exception as e:
        raise HTTPException(status_code=400, detail=str(e))


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)

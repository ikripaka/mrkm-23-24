from fastapi import FastAPI, HTTPException, File, UploadFile
from Crypto.Signature import pkcs1_15
from Crypto.PublicKey import RSA
from Crypto.Hash import SHA256


app = FastAPI()


@app.get("/generate-signature")
async def get_key(
    private_key: UploadFile = File(...),
    text_file: UploadFile = File(...),
):
    if private_key is None:
        raise HTTPException(status_code=400, detail="Not found file")
    
    if text_file is None:
        raise HTTPException(status_code=400, detail="Not found file")
    
    with open(private_key, "r") as f:
        private_key = f.read()
    
    with open(text_file, "r") as f:
        message = f.read().encode()

    private_key = RSA.import_key(private_key)
    message_hash = SHA256.new(message)
    signature = pkcs1_15.new(private_key).sign(message_hash)
    return {"signature": signature.hex()}


@app.post("/verify_signature")
async def verify_signature(
    message: UploadFile = File(...),
    signature: UploadFile = File(...),
    public_key: UploadFile = File(...),
):
    if message is None:
        raise HTTPException(status_code=400, detail="Not found file")
    
    if signature is None:
        raise HTTPException(status_code=400, detail="Not found file")
    
    if public_key is None:
        raise HTTPException(status_code=400, detail="Not found file")
    
    with open(message, "r") as f:
        signature = f.read()
    
    with open(signature, "r") as f:
        message = f.read().encode()

    with open(public_key, "r") as f:
        public_key = f.read()
    
    public_key = RSA.import_key(public_key)
    message_hash = SHA256.new(message)
    signature = bytes.fromhex(signature)

    try:
        pkcs1_15.new(public_key).verify(message_hash, signature)
        return {"verified": True}
    except (ValueError, TypeError, pkcs1_15.pkcs1_15Error):
        raise HTTPException(status_code=400, detail="Signature verification failed")


from fastapi import FastAPI, HTTPException, File, UploadFile
from Crypto.Signature import pkcs1_15
from Crypto.PublicKey import RSA
from Crypto.Hash import SHA256


app = FastAPI()


@app.post("/get_key")
async def get_key(
    private_key: UploadFile = File(...),
    text_file: UploadFile = File(...),
):
    print(private_key)
    if private_key is None:
        raise HTTPException(status_code=400, detail="Not found file")
    
    if text_file is None:
        raise HTTPException(status_code=400, detail="Not found file")
    
    private_key = await private_key.read()
    message = await text_file.read()

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
    
    signature = await signature.read()
    message = await message.read()
    public_key = await public_key.read()
    
    public_key = RSA.import_key(public_key)
    message_hash = SHA256.new(message)
    signature = bytes.fromhex(signature)

    try:
        pkcs1_15.new(public_key).verify(message_hash, signature)
        return {"verified": True}
    except (ValueError, TypeError, pkcs1_15.pkcs1_15Error):
        raise HTTPException(status_code=400, detail="Signature verification failed")


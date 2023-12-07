from fastapi import FastAPI, HTTPException, File, UploadFile
import subprocess

app = FastAPI()


@app.get("/generate-signature")
async def generate_signature():
    try:
        subprocess.run(["/fast-api/eds.sh", "generate"])
        return {
            "message": "Signature generated successfully",
            "signature": 'heh'
            }
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Error generating signature: {str(e)}")


@app.post("/verify-signature")
async def verify_signature(
    signature: UploadFile = File(...),
    text_file: UploadFile = File(...),
):
    if signature is None:
        return {"message": "Not found file"}
    
    if text_file is None:
        return {"message": "Not found file"}
    
    with open(signature.filename, "wb") as f:
        f.write(signature.file.read())
    
    with open(text_file.filename, "wb") as f:
        f.write(text_file.file.read())
    
    try:
        result = subprocess.run(["/fast-api/eds.sh", "verify", signature.filename, text_file.filename], capture_output=True)
        output = result.stdout.decode().strip()
        if "Signature is valid" in output:
            return {"message": "Signature is valid"}
        else:
            return {"message": "Signature is invalid"}
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Error verifying signature: {str(e)}")

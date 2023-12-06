from fastapi import FastAPI, HTTPException
import subprocess

app = FastAPI()


@app.get("/generate-signature")
def generate_signature():
    try:
        subprocess.run(["docker", "run", "--rm", "-v", "$(pwd)/data:/data", "ecdsa-container", "generate"])
        return {"message": "Signature generated successfully"}
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Error generating signature: {str(e)}")


@app.get("/verify-signature")
def verify_signature():
    try:
        result = subprocess.run(["docker", "run", "--rm", "-v", "$(pwd)/data:/data", "ecdsa-container", "verify"], capture_output=True)
        output = result.stdout.decode().strip()
        if "Signature is valid" in output:
            return {"message": "Signature is valid"}
        else:
            return {"message": "Signature is invalid"}
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Error verifying signature: {str(e)}")

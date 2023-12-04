from flask import Flask, request, jsonify
from cryptography.hazmat.backends import default_backend
from cryptography.hazmat.primitives import serialization, hashes
from cryptography.hazmat.primitives.asymmetric import padding
from cryptography.exceptions import InvalidSignature
from datetime import datetime

app = Flask(__name__)

@app.route('/v2/blockchain-data/sign', methods=['POST'])
def sign_document():
    private_key = request.files['private_key'].read()
    document = request.files['document'].read()

    key = serialization.load_pem_private_key(
        private_key,
        password=None,
        backend=default_backend()
    )

    signature = key.sign(
        document,
        padding.PSS(
            mgf=padding.MGF1(hashes.SHA256()),
            salt_length=padding.PSS.MAX_LENGTH
        ),
        hashes.SHA256()
    )

    response_data = {
        "apiVersion": "2.0",
        "context": "You can add any text here.",
        "data": {
            "item": {
                "signature": signature.hex(),
                "timestamp": datetime.utcnow().strftime('%Y-%m-%dT%H:%M:%SZ')
            }
        }
    }

    return jsonify(response_data)

@app.route('/v2/blockchain-data/verify', methods=['POST'])
def verify_signature():
    public_key = request.files['public_key'].read()
    document = request.files['document'].read()
    signature = bytes.fromhex(request.form['signature'])

    key = serialization.load_pem_public_key(
        public_key,
        backend=default_backend()
    )

    try:
        key.verify(
            signature,
            document,
            padding.PSS(
                mgf=padding.MGF1(hashes.SHA256()),
                salt_length=padding.PSS.MAX_LENGTH
            ),
            hashes.SHA256()
        )
        response_data = {
            "apiVersion": "2.0",
            "requestId": "your_request_id",
            "context": "",
            "data": {
                "item": {"valid": True}
            }
        }
        return jsonify(response_data)
    except InvalidSignature:
        response_data = {
            "apiVersion": "2.0",
            "requestId": "your_request_id",
            "context": "",
            "data": {
                "item": {"valid": False}
            }
        }
        return jsonify(response_data), 400  # Indicate a bad request for signature verification failure

if __name__ == '__main__':
    app.run()

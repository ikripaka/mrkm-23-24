from flask import Flask, request, jsonify, abort
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.asymmetric import padding
from cryptography.hazmat.primitives.serialization import load_pem_private_key, load_pem_public_key
from cryptography.hazmat.backends import default_backend
from cryptography.exceptions import InvalidSignature
from datetime import datetime
from functools import wraps

app = Flask(__name__)

API_KEY = "your_secret_api_key_here"



def require_api_key(f):
    @wraps(f)
    def decorated_function(*args, **kwargs):
        if request.headers.get('X-API-Key') != API_KEY:
            # Custom JSON payload for unauthorized access
            payload = {
                "apiVersion": "2.0",
                "requestId": "requestId",
                "context": "Authorization Error",
                "data": {
                    "code": 401,
                    "message": "Invalid or missing API key"
                }
            }
            return jsonify(payload), 401
        return f(*args, **kwargs)
    return decorated_function


# Function to check allowed file
def allowed_file(filename):
    return '.' in filename and filename.rsplit('.', 1)[1].lower() in {'pem', 'txt'}


# Route for sign file
@app.route('/v2/document-processing/sign', methods=['POST'])
@require_api_key
def sign():
    # Check is all necessary files sent(private_key_file and data_file)
    if 'private_key' not in request.files or 'data_file' not in request.files:
        payload = {
            "apiVersion": "2.0",
            "requestId": "requestId",
            "context": "Sign Error",
            "data": {
                "code": 400,
                "message": "Missing key or data"
            }
        }
        return jsonify(payload), 400

    # Extracting data from request
    private_key_file = request.files['private_key']
    data_file = request.files['data_file']

    # Check is files set. Send 400 error(Request failed) if they not
    if private_key_file.filename == '' or data_file.filename == '':
        payload = {
            "apiVersion": "2.0",
            "requestId": "requestId",
            "context": "Sign Error",
            "data": {
                "code": 400,
                "message": "No selected file"
            }
        }
        return jsonify(payload), 400

    # Check is files have allowed format
    if allowed_file(private_key_file.filename) and allowed_file(data_file.filename):
        # Get instance of private key from *.pem file
        private_key = load_pem_private_key(
            private_key_file.read(), password=None, backend=default_backend()
        )
        data_to_sign = data_file.read()

        # Signing data and getting signature
        try:
            signature = private_key.sign(
                data_to_sign,
                padding.PSS(
                    mgf=padding.MGF1(hashes.SHA256()),
                    salt_length=padding.PSS.MAX_LENGTH
                ),
                hashes.SHA256()
            )
            # Prepare response
            payload = {
                "apiVersion": "2.0",
                "requestId": "requestId",
                "context": "Sign Document",
                "data": {
                    "items": {
                        "signature": signature.hex(),
                        "timestamp": datetime.utcnow().strftime('%Y-%m-%dT%H:%M:%SZ')
                    }
                }
            }
            # Return 200 status code with message when sign completed
            return jsonify(payload), 200
        except Exception as e:
            payload = {
                "apiVersion": "2.0",
                "requestId": "requestId",
                "context": "Sign Error",
                "data": {
                    "error": str(e),
                    "code": 400,
                    "message": "Singing Failed"
                }
            }
            return jsonify(payload), 400

    else:
        payload = {
            "apiVersion": "2.0",
            "requestId": "requestId",
            "context": "Sign Error",
            "data": {
                "code": 400,
                "message": "Invalid file type"
            }
        }
        return jsonify(payload), 400


# Route for verification
@app.route('/v2/document-processing/verify', methods=['POST'])
@require_api_key
def verify():
    # Check is all necessary data sent(private_key_file, data_file, signature)
    if 'public_key' not in request.files or 'data_file' not in request.files or 'signature' not in request.form:
        payload = {
            "apiVersion": "2.0",
            "requestId": "requestId",
            "context": "Verify error",
            "data": {
                "code": 400,
                "message": "Missing files or signature"
            }
        }
        return jsonify(payload), 400

    # Getting data from request
    public_key_file = request.files['public_key']
    data_file = request.files['data_file']
    signature_hex = request.form['signature']

    # Check if extracted data not empty. Return 400 status code if empty
    if public_key_file.filename == '' or data_file.filename == '' or len(signature_hex) == 0:
        payload = {
            "apiVersion": "2.0",
            "requestId": "requestId",
            "context": "Verify error",
            "data": {
                "code": 400,
                "message": "No data in received files"
            }
        }
        return jsonify(payload), 400

    # Check is files have allowed format
    if allowed_file(public_key_file.filename) and allowed_file(data_file.filename):
        # Get instance of public key from *.pem file
        public_key = load_pem_public_key(
            public_key_file.read(), backend=default_backend()
        )
        data_to_verify = data_file.read()

        try:
            signature = bytes.fromhex(signature_hex)
            # Verify signature
            public_key.verify(
                signature,
                data_to_verify,
                padding.PSS(
                    mgf=padding.MGF1(hashes.SHA256()),
                    salt_length=padding.PSS.MAX_LENGTH
                ),
                hashes.SHA256()
            )
            payload = {
                "apiVersion": "2.0",
                "requestId": "requestId",
                "context": "Verify success",
                "items": {
                    "status": 'Signature is valid',
                    "timestamp": datetime.utcnow().strftime('%Y-%m-%dT%H:%M:%SZ')
                }
            }
            return jsonify(payload)
        except InvalidSignature:
            payload = {
                "apiVersion": "2.0",
                "requestId": "requestId",
                "context": "Verify error",
                "data": {
                    "code": 400,
                    "message": "Signature is invalid"
                }
            }
            return jsonify(payload)
        except Exception as e:
            payload = {
                "apiVersion": "2.0",
                "requestId": "requestId",
                "context": "Unexpected server Error",
                "data": {
                    "error": str(e),
                    "code": 500,
                    "message": "Unexpected server Error"
                }
            }
            return jsonify(payload), 500
    else:
        payload = {
            "apiVersion": "2.0",
            "requestId": "requestId",
            "context": "Verify error",
            "data": {
                "code": 400,
                "message": "Invalid file types"
            }
        }
        return jsonify(payload), 400


if __name__ == '__main__':
    app.run(debug=True)

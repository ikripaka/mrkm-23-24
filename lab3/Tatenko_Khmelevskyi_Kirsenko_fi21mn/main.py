from flask import Flask, request, jsonify
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.asymmetric import padding
from cryptography.hazmat.primitives.serialization import load_pem_private_key, load_pem_public_key
from cryptography.hazmat.backends import default_backend
from cryptography.exceptions import InvalidSignature

app = Flask(__name__)


# Function to check allowed file
def allowed_file(filename):
    return '.' in filename and filename.rsplit('.', 1)[1].lower() in {'pem', 'txt'}


# Route for sign file
@app.route('/sign', methods=['POST'])
def sign():
    # Check is all necessary files sent(private_key_file and data_file)
    if 'private_key' not in request.files or 'data_file' not in request.files:
        return jsonify({'error': 'Missing key or data'}), 400

    # Extracting data from request
    private_key_file = request.files['private_key']
    data_file = request.files['data_file']

    # Check is files set. Send 400 error(Request failed) if they not
    if private_key_file.filename == '' or data_file.filename == '':
        return jsonify({'error': 'No selected file'}), 400

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
            # Return 200 status code with message when sign completed
            return jsonify({'signature': signature.hex()}), 200
        except Exception as e:
            return jsonify({'error': str(e)}), 500

    else:
        return jsonify({'error': 'Invalid file type'}), 400


# Route for verification
@app.route('/verify', methods=['POST'])
def verify():
    # Check is all necessary data sent(private_key_file, data_file, signature)
    if 'public_key' not in request.files or 'data_file' not in request.files or 'signature' not in request.form:
        return jsonify({'error': 'Missing files or signature'}), 400

    # Getting data from request
    public_key_file = request.files['public_key']
    data_file = request.files['data_file']
    signature_hex = request.form['signature']

    # Check if extracted data not empty. Return 400 status code if empty
    if public_key_file.filename == '' or data_file.filename == '' or len(signature_hex) == 0:
        return jsonify({'error': 'Empty data'}), 400

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
            return jsonify({'status': 'Signature is valid'})
        except InvalidSignature:
            return jsonify({'status': 'Signature is invalid'})
        except Exception as e:
            return jsonify({'error': str(e)}), 500
    else:
        return jsonify({'error': 'Invalid file types'}), 400


if __name__ == '__main__':
    app.run(debug=True)

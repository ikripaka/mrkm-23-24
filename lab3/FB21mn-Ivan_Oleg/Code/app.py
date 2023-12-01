import base64

from flask import Flask, render_template, request

from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.asymmetric import padding, rsa

private_key = rsa.generate_private_key(public_exponent=65537, key_size=2048)
public_key = private_key.public_key()

app = Flask(__name__)


@app.route('/')
def index():
    return render_template('index.html')


@app.route('/sign', methods=['POST'])
def sign_message():
    message = request.form['message'].encode()
    signature = private_key.sign(
        message,
        padding.PSS(mgf=padding.MGF1(hashes.SHA256()), salt_length=padding.PSS.MAX_LENGTH),
        hashes.SHA256()
    )
    signature_encoded = base64.b64encode(signature).decode()
    return render_template('sign.html', signature=signature_encoded)


@app.route('/verify', methods=['POST'])
def verify_signature():
    message = request.form['message'].encode()
    signature = request.form['signature'].encode()
    try:
        decoded_signature = base64.b64decode(signature)
        public_key.verify(
            decoded_signature,
            message,
            padding.PSS(mgf=padding.MGF1(hashes.SHA256()), salt_length=padding.PSS.MAX_LENGTH),
            hashes.SHA256()
        )
        return render_template('verify_result.html', result="Correct")
    except Exception as e:
        print(e)
        return render_template('verify_result.html', result="Invalid. Error: " + str(e))

if __name__ == '__main__':
    app.run(debug=True)

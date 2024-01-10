import requests
import time
import aiohttp
import asyncio

# Configuration
url_verify = "http://127.0.0.1:5000/v2/document-processing/verify"
public_key_path = "E:/public_key.pem"
data_file_path = "E:/document.txt"
api_key = "your_secret_api_key_here"


# Read files
with open(public_key_path, 'rb') as file:
    public_key = file.read()
with open(data_file_path, 'rb') as file:
    data_file = file.read()

# Sample invalid signature
invalid_signature = "9f291ca5c31055e9c420ace004e89f4e"
valid_signature = "58db87ec9bba360925dafa602352450124b8737b13c50a13163249e74f4d66d21efcee34e8424e563b40e44a89969c53956e2b8b8b34dbfda43f7e376f09a55fea7e6284bbc3e6f44263cb922033167a5ed439eb36e0a901b7965840d601a14afabefc265faccd46e84cd47a5a4043d3c337a281075c1f310f5c09714ed4f51ef8ad069e453452d912c87ccb3f823c1fc33707be3908b7934e144ca0d3200b6f7d977c1c023437d956960e89e986f307e1846d7687edb8cdad0f51483f4f1ba0d5c647461e50595721e97306b7e0a6ce7967be4a67fedf00288ce0f0946f747c716421c775ed966dda36850ce4ca663d32fd5c49cea3b2f2acaaa1bee1043611"


# Function to make a request and measure time
async def measure_response_time(signature):
    async with aiohttp.ClientSession() as session:
        with open(public_key_path, 'rb') as pk_file, open(data_file_path, 'rb') as data_file:
            data = aiohttp.FormData()
            data.add_field('public_key', pk_file, filename=public_key_path.split('/')[-1])
            data.add_field('data_file', data_file, filename=data_file_path.split('/')[-1])
            data.add_field('signature', signature)

            start_time = time.time()
            async with session.post(url_verify, data=data, headers={'X-API-Key': api_key}) as response:
                elapsed_time = time.time() - start_time
                response_json = await response.json()
                return elapsed_time, response.status, response_json


async def main():
    # time_taken, status, response2_json = await measure_response_time(valid_signature)
    # print(f"Valid Signature - Time taken: {time_taken} seconds, Status Code: {status} Response: {response2_json}")
    #
    time_taken, status, response1_json = await measure_response_time(invalid_signature)
    print(f"InvalidSignature - Time taken: {time_taken} seconds, Status Code: {status}, Response: {response1_json}")

asyncio.run(main())

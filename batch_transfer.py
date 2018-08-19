import json
import logging
import requests

LOLCOIN_TRANSFER_URL = 'http://api.lolcoin.lol-2018.lol/transfers/'

logging.basicConfig(level=logging.INFO)

for user_id, user in json.loads(open('/dev/stdin').read()).items():
    if not user['amount']:
        continue
    logging.info(f"Sending lolcoins to {user['full_name']}...")
    response = requests.post(
            LOLCOIN_TRANSFER_URL,
            headers={'Authorization': '%D0%AD%D1%82%D0%BE%20%D1%81%D0%B0%D0%BC%D1%8B%D0%B9%20%D0%B2%D0%B0%D0%B6%D0%BD%D1%8B%D0%B9%20%D0%B0%D0%BA%D0%BA%D0%B0%D1%83%D0%BD%D1%82%20%D0%B2%20%D0%9B%D0%9E%D0%9B%D0%B5'},
            json={'to': user_id, 'amount': user['amount']}
        )
    if response.status_code != 200:
        logging.error(f"Failed to send lolcoins to {user['full_name']}! Here is some debug info: {response.status_code} {response.reason} / '{response.text}'")
        continue
    data = response.json()
    if data['status'] != 'ok':
        logging.error(f"Failed to send lolcoins to {user['full_name']}! Here is some debug info: {response.status_code} {response.reason} / '{response.text}'")
        continue
    logging.info(f"{user['full_name']} has received {user['amount']} lolcoins.")

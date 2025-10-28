import requests
import json

binance_endpoint = "https://fapi.binance.com/fapi/v1/ticker/price"
bybit_endpoint = "https://api.bybit.com/v5/market/tickers?category=linear"

response = requests.get(binance_endpoint)

coins = {}

if response.status_code == 200:
    for coin in response.json():
        coin_name = coin.get("symbol")
        coin_price = coin.get("price")
        if coin_name is not None and coin_price is not None:
            coins[coin_name] = coin_price

    json_string = json.dumps(coins)
    print(json_string)
else:
    print(response)
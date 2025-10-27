FROM python:3.13-slim

WORKDIR /pump_dump_bot

COPY requirements.txt .

RUN pip install --no-cache-dir -r requirements.txt

COPY . .

CMD ["sh", "-c", "python3 birga.py & python3 bybit.py & python3 run_binance.py & tail -f /dev/null"]
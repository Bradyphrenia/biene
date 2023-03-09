FROM python:3.11.2-bullseye

Workdir /usr/src/bee_back

RUN python3 -m pip install --upgrade pip

COPY ./requirements.txt .
COPY . .

RUN python3 -m pip install -r requirements.txt

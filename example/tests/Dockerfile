FROM python:3.11

WORKDIR /tests

COPY ./requirements.txt /tests/requirements.txt

RUN pip install --no-cache-dir --upgrade -r /tests/requirements.txt

COPY . .

CMD [ "./wait-upstream.sh", "pytest"]
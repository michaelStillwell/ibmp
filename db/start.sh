#!/bin/bash

docker compose down

dbml2sql --postgres -o schema.sql

docker compose up --remove-orphans
#!/bin/bash

# Create expenses
echo "Creating expenses:"
curl -X POST -H "Content-Type: application/json" -d '{"id": 1, title": "Groceries", "amount": 100.5}' http://localhost:8080/expenses
echo
curl -X POST -H "Content-Type: application/json" -d '{"id": 2, title": "Rent", "amount": 800}' http://localhost:8080/expenses
echo

# Read expenses
echo "Reading expenses:"
curl -X GET http://localhost:8080/expenses
echo

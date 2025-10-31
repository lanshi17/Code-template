#!/bin/bash
cargo run -p user-service &
cargo run -p order-service &
cargo run -p payment-service &

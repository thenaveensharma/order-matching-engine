-- Enable TimescaleDB extension
CREATE EXTENSION IF NOT EXISTS timescaledb;

-- Create enum types
CREATE TYPE order_side AS ENUM ('bid', 'ask');
CREATE TYPE order_status AS ENUM ('new', 'filled', 'partially_filled', 'cancelled');

-- Create orders table
CREATE TABLE orders (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    price DECIMAL NOT NULL,
    size DECIMAL NOT NULL,
    remaining_size DECIMAL NOT NULL,
    side order_side NOT NULL,
    status order_status NOT NULL
);

-- Convert to hypertable
SELECT create_hypertable('orders', 'created_at');

-- Create indexes
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_side ON orders(side);
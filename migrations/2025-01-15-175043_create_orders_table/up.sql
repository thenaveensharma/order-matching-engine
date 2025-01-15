CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL,
    token_id INT NOT NULL,
    order_type TEXT NOT NULL CHECK (order_type IN ('buy', 'sell')),
    price NUMERIC(32, 8) NOT NULL,
    amount NUMERIC(32, 8) NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('open', 'filled', 'cancelled')),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

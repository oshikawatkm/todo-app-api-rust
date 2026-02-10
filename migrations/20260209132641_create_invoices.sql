-- Add migration script here
CREATE TABLE invoices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    amount NUMERIC(12, 2) NOT NULL,
    paid  BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'Asia/Tokyo') NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT (now() AT TIME ZONE 'Asia/Tokyo') NOT NULL
);
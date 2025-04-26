-- Add migration script here
CREATE TABLE IF NOT EXISTS tasks (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    title VARCHAR(255) NOT NULL,
    body TEXT NULL,
    state VARCHAR(11) CHECK (state IN ('TO_DO', 'IN_PROGRESS', 'DONE')) DEFAULT 'TO_DO',
    priority VARCHAR(6) CHECK (priority IN ('LOW', 'MEDIUM', 'HIGH')) DEFAULT 'MEDIUM',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
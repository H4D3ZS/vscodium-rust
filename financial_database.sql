-- Nexus Finance Database Schema (ERD Test)

-- Users Table
CREATE TABLE users (
    id VARCHAR(50) PRIMARY KEY,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    email VARCHAR(255) UNIQUE,
    tier VARCHAR(50),
    mfa_enabled BOOLEAN,
    last_login TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- User Preferences
CREATE TABLE user_preferences (
    user_id VARCHAR(50) PRIMARY KEY,
    currency VARCHAR(10),
    theme VARCHAR(50),
    email_notifications BOOLEAN,
    push_notifications BOOLEAN,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Accounts Table
CREATE TABLE accounts (
    account_id VARCHAR(50) PRIMARY KEY,
    user_id VARCHAR(50),
    account_type VARCHAR(50),
    institution VARCHAR(100),
    balance DECIMAL(15, 2),
    currency VARCHAR(10),
    interest_rate DECIMAL(5, 2),
    credit_limit DECIMAL(15, 2),
    status VARCHAR(20),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Transactions Table
CREATE TABLE transactions (
    tx_id VARCHAR(50) PRIMARY KEY,
    account_id VARCHAR(50),
    date DATE,
    amount DECIMAL(15, 2),
    merchant VARCHAR(255),
    category VARCHAR(100),
    verified BOOLEAN,
    city VARCHAR(100),
    is_recurring BOOLEAN,
    period VARCHAR(20),
    FOREIGN KEY (account_id) REFERENCES accounts(account_id)
);

-- Investment Portfolio Holdings
CREATE TABLE holdings (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(50),
    symbol VARCHAR(20),
    shares DECIMAL(15, 6),
    avg_price DECIMAL(15, 2),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Budgets
CREATE TABLE budgets (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(50),
    category VARCHAR(100),
    budget_limit DECIMAL(15, 2),
    spent DECIMAL(15, 2),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Financial Goals
CREATE TABLE financial_goals (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(50),
    name VARCHAR(255),
    target_amount DECIMAL(15, 2),
    current_amount DECIMAL(15, 2),
    deadline DATE,
    completed BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

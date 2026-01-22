-- ユーザーテーブル
-- CREATE TABLE users (
--     user_id INTEGER PRIMARY KEY AUTOINCREMENT,
--     username TEXT NOT NULL,
--     created_by TEXT NOT NULL DEFAULT "ADMIN",
--     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
--     updated_by TEXT NOT NULL DEFAULT "ADMIN",
--     updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
-- );

-- 収支テーブル
CREATE TABLE accountings (
    accounting_id INTEGER PRIMARY KEY AUTOINCREMENT,
    -- user_id INTEGER NOT NULL,
    -- tag_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    income_expenditure INTEGER NOT NULL, -- 0: 支出, 1: 収入
    price INTEGER DEFAULT 0,
    registered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    memo TEXT,
    created_by TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_by TEXT NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    -- FOREIGN KEY (user_id) REFERENCES users(user_id),
    -- FOREIGN KEY (tag_id) REFERENCES tags(tag_id)
);

-- 予算テーブル
-- CREATE TABLE budgets (
--     budget_id INTEGER PRIMARY KEY AUTOINCREMENT,
--     user_id INTEGER NOT NULL,
--     tag_id INTEGER NOT NULL,
--     amount INTEGER DEFAULT 0,
--     created_by TEXT NOT NULL,
--     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
--     updated_by TEXT NOT NULL,
--     updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
--     FOREIGN KEY (user_id) REFERENCES users(user_id),
--     FOREIGN KEY (tag_id) REFERENCES tags(tag_id)
-- );

-- カテゴリテーブル
-- CREATE TABLE tags (
--     tag_id INTEGER PRIMARY KEY AUTOINCREMENT,
--     name TEXT NOT NULL,
--     color TEXT NOT NULL
-- );

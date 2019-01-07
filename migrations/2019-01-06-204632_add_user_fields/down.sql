-- This file should undo anything in `up.sql`
ALTER TABLE users REMOVE COLUMN email;
ALTER TABLE users REMOVE COLUMN password_hash;


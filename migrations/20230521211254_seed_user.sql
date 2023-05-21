-- Add migration script here
INSERT INTO users (user_id, username, password)
VALUES (
    'd8e234a8-00f5-442f-81a3-57d51d15a500',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$BLsiBBGWjpccNOcdbEy3mQ$I0TNrsFsKetDE4NJ9NH76PJajWOsQSrDyFlOZrKHD0o'
);

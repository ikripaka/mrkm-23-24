-- +goose Up
-- +goose StatementBegin
create table users (
    login varchar(255) primary key,
    password_hash varchar,

    key json
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
SELECT 'down SQL query';
-- +goose StatementEnd

-- +goose Up
-- +goose StatementBegin
create table users (
    nickname varchar(255) primary key,

    public_key bytea
);

create index idx_users_public_key  on users using hash(public_key);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
SELECT 'down SQL query';
-- +goose StatementEnd

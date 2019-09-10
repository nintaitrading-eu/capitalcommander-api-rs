CREATE TABLE T_ACCOUNT
(
    account_id bigserial not null,
    name varchar(4000) not null,
    description varchar(4000) not null,
    is_deleted boolean not null default 'false',
    date_created timestamp with time zone not null,
    date_modified timestamp with time zone not null default (now() at time zone 'UTC'),
    constraint pk_account_id primary key(account_id),
    unique(account_id),
    unique(name),
    check(extract(timezone from date_created) = '0'),
    check(extract(timezone from date_modified) = '0')
    );
COMMENT ON TABLE T_ACCOUNT IS 'Contains records with account names, as used in ledger. Accounts can be added manually or imported from the ledger accounts file.';
COMMENT ON COLUMN T_ACCOUNT.is_deleted IS 'When a record is deleted. Nothing gets actually removed.';
COMMENT ON COLUMN T_ACCOUNT.date_created IS 'Date, on which the record was created.';
COMMENT ON COLUMN T_ACCOUNT.date_modified IS 'Date, on which the record was last modified.';

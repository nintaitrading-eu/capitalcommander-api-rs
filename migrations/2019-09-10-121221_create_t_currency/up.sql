CREATE TABLE T_CURRENCY
(
    currency_id bigserial not null,
    code varchar(3) not null,
    description varchar(4000) not null,
    is_deleted boolean not null default 'false',
    date_created timestamp with time zone not null,
    date_modified timestamp with time zone not null default (now() at time zone 'UTC'),
    constraint pk_currency_id primary key(currency_id),
    unique(currency_id),
    unique(code),
    check(extract(timezone from date_created) = '0'),
    check(extract(timezone from date_modified) = '0')
);
COMMENT ON TABLE T_CURRENCY IS 'Table that holds ISO currency codes.';
COMMENT ON COLUMN T_CURRENCY.currency_id IS 'Primary key.';
COMMENT ON COLUMN T_CURRENCY.code IS '3 character ISO code for the currency.';
COMMENT ON COLUMN T_CURRENCY.description IS 'Full currency name.';
COMMENT ON COLUMN T_CURRENCY.is_deleted IS 'When a record is deleted. Nothing gets actually removed.';
COMMENT ON COLUMN T_CURRENCY.date_created IS 'Date, on which the record was created.';
COMMENT ON COLUMN T_CURRENCY.date_modified IS 'Date, on which the record was last modified.';

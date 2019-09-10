CREATE TABLE T_MARKET
(
    market_id bigserial not null,
    code varchar(50) not null,
    name varchar(50) not null,
    country char(2) not null,
    is_deleted boolean not null default 'false',
    date_created timestamp with time zone not null,
    date_modified timestamp with time zone not null default (now() at time zone 'UTC'),
    constraint pk_market_id primary key(market_id),
    unique(market_id),
    unique(code),
    check(extract(timezone from date_created) = '0'),
    check(extract(timezone from date_modified) = '0')
);
COMMENT ON TABLE T_MARKET IS 'Contains a list of markets you can trade. E.g.: ''cfd .gold'', ''ebr''';
COMMENT ON COLUMN T_MARKET.code IS 'A unique short code representation to use in applications. e.g.: EBR';
COMMENT ON COLUMN T_MARKET.country IS 'Iso country code, 2 characters.';
COMMENT ON COLUMN T_MARKET.is_deleted IS 'When a record is deleted. Nothing gets actually removed.';

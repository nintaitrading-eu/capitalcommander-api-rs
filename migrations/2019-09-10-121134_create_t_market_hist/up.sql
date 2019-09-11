CREATE TABLE T_MARKET_HIST
(
    market_hist_id bigserial not null,
    market_id bigint not null,
    code varchar(50) not null,
    name varchar(50) not null,
    country char(2) not null,
    is_deleted int not null,
    date_created timestamp with time zone not null,
    date_modified timestamp with time zone not null,
    date_hist_created timestamp with time zone not null,
    date_hist_modified timestamp with time zone not null default (now() at time zone 'UTC'),
    constraint pk_market_hist_id primary key(market_hist_id),
    constraint fk_T_MARKET_HIST_market_id foreign key (market_id) references T_MARKET (market_id),
    check(extract(timezone from date_created) = '0'),
    check(extract(timezone from date_modified) = '0'),
    check(extract(timezone from date_hist_created) = '0'),
    check(extract(timezone from date_hist_modified) = '0')
);
COMMENT ON TABLE T_MARKET_HIST IS 'History table for T_MARKET.';

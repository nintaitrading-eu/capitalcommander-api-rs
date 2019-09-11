CREATE TABLE T_CURRENCY_EXCHANGE
(
    currency_exchange_id serial not null,
    currency_from_id int not null,
    currency_to_id int not null,
    exchange_rate decimal(18,6) not null,
    date_created timestamp with time zone not null,
    date_modified timestamp with time zone not null default (now() at time zone 'UTC'),
    constraint pk_currency_exchange_id primary key(currency_exchange_id),
    constraint fk_T_CURRENCY_EXCHANGE_currency_from_id foreign key(currency_from_id) references T_CURRENCY(currency_id),
    constraint fk_T_CURRENCY_EXCHANGE_currency_to_id foreign key(currency_to_id) references T_CURRENCY(currency_id),
    check(extract(timezone from date_created) = '0'),
    check(extract(timezone from date_modified) = '0')
);
COMMENT ON TABLE T_CURRENCY IS 'Table that holds the currency exchange rates.';

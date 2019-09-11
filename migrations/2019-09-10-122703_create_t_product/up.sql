CREATE TABLE T_PRODUCT
(
    product_id bigserial not null,
    name varchar(50) not null,
    description varchar(4000) not null,
    product_type_id bigint not null,
    currency_id bigint not null,
    market_id bigint not null,
    product_tick_info_id bigint,
    is_deleted boolean not null default 'false',
    date_created timestamp with time zone not null,
    date_modified timestamp with time zone not null default (now() at time zone 'UTC'),
    constraint pk_product_id primary key(product_id),
    unique(product_id),
    constraint fk_T_PRODUCT_currency_id foreign key(currency_id) references T_CURRENCY(currency_id),
    constraint fk_T_PRODUCT_market_id foreign key(market_id) references T_MARKET(market_id),
    constraint fk_T_PRODUCT_product_type_id foreign key(product_type_id) references T_PRODUCT_TYPE(product_type_id),
    constraint fk_T_PRODUCT_product_tick_info_id foreign key(product_tick_info_id) references T_PRODUCT_TICK_INFO(product_tick_info_id),
    check(extract(timezone from date_created) = '0'),
    check(extract(timezone from date_modified) = '0')
);
COMMENT ON TABLE T_PRODUCT IS 'Table with products. This is the detailed version of the type of contract or other product_type you deal with. E.g: CCZ3.cfd';
COMMENT ON COLUMN T_PRODUCT.product_tick_info_id IS 'May be NULL! The link is only valid for products that are cfd or futures contracts.';

CREATE TABLE T_PRODUCT_TYPE
(
    product_type_id bigserial not null,
    name varchar(50) not null,
    description varchar(4000) not null,
    is_deleted boolean not null default 'false',
    date_created timestamp with time zone not null,
    date_modified timestamp with time zone not null default (now() at time zone 'UTC'),
    constraint pk_product_type_id primary key(product_type_id),
    unique(product_type_id),
    check(extract(timezone from date_created) = '0'),
    check(extract(timezone from date_modified) = '0')
);
COMMENT ON TABLE T_PRODUCT_TYPE IS 'Table with product type, e.g.: cfd, stock, fund, ...';

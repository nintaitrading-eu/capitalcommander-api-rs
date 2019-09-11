CREATE TABLE T_PARAMETER_DATATYPE
(
    parameter_datatype_id bigserial not null,
    datatype varchar(512) not null,
    is_deleted boolean not null default 'false',
    date_created timestamp with time zone not null,
    date_modified timestamp with time zone not null default (now() at time zone 'UTC'),
    constraint pk_parameter_datatype_id primary key(parameter_datatype_id),
    unique(parameter_datatype_id),
    check(extract(timezone from date_created) = '0'),
    check(extract(timezone from date_modified) = '0')
    );
COMMENT ON TABLE T_PARAMETER_DATATYPE IS 'Table that contains datatypes that may be used for the parameters. Please use valid datatypes, so a conversion can succeed.';


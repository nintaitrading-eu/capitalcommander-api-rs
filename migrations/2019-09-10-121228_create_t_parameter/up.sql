CREATE TABLE T_PARAMETER
(
    parameter_id bigserial not null,
    name varchar(50) not null,
    value varchar(512) not null,
    value_default varchar(512) not null,
    description varchar(4000) not null,
    datatype varchar(50) not null,
    is_deleted boolean not null default 'false',
    date_created timestamp with time zone not null,
    date_modified timestamp with time zone not null default (now() at time zone 'UTC'),
    constraint pk_parameter_id primary key(parameter_id),
    unique(parameter_id),
    unique(name),
    check(extract(timezone from date_created) = '0'),
    check(extract(timezone from date_modified) = '0')
);
COMMENT ON TABLE T_PARAMETER IS 'Table that contains global parameters, with default values and descriptions.';
COMMENT ON COLUMN T_PARAMETER.parameter_id IS 'Primary key.';
COMMENT ON COLUMN T_PARAMETER.name IS 'Name of parameter.';
COMMENT ON COLUMN T_PARAMETER.value IS 'Value of parameter.';
COMMENT ON COLUMN T_PARAMETER.value_default IS 'Default value of parameter.';
COMMENT ON COLUMN T_PARAMETER.is_deleted IS 'When a record is deleted. Nothing gets actually removed.';
COMMENT ON COLUMN T_PARAMETER.datatype IS 'Note: add check constraint: integer, string, decimal(18,6)';
COMMENT ON COLUMN T_PARAMETER.description IS 'Description of parameter.';
COMMENT ON COLUMN T_PARAMETER.date_created IS 'Date, on which the record was created.';
COMMENT ON COLUMN T_PARAMETER.date_modified IS 'Date, on which the record was modified.';

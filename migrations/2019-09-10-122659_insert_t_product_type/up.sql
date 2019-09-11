insert into t_product_type(name, description, date_created) values('cfd', 'Contracts for difference', (select now() at time zone 'UTC'));

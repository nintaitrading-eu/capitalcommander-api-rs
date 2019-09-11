insert into t_parameter_datatype(datatype, date_created)
     values ('int', (select now() at time zone 'UTC')),
            ('string', (select now() at time zone 'UTC')),
            ('decimal(18,6)', (select now() at time zone 'UTC')),
            ('double', (select now() at time zone 'UTC'));

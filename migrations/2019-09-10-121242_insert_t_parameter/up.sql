/* Note: 4 = double, 1 = int
 * See insert script for parameter_datatype
 */
insert into t_parameter(name, description, value, value_default, datatype, date_created)
     values ('pool_margin', 'Margin to leave of pool.', '0.25', '0.25', 4, (select now() at time zone 'UTC')),
            ('risk', 'Percent risk of pool we are willing to take initially.', '2.0', '2.0', 4, (select now() at time zone 'UTC')),
            ('default_timezone', 'Default timezone for the users local time.', 'Europe/Brussels', 'Europe/Brussels', 2, (select now() at time zone 'UTC')),
            ('default_currency_from', 'Default index for the currency_from combobox.', '2', '2', 1, (select now() at time zone 'UTC')),
            ('default_currency_to', 'Default index for the currency_to combobox.', '1', '1', 1, (select now() at time zone 'UTC'));

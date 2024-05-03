
This is a brief of each example, with a link to its documentation. General guidance 
on how to build and 
use examples is provided here.

# Get started

Clone the 

# Integrate your extension with `osquery`

There are different options to integrate an extension built with `osquery-rust`.

1. Start osqueryi and provide the path to the extension with `--extension`.
   - osquery and extension run under the same user
   - unprivileged user suffice

2. Start osqueryi. Then start the extension and provide the path to osquery's socket with `--socket`.
   - osquery and extension might run under different users
   - user running the extension requires access to osquery's socket

3. Let osqueryd autoload extensions.
   - extension must be owned by a privileged user (root / Administrator)

While option 1 and 2 are good for ad hoc usage and fast iterations during development, 
option 3 is best suited for production deployments.

In either case, users can check the status of extensions loaded by querying osquery's 
internal tables.

This tells us that the extension `table-proc-meminfo` is loaded.

    osquery> SELECT * FROM osquery_extensions WHERE uuid != 0;
    +-------+--------------------+---------+-------------+--------------------------------------+-----------+
    | uuid  | name               | version | sdk_version | path                                 | type      |
    +-------+--------------------+---------+-------------+--------------------------------------+-----------+
    | 58615 | table-proc-meminfo | 1.0     | Unknown     | /home/tobias/.osquery/shell.em.58615 | extension |
    +-------+--------------------+---------+-------------+--------------------------------------+-----------+
    osquery>

Looking at osquery's registry, we get the table(s) provided by extensions.

    osquery> SELECT * FROM osquery_registry WHERE owner_uuid != 0;
    +----------+--------------+------------+----------+--------+
    | registry | name         | owner_uuid | internal | active |
    +----------+--------------+------------+----------+--------+
    | table    | proc_meminfo | 58615      | 0        | 1      |
    +----------+--------------+------------+----------+--------+
    osquery>

Hey this is SQL, so we can get both at once.

    osquery> SELECT uuid, os_ext.name AS extension, version, type, registry, os_reg.name, active 
               FROM osquery_registry os_reg 
               JOIN osquery_extensions os_ext 
                 ON os_reg.owner_uuid = os_ext.uuid 
              WHERE owner_uuid != 0;
    +-------+--------------------+---------+-----------+----------+--------------+--------+
    | uuid  | extension          | version | type      | registry | name         | active |
    +-------+--------------------+---------+-----------+----------+--------------+--------+
    | 58615 | table-proc-meminfo | 1.0     | extension | table    | proc_meminfo | 1      |
    +-------+--------------------+---------+-----------+----------+--------------+--------+
    osquery>

Knowing the table's name, we can get its schema.

    osquery> PRAGMA TABLE_INFO("proc_meminfo");
    +-----+-------------------+--------+---------+------------+----+
    | cid | name              | type   | notnull | dflt_value | pk |
    +-----+-------------------+--------+---------+------------+----+
    | 0   | memtotal          | BIGINT | 0       |            | 0  |
    | 1   | memfree           | BIGINT | 0       |            | 0  |
    | 2   | memavailable      | BIGINT | 0       |            | 0  |
    | 3   | buffers           | BIGINT | 0       |            | 0  |
    | 4   | cached            | BIGINT | 0       |            | 0  |
    [...]
    +-----+-------------------+--------+---------+------------+----+

## Integrate via --extension

    $ osqueryi --version
    osqueryi version 5.7.0
    $ osqueryi --extension target/debug/table-proc-meminfo
    Status 0 registering extension table-proc-meminfo (58615): OK
    osquery> SELECT * FROM osquery_extensions;
    +-------+--------------------+---------+-------------+--------------------------------------+-----------+
    | uuid  | name               | version | sdk_version | path                                 | type      |
    +-------+--------------------+---------+-------------+--------------------------------------+-----------+
    | 0     | core               | 5.7.0   | 0.0.0       | /home/tobias/.osquery/shell.em       | core      |
    | 58615 | table-proc-meminfo | 1.0     | Unknown     | /home/tobias/.osquery/shell.em.58615 | extension |
    +-------+--------------------+---------+-------------+--------------------------------------+-----------+
    osquery>

## Integrate via --socket

In one shell, start `osqueryi` and find out the socket:

    $ osqueryi --version
    osqueryi version 5.7.0
    osquery> SELECT path AS socket FROM osquery_extensions WHERE uuid = 0;
    +--------------------------------+
    | socket                         |
    +--------------------------------+
    | /home/tobias/.osquery/shell.em |
    +--------------------------------+
    osquery>

In a second shell, start the extension from the project root pointing to the socket:

    $ target/debug/table-proc-meminfo --socket /home/tobias/.osquery/shell.em
    Status 0 registering extension table-proc-meminfo (39134): OK
    
Now, go back to the first shell and work with the extension through osquery's SQL interface.

# Links

[Developing osquery Extensions](https://osquery.readthedocs.io/en/stable/deployment/extensions/)

[The osquery SDK](https://osquery.readthedocs.io/en/stable/development/osquery-sdk/)

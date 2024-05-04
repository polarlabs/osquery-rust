To get started with the examples provided here, we provide general guidance on how to build and integrate 
the extensions with Osquery.

For the time being, we have only one example to link to. Anyway, expect this to grow over time.

# Get started

It's best to clone the complete repo and copy the example you would like to work with to an independent folder.

# Integrate your extension with Osquery

There are different options to integrate an extension built with `osquery-rust`.

1. Start `osqueryi` and provide the path to the extension with `--extension`.
   - Osquery and extension run under the same user.
   - unprivileged user suffice.

2. Start `osqueryi`. Then start the extension and provide the path to Osquery's socket with `--socket`.
   - Osquery and extension might run under different users.
   - user running the extension requires access to Osquery's socket.

3. Let `osqueryd` autoload extensions.
   - extension must be owned by a privileged user (root / Administrator).

While option 1 and 2 are good for ad hoc usage and fast iterations during development, 
option 3 is best suited for production deployments.

In either case, users can check the status of extensions loaded by querying Osquery's 
internal tables.

This tells us that the extension `table-proc-meminfo` is loaded.

    osquery> SELECT * FROM osquery_extensions WHERE uuid != 0;
    +-------+--------------------+---------+-------------+--------------------------------------+-----------+
    | uuid  | name               | version | sdk_version | path                                 | type      |
    +-------+--------------------+---------+-------------+--------------------------------------+-----------+
    | 58615 | table-proc-meminfo | 1.0     | Unknown     | $HOME/.osquery/shell.em.58615 | extension |
    +-------+--------------------+---------+-------------+--------------------------------------+-----------+
    osquery>

Looking at Osquery's registry, we get the table(s) provided by extensions.

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
    | 0     | core               | 5.7.0   | 0.0.0       | $HOME/.osquery/shell.em       | core      |
    | 58615 | table-proc-meminfo | 1.0     | Unknown     | $HOME/.osquery/shell.em.58615 | extension |
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
    | $HOME/.osquery/shell.em |
    +--------------------------------+
    osquery>

In a second shell, start the extension from the project root pointing to the socket:

    $ target/debug/table-proc-meminfo --socket $HOME/.osquery/shell.em
    Status 0 registering extension table-proc-meminfo (39134): OK
    
Now, go back to the first shell and work with the extension through Osquery's SQL interface.

# Links

[Developing Osquery Extensions](https://osquery.readthedocs.io/en/stable/deployment/extensions/)

[The Osquery SDK](https://osquery.readthedocs.io/en/stable/development/osquery-sdk/)

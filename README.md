# DSG

🚧 Work in progress 🚧

Data Set Generator is a CLI tool which generates (postgresql) SQL DML based on the input schema.
Optimized to output large data sets for database query performance benchmarking.

## CLI Features

- generate SQL DML with fake data
- direct output to file directly (stream)

## Usage version 0.1 (design)

Version 0.1 is intended to test a UX of the CLI tool.

### Roadmap

- [x] basic generator from model to DDL
- [x] handle basic constraints (UNIQUE)
- [ ] json model representation reader
- [ ] json schema validation (jsonschema or boon?)
- [ ] have introspector to model command
- [ ] semantic value generation based on faker annotations
- [ ] foreign keys and multiple tables
- [ ] automatic semantic inferrence for values from original data model/schema
- [ ] experiment with concurrent generation

### Generate data using a config file

```bash
dsg generate
    -c <path to input config>
    -o <path to output sql file>
    -s <data set size>
```

### Introspect existing tables into a config

```bash
dsg introspect
    -o <path to resulting introspection config>
    -h <host>
    -p <port>
    -u <user>
```

### Generate data directly into a live db instance

```bash
dsg write
    -c <path to input config>
    -h <host>
    -p <port>
    -u <user>
```

# Problems

- postres init sql times out already on 500K entries in a simple model, we'll have add direct write

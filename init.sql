CREATE TABLE table_normal (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    type VARCHAR(255) NOT NULL
);

CREATE INDEX idx_name ON table_normal (name);
CREATE INDEX idx_type ON table_normal (type);

CREATE TABLE table_partitioned (
    id VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    type VARCHAR(255) NOT NULL,
    PRIMARY KEY (type, id)
) 
PARTITION BY LIST COLUMNS(type) (
    PARTITION p0 VALUES IN ('example_type'),
    PARTITION p1 VALUES IN ('another_type'),
    PARTITION p2 VALUES IN ('yet_another_type')
);

CREATE INDEX idx_id ON table_partitioned (id);
CREATE INDEX idx_name ON table_partitioned (name);
CREATE INDEX idx_type ON table_partitioned (type);
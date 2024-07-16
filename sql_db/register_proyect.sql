-- Create the Family table
CREATE TABLE FAMILY(
	ID_FAMILY SERIAL,
	SCIENTIFIC_NAME TEXT NOT NULL,
	CONSTRAINT PK_FAMILY PRIMARY KEY (ID_FAMILY)
);

-- Insert the family
INSERT INTO
	FAMILY(SCIENTIFIC_NAME)
VALUES
	('Formicidae')
RETURNING
	ID_FAMILY;

-- Create the Subfamily table
CREATE TABLE SUBFAMILY (
	ID_SUBFAMILY SERIAL,
	ID_FAMILY INTEGER NOT NULL,
	SCIENTIFIC_NAME TEXT NOT NULL,
	CONSTRAINT FK_SUBFAMILY_FAMILY FOREIGN KEY (ID_FAMILY) REFERENCES FAMILY(ID_FAMILY),
	CONSTRAINT PK_SUBFAMILY PRIMARY KEY (ID_SUBFAMILY)
);

-- Insert the subfamilies
INSERT INTO
	SUBFAMILY (ID_FAMILY, SCIENTIFIC_NAME)
VALUES
	(
		(
			SELECT
				ID_FAMILY
			FROM
				FAMILY
			WHERE
				SCIENTIFIC_NAME = 'Formicidae'
		),
		'Amblyoponinae'
	)
RETURNING
	ID_SUBFAMILY;

INSERT INTO
	SUBFAMILY (ID_FAMILY, SCIENTIFIC_NAME)
VALUES
	(
		(
			SELECT
				ID_FAMILY
			FROM
				FAMILY
			WHERE
				SCIENTIFIC_NAME = 'Formicidae'
		),
		'Ponerinae'
	)
RETURNING
	ID_SUBFAMILY;

INSERT INTO
	SUBFAMILY (ID_FAMILY, SCIENTIFIC_NAME)
VALUES
	(
		(
			SELECT
				ID_FAMILY
			FROM
				FAMILY
			WHERE
				SCIENTIFIC_NAME = 'Formicidae'
		),
		'Myrmicinae'
	)
RETURNING
	ID_SUBFAMILY;

-- Create the Tribe table
CREATE TABLE TRIBE (
	ID_TRIBE SERIAL,
	ID_SUBFAMILY INTEGER NOT NULL,
	SCIENTIFIC_NAME TEXT NOT NULL,
	CONSTRAINT FK_TRIBE_SUBFAMILY FOREIGN KEY (ID_SUBFAMILY) REFERENCES SUBFAMILY (ID_SUBFAMILY),
	CONSTRAINT PK_TRIBE PRIMARY KEY (ID_TRIBE)
);

-- Insert the tribes
INSERT INTO
	TRIBE (ID_SUBFAMILY, SCIENTIFIC_NAME)
VALUES
	(
		(
			SELECT
				ID_SUBFAMILY
			FROM
				SUBFAMILY
			WHERE
				SCIENTIFIC_NAME = 'Amblyoponinae'
		),
		'Amblyoponini'
	)
RETURNING
	ID_TRIBE;

INSERT INTO
	TRIBE (ID_SUBFAMILY, SCIENTIFIC_NAME)
VALUES
	(
		(
			SELECT
				ID_SUBFAMILY
			FROM
				SUBFAMILY
			WHERE
				SCIENTIFIC_NAME = 'Ponerinae'
		),
		'Ponerini'
	)
RETURNING
	ID_TRIBE;

INSERT INTO
	TRIBE (ID_SUBFAMILY, SCIENTIFIC_NAME)
VALUES
	(
		(
			SELECT
				ID_SUBFAMILY
			FROM
				SUBFAMILY
			WHERE
				SCIENTIFIC_NAME = 'Myrmicinae'
		),
		'Attini'
	)
RETURNING
	ID_TRIBE;

INSERT INTO
	TRIBE (ID_SUBFAMILY, SCIENTIFIC_NAME)
VALUES
	(
		(
			SELECT
				ID_SUBFAMILY
			FROM
				SUBFAMILY
			WHERE
				SCIENTIFIC_NAME = 'Myrmicinae'
		),
		'Solenopsidini'
	)
RETURNING
	ID_TRIBE;

-- Create the Genus table
CREATE TABLE GENUS (
	ID_GENUS SERIAL,
	ID_TRIBE INTEGER NOT NULL,
	SCIENTIFIC_NAME TEXT NOT NULL,
	CONSTRAINT FK_GENUS_TRIBE FOREIGN KEY (ID_TRIBE) REFERENCES TRIBE (ID_TRIBE),
	CONSTRAINT PK_GENUS PRIMARY KEY (ID_GENUS)
);

-- Insert the genera
INSERT INTO
	GENUS (ID_TRIBE, SCIENTIFIC_NAME)
VALUES
	(
		(
			SELECT
				ID_TRIBE
			FROM
				TRIBE
			WHERE
				SCIENTIFIC_NAME = 'Amblyoponini'
		),
		'Prionopelta'
	)
RETURNING
	ID_GENUS;

INSERT INTO
	GENUS (ID_TRIBE, SCIENTIFIC_NAME)
VALUES
	(
		(
			SELECT
				ID_TRIBE
			FROM
				TRIBE
			WHERE
				SCIENTIFIC_NAME = 'Ponerini'
		),
		'Hypoponera'
	)
RETURNING
	ID_GENUS;

INSERT INTO
	GENUS (ID_TRIBE, SCIENTIFIC_NAME)
VALUES
	(
		(
			SELECT
				ID_TRIBE
			FROM
				TRIBE
			WHERE
				SCIENTIFIC_NAME = 'Attini'
		),
		'Octostruma'
	)
RETURNING
	ID_GENUS;

INSERT INTO
	GENUS (ID_TRIBE, SCIENTIFIC_NAME)
VALUES
	(
		(
			SELECT
				ID_TRIBE
			FROM
				TRIBE
			WHERE
				SCIENTIFIC_NAME = 'Solenopsidini'
		),
		'Solenopsis'
	)
RETURNING
	ID_GENUS;

-- Create the Species table
CREATE TABLE SPECIES (
	ID_SPECIES SERIAL,
	ID_GENUS INTEGER NOT NULL,
	SCIENTIFIC_NAME TEXT NOT NULL,
	CONSTRAINT FK_SPECIES_GENUS FOREIGN KEY (ID_GENUS) REFERENCES GENUS (ID_GENUS),
	CONSTRAINT PK_SPECIES PRIMARY KEY (ID_SPECIES)
);

-- Insert the species
INSERT INTO
	SPECIES (ID_GENUS, SCIENTIFIC_NAME)
VALUES
	(
		(
			SELECT
				ID_GENUS
			FROM
				GENUS
			WHERE
				SCIENTIFIC_NAME = 'Prionopelta'
		),
		'Prionopelta punctulata'
	);

INSERT INTO
	SPECIES (ID_GENUS, SCIENTIFIC_NAME)
VALUES
	(
		(
			SELECT
				ID_GENUS
			FROM
				GENUS
			WHERE
				SCIENTIFIC_NAME = 'Hypoponera'
		),
		'Hypoponera sp1'
	);

INSERT INTO
	SPECIES (ID_GENUS, SCIENTIFIC_NAME)
VALUES
	(
		(
			SELECT
				ID_GENUS
			FROM
				GENUS
			WHERE
				SCIENTIFIC_NAME = 'Octostruma'
		),
		'Octostruma balzani'
	);

INSERT INTO
	SPECIES (ID_GENUS, SCIENTIFIC_NAME)
VALUES
	(
		(
			SELECT
				ID_GENUS
			FROM
				GENUS
			WHERE
				SCIENTIFIC_NAME = 'Solenopsis'
		),
		'Solenopsis sp1'
	);

CREATE TABLE COUNTRY (
	ID_COUNTRY SERIAL,
	COUNTRY_NAME TEXT NOT NULL,
	CONSTRAINT PK_COUNTRY PRIMARY KEY (ID_COUNTRY)
);

INSERT INTO
	COUNTRY (COUNTRY_NAME)
VALUES
	('Argentina')
RETURNING
	ID_COUNTRY;

CREATE TABLE PROVINCE (
	ID_PROVINCE SERIAL,
	ID_COUNTRY INTEGER NOT NULL,
	PROVINCE_NAME TEXT NOT NULL,
	CONSTRAINT FK_PROVINCE_COUNTRY FOREIGN KEY (ID_COUNTRY) REFERENCES COUNTRY (ID_COUNTRY),
	CONSTRAINT PK_PROVINCE PRIMARY KEY (ID_PROVINCE)
);

INSERT INTO
	PROVINCE (ID_COUNTRY, PROVINCE_NAME)
VALUES
	(1, 'Chaco')
RETURNING
	ID_PROVINCE;

INSERT INTO
	PROVINCE (ID_COUNTRY, PROVINCE_NAME)
VALUES
	(1, 'Corrientes')
RETURNING
	ID_PROVINCE;

CREATE TABLE DEPARTMENT (
	ID_DEPARTMENT SERIAL,
	ID_PROVINCE INTEGER NOT NULL,
	DEPARTMENT_NAME TEXT NOT NULL,
	CONSTRAINT FK_DEPARTMENT_PROVINCE FOREIGN KEY (ID_PROVINCE) REFERENCES PROVINCE (ID_PROVINCE),
	CONSTRAINT PK_DEPARTMENT PRIMARY KEY (ID_DEPARTMENT)
);

INSERT INTO
	DEPARTMENT (ID_PROVINCE, DEPARTMENT_NAME)
VALUES
	(2, 'Sargento Cabral')
RETURNING
	ID_DEPARTMENT;

CREATE TABLE LOCALITY (
	ID_LOCALITY SERIAL,
	ID_DEPARTMENT INTEGER NOT NULL,
	LOCALITY_NAME TEXT NOT NULL,
	CONSTRAINT FK_LOCALITY_DEPARTMENT FOREIGN KEY (ID_DEPARTMENT) REFERENCES DEPARTMENT (ID_DEPARTMENT),
	CONSTRAINT PK_LOCALITY PRIMARY KEY (ID_LOCALITY)
);

INSERT INTO
	LOCALITY (ID_DEPARTMENT, LOCALITY_NAME)
VALUES
	(1, 'Estancia El Bagual')
RETURNING
	ID_LOCALITY;


CREATE TABLE PRESERVATION_METHOD (
	ID_PRESERVATION_METHOD SERIAL,
	METHOD_NAME TEXT NOT NULL,
	CONSTRAINT PK_PRESERVATION_METHOD PRIMARY KEY (ID_PRESERVATION_METHOD)
);

-- Insert preservation methos
INSERT INTO
	PRESERVATION_METHOD (METHOD_NAME)
VALUES
	('Alcohol')
RETURNING
	ID_PRESERVATION_METHOD;

CREATE TABLE ENVIRONMENT (
	ID_ENVIRONMENT SERIAL,
	ENVIRONMENT_NAME TEXT NOT NULL,
	CONSTRAINT PK_ENVIRONMENT PRIMARY KEY (ID_ENVIRONMENT)
);

--insert into environment 
INSERT INTO
	ENVIRONMENT (ENVIRONMENT_NAME)
VALUES
	('Selva riparia')
RETURNING
	ID_ENVIRONMENT;

CREATE TABLE TRAP (
	ID_TRAP SERIAL,
	TRAP_NAME TEXT NOT NULL,
	CONSTRAINT PK_TRAP PRIMARY KEY (ID_TRAP)
);

--insert into trap
INSERT INTO
	PUBLIC.TRAP (TRAP_NAME)
VALUES
	('Winkler')
RETURNING
	ID_TRAP;

CREATE TABLE PERSON (
	ID_PERSON SERIAL,
	PERSON_NAME TEXT NOT NULL,
	PERSON_LASTNAME TEXT NOT NULL,
	CONSTRAINT PK_PERSON PRIMARY KEY (ID_PERSON)
);

INSERT INTO
	PUBLIC.PERSON (PERSON_NAME, PERSON_LASTNAME)
VALUES
	('DD', 'Larrea')
RETURNING
	ID_PERSON;

CREATE TABLE IDENTIFIER (
	ID_IDENTIFIER SERIAL,
	ID_PERSON INTEGER NOT NULL,
	DESCRIPTION TEXT NULL,
	CONSTRAINT FK_IDENTIFIER_PERSON FOREIGN KEY (ID_PERSON) REFERENCES PERSON (ID_PERSON),
	CONSTRAINT PK_IDENTIFIER PRIMARY KEY (ID_IDENTIFIER)
);

INSERT INTO
	PUBLIC.IDENTIFIER (ID_PERSON, DESCRIPTION)
VALUES
	(1, 'dario el jefecito')
RETURNING
	ID_IDENTIFIER;

CREATE TABLE COLLECTOR (
	ID_COLLECTOR SERIAL,
	ID_PERSON INTEGER NOT NULL,
	DESCRIPTION TEXT NULL,
	CONSTRAINT FK_COLLECTOR_PERSON FOREIGN KEY (ID_PERSON) REFERENCES PERSON (ID_PERSON),
	CONSTRAINT PK_COLLECTOR PRIMARY KEY (ID_COLLECTOR)
);

INSERT INTO
	PUBLIC.COLLECTOR (ID_PERSON, DESCRIPTION)
VALUES
	(1, 'colector profesional')
RETURNING
	ID_COLLECTOR;

CREATE TABLE COLLECTION (
    ID_COLLECTION SERIAL,
    ID_COLLECTOR INTEGER NOT NULL,
    ID_PRESERVATION_METHOD INTEGER NOT NULL,
    ID_TRAP INTEGER NOT NULL,
    COLLECTION_DATE DATE NOT NULL,
    CONSTRAINT PK_COLLECTION PRIMARY KEY (ID_COLLECTION),
    CONSTRAINT FK_COLLECTION_COLLECTOR FOREIGN KEY (ID_COLLECTOR) REFERENCES COLLECTOR (ID_COLLECTOR),
    CONSTRAINT FK_COLLECTION_PRESERVATION_METHOD FOREIGN KEY (ID_PRESERVATION_METHOD) REFERENCES PRESERVATION_METHOD (ID_PRESERVATION_METHOD),
    CONSTRAINT FK_COLLECTION_TRAP FOREIGN KEY (ID_TRAP) REFERENCES TRAP (ID_TRAP),
    CONSTRAINT CHK_COLLECTION_DATE CHECK (COLLECTION_DATE <= CURRENT_DATE)
);


CREATE TABLE OBSERVATION (
	ID_OBSERVATION SERIAL,
	ID_SPECIES INTEGER NOT NULL,
	ID_LOCALITY INTEGER NOT NULL,
	ID_COLLECTION INTEGER NOT NULL,
	CONSTRAINT FK_OBSERVATION_SPECIES FOREIGN KEY (ID_SPECIES) REFERENCES SPECIES (ID_SPECIES),
	CONSTRAINT FK_OBSERVATION_LOCALITY FOREIGN KEY (ID_LOCALITY) REFERENCES LOCALITY (ID_LOCALITY),
	CONSTRAINT FK_OBSERVATION_COLLECTION FOREIGN KEY (ID_COLLECTION) REFERENCES COLLECTION (ID_COLLECTION),
	CONSTRAINT PK_OBSERVATION PRIMARY KEY (ID_OBSERVATION)
);


select * from public.observation;
-- FUNCTION: public.get_observations(integer, integer)

-- DROP FUNCTION IF EXISTS public.get_observations(integer, integer);
CREATE OR REPLACE FUNCTION public.get_observations(
	_limit integer,
	_offset integer)
    RETURNS TABLE(id_observation integer, collection_date date, person_name text, person_lastname text, trap_name text, method_name text, species text, genus text, tribe text, subfamily text, family text, province text, department text, locality text) 
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE PARALLEL SAFE 
    ROWS 2007

AS $BODY$
BEGIN
    IF _limit = -1 THEN
        RETURN QUERY
		SELECT
			O.ID_OBSERVATION,
			CLTN.COLLECTION_DATE AS COLLECTION_DATE,
			PRSN.PERSON_NAME AS PERSON_NAME,
			PRSN.PERSON_LASTNAME AS PERSON_LASTNAME,
			TRP.TRAP_NAME AS TRAP_NAME,
			PM.METHOD_NAME AS METHOD_NAME,
			E.SCIENTIFIC_NAME AS SPECIES,
			G.SCIENTIFIC_NAME AS GENUS,
			T.SCIENTIFIC_NAME AS TRIBE,
			S.SCIENTIFIC_NAME AS SUBFAMILY,
			F.SCIENTIFIC_NAME AS FAMILY,
			P.PROVINCE_NAME AS PROVINCE,
			D.DEPARTMENT_NAME AS DEPARTMENT,
			L.LOCALITY_NAME AS LOCALITY
		FROM
			OBSERVATION O
			-- JOINs de collection
			JOIN COLLECTION CLTN ON O.ID_COLLECTION = CLTN.ID_COLLECTION
			JOIN PERSON PRSN ON PRSN.ID_PERSON = CLTN.ID_PERSON
			JOIN TRAP TRP ON TRP.ID_TRAP = CLTN.ID_TRAP
			JOIN PRESERVATION_METHOD PM ON  PM.ID_PRESERVATION_METHOD = CLTN.ID_PRESERVATION_METHOD
			-- JOINs de descripcion animal
			JOIN SPECIES E ON O.ID_SPECIES = E.ID_SPECIES
			JOIN GENUS G ON E.ID_GENUS = G.ID_GENUS
			JOIN TRIBE T ON G.ID_TRIBE = T.ID_TRIBE
			JOIN SUBFAMILY S ON T.ID_SUBFAMILY = S.ID_SUBFAMILY
			JOIN FAMILY F ON S.ID_FAMILY = F.ID_FAMILY
			-- JOINs de location
			JOIN LOCALITY L ON O.ID_LOCALITY = L.ID_LOCALITY
			JOIN DEPARTMENT D ON L.ID_DEPARTMENT = D.ID_DEPARTMENT
			JOIN PROVINCE P ON D.ID_PROVINCE = P.ID_PROVINCE
        ORDER BY O.ID_OBSERVATION;
    ELSE
        RETURN QUERY
        SELECT
			O.ID_OBSERVATION,
			CLTN.COLLECTION_DATE AS COLLECTION_DATE,
			PRSN.PERSON_NAME AS PERSON_NAME,
			PRSN.PERSON_LASTNAME AS PERSON_LASTNAME,
			TRP.TRAP_NAME AS TRAP_NAME,
			PM.METHOD_NAME AS METHOD_NAME,
			E.SCIENTIFIC_NAME AS SPECIES,
			G.SCIENTIFIC_NAME AS GENUS,
			T.SCIENTIFIC_NAME AS TRIBE,
			S.SCIENTIFIC_NAME AS SUBFAMILY,
			F.SCIENTIFIC_NAME AS FAMILY,
			P.PROVINCE_NAME AS PROVINCE,
			D.DEPARTMENT_NAME AS DEPARTMENT,
			L.LOCALITY_NAME AS LOCALITY
		FROM
			OBSERVATION O
			-- JOINs de collection
			JOIN COLLECTION CLTN ON O.ID_COLLECTION = CLTN.ID_COLLECTION
			JOIN PERSON PRSN ON PRSN.ID_PERSON = CLTN.ID_PERSON
			JOIN TRAP TRP ON TRP.ID_TRAP = CLTN.ID_TRAP
			JOIN PRESERVATION_METHOD PM ON  PM.ID_PRESERVATION_METHOD = CLTN.ID_PRESERVATION_METHOD
			-- JOINs de descripcion animal
			JOIN SPECIES E ON O.ID_SPECIES = E.ID_SPECIES
			JOIN GENUS G ON E.ID_GENUS = G.ID_GENUS
			JOIN TRIBE T ON G.ID_TRIBE = T.ID_TRIBE
			JOIN SUBFAMILY S ON T.ID_SUBFAMILY = S.ID_SUBFAMILY
			JOIN FAMILY F ON S.ID_FAMILY = F.ID_FAMILY
			-- JOINs de location
			JOIN LOCALITY L ON O.ID_LOCALITY = L.ID_LOCALITY
			JOIN DEPARTMENT D ON L.ID_DEPARTMENT = D.ID_DEPARTMENT
			JOIN PROVINCE P ON D.ID_PROVINCE = P.ID_PROVINCE
        ORDER BY O.ID_OBSERVATION
        LIMIT _limit
        OFFSET _offset;
    END IF;
END;
$BODY$;

ALTER FUNCTION public.get_observations(integer, integer)
    OWNER TO postgres;

-- Prueba sin límite y sin offset
SELECT * FROM public.get_observations(-1, 0);

-- Prueba con límite y offset
SELECT * FROM public.get_observations(10, 0); -- Devuelve las primeras 10 observaciones
SELECT * FROM public.get_observations(10, 10); -- Devuelve las siguientes 10 observaciones




-- FUNCTION: public.get_locations()

-- DROP FUNCTION IF EXISTS public.get_locations();

CREATE OR REPLACE FUNCTION public.get_locations(
	)
    RETURNS TABLE(locality text, department text, province text, country text) 
    LANGUAGE 'plpgsql'
    COST 50
    VOLATILE PARALLEL SAFE
    ROWS 100

AS $BODY$
BEGIN
	RETURN QUERY
	SELECT
		L.LOCALITY_NAME AS LOCALITY,
		D.DEPARTMENT_NAME AS DEPARTMENT,
		P.PROVINCE_NAME AS PROVINCE,
		C.COUNTRY_NAME AS COUNTRY
	FROM
		LOCALITY L
		RIGHT JOIN DEPARTMENT D ON D.ID_DEPARTMENT = L.ID_DEPARTMENT
		RIGHT JOIN PROVINCE P ON P.ID_PROVINCE = D.ID_PROVINCE
		RIGHT JOIN COUNTRY C ON C.ID_COUNTRY = P.ID_COUNTRY;
END;
$BODY$;

ALTER FUNCTION public.get_locations()
    OWNER TO postgres;

	

-- FUNCTION: public.add_collection_and_observation(integer, integer, integer, date, integer, integer)

-- DROP FUNCTION IF EXISTS public.add_collection_and_observation(integer, integer, integer, date, integer, integer);

CREATE OR REPLACE FUNCTION public.add_collection_and_observation(
	_id_person integer,
	_id_preservation_method integer,
	_id_trap integer,
	_collection_date date,
	_id_species integer,
	_id_locality integer)
    RETURNS integer
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE PARALLEL UNSAFE
AS $BODY$
DECLARE
    _id_collection INTEGER;
    _id_observation INTEGER;
BEGIN
    BEGIN
        -- Insert into COLLECTION
        INSERT INTO COLLECTION (ID_PERSON, ID_PRESERVATION_METHOD, ID_TRAP, COLLECTION_DATE)
        VALUES (_id_person, _id_preservation_method, _id_trap, _collection_date)
        RETURNING ID_COLLECTION INTO _id_collection;
    EXCEPTION WHEN OTHERS THEN
        -- Handle the error
        RAISE EXCEPTION 'Failed to insert into COLLECTION: %', SQLERRM;
    END;

    BEGIN
        -- Insert into OBSERVATION
        INSERT INTO OBSERVATION (ID_SPECIES, ID_LOCALITY, ID_COLLECTION)
        VALUES (_id_species, _id_locality, _id_collection)
        RETURNING ID_OBSERVATION INTO _id_observation;
    EXCEPTION WHEN OTHERS THEN
        -- Handle the error and rollback the previous insert
        RAISE EXCEPTION 'Failed to insert into OBSERVATION: %', SQLERRM;
    END;

    -- Return the ID of the new observation
    RETURN _id_observation;
END;
$BODY$;

ALTER FUNCTION public.add_collection_and_observation(integer, integer, integer, date, integer, integer)
    OWNER TO postgres;



	-- Llamada de prueba a la función add_collection_and_observation
DO $$
DECLARE
    result INTEGER;
BEGIN
    result := add_collection_and_observation(
        _id_person => 1,
        _id_preservation_method => 1,
        _id_trap => 1,
        _collection_date => '2023-07-11',  -- Usar una fecha válida en formato ISO
        _id_species => 1,
        _id_locality => 1
    );
    RAISE NOTICE 'New observation ID: %', result;
END;
$$;


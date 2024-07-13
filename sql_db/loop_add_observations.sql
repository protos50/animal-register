DO $$
DECLARE
    species_id INT;
BEGIN
    FOR i IN 1..2000 LOOP
        SELECT id_species
        INTO species_id
        FROM unnest(array[1,2,3,4,8]) AS t(id_species)
        ORDER BY random() 
        LIMIT 1;

        INSERT INTO observation (id_species, id_locality) 
        VALUES (species_id, 1);
    END LOOP;
END $$;

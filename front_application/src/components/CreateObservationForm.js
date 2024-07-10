import React, { useState, useEffect } from 'react';
import axios from 'axios';

const CreateObservationForm = () => {
    const [species, setSpecies] = useState([]);
    const [localities, setLocalities] = useState([]);
    const [selectedSpecies, setSelectedSpecies] = useState('');
    const [selectedLocality, setSelectedLocality] = useState('');

    useEffect(() => {
        // Obtener todas las especies del servidor
        axios.get('/api/simple_species')
            .then(response => {
                setSpecies(response.data);
            })
            .catch(error => {
                console.error("Error fetching species:", error);
            });

        axios.get('/api/simple_localities')
            .then(response => {
                setLocalities(response.data);
            })
            .catch(error => {
                console.error("Error fetching localities:", error);
            });

    }, []);


    const handleSubmit = (e) => {
        e.preventDefault();

        if (!selectedSpecies) {
            alert("Please select a species");
            return;
        }
        if (!selectedLocality) {
            alert("Please select a locality");
            return;
        }

        const newObservation = {
            id_species: parseInt(selectedSpecies, 10), // Convertir a entero base 10 el valor selectedSpecies
            id_locality: parseInt(selectedLocality, 10)
        };

        // Realizar la solicitud POST para crear la nueva observación
        axios.post('/api/observations', newObservation)
            .then(response => {
                alert("Observation created successfully!");
            })
            .catch(error => {
                console.error("Error creating observation:", error);
            });
    };

    return (
        <form onSubmit={handleSubmit}>
            <label>
                Species:
                <select value={selectedSpecies} onChange={(e) => setSelectedSpecies(e.target.value)}>
                    <option value="">Select a species</option>
                    {species.map(species => (
                        <option key={species.id_species} value={species.id_species}>
                            {species.scientific_name}
                        </option>
                    ))}
                </select>
            </label>
            <label>
                Locality:
                <select value={selectedLocality} onChange={(e) => setSelectedLocality(e.target.value)}>
                    <option value="">Select a locality</option>
                    {localities.map(locality => (
                        <option key={locality.id_locality} value={locality.id_locality}>
                            {locality.locality_name}
                        </option>
                    ))}
                </select>
            </label>
            <button type="submit">Create Observation</button>
        </form>
    );
};

export default CreateObservationForm;

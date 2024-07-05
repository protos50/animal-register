import React, { useState, useEffect } from 'react';
import axios from 'axios';

const CreateObservationForm = () => {
    const [species, setSpecies] = useState([]);
    const [selectedSpecies, setSelectedSpecies] = useState('');

    useEffect(() => {
        // Obtener todas las especies del servidor
        axios.get('/api/simple_species')
            .then(response => {
                setSpecies(response.data);
            })
            .catch(error => {
                console.error("Error fetching species:", error);
            });
    }, []);
    

    const handleSubmit = (e) => {
        e.preventDefault();

        if (!selectedSpecies) {
            alert("Please select a species");
            return;
        }

        const newObservation = {
            id_species: parseInt(selectedSpecies, 10), // Convertir a entero base 10 el valor selectedSpecies
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
            <button type="submit">Create Observation</button>
        </form>
    );
};

export default CreateObservationForm;

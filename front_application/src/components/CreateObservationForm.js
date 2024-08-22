import React, { useState, useEffect } from 'react';
import axios from 'axios';

const CreateObservationForm = () => {
    const [species, setSpecies] = useState([]);
    const [localities, setLocalities] = useState([]);
    const [preservation_methods, setPreservationMethods] = useState([]);
    const [traps, setTraps] = useState([]);
    const [people, setPeople] = useState([]);
    const [selectedSpecies, setSelectedSpecies] = useState('');
    const [selectedLocality, setSelectedLocality] = useState('');
    const [selectedPreservationMethod, setSelectedPreservationMethod] = useState('');
    const [selectedTrap, setSelectedTrap] = useState('');
    const [selectedPerson, setSelectedPerson] = useState('');
    const [collectionDate, setCollectionDate] = useState('');

    useEffect(() => {
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

        axios.get('/api/preservation_methods')
            .then(response => {
                setPreservationMethods(response.data);
            })
            .catch(error => {
                console.error("Error fetching preservation methods:", error);
            });

        axios.get('/api/traps')
            .then(response => {
                setTraps(response.data);
            })
            .catch(error => {
                console.error("Error fetching traps:", error);
            });

        axios.get('/api/people')
            .then(response => {
                setPeople(response.data);
            })
            .catch(error => {
                console.error("Error fetching people:", error);
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

        if (!selectedPreservationMethod) {
            alert("Please select a preservation method");
            return;
        }

        if (!selectedTrap) {
            alert("Please select a trap");
            return;
        }

        if (!collectionDate) {
            alert("Please select a collection date");
            return;
        }

        if (!selectedPerson) {
            alert("Please select a person");
            return;
        }

        const newObservation = {
            id_person: parseInt(selectedPerson, 10),
            id_preservation_method: parseInt(selectedPreservationMethod, 10),
            id_trap: parseInt(selectedTrap, 10),
            collection_date: collectionDate,
            id_species: parseInt(selectedSpecies, 10), // Convertir a entero base 10 el valor selectedSpecies
            id_locality: parseInt(selectedLocality, 10),
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

    // Get today's date in YYYY-MM-DD format for the max attribute
    const today = new Date().toISOString().split('T')[0];

    return (
        <form onSubmit={handleSubmit}>
            <label>
                Person:
                <select value={selectedPerson} onChange={(e) => setSelectedPerson(e.target.value)}>
                    <option value="">Select a person</option>
                    {people.map(person => (
                        <option key={person.id_person} value={person.id_person}>
                            {person.person_name+" "+person.person_lastname}
                        </option>
                    ))}
                </select>
            </label>
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
            <label>
                Preservation_Methods:
                <select value={selectedPreservationMethod} onChange={(e) => setSelectedPreservationMethod(e.target.value)}>
                    <option value="">Select a preservation method</option>
                    {preservation_methods.map(preservation_method => (
                        <option key={preservation_method.id_preservation_method} value={preservation_method.id_preservation_method}>
                            {preservation_method.method_name}
                        </option>
                    ))}
                </select>
            </label>
            <label>
                Traps:
                <select value={selectedTrap} onChange={(e) => setSelectedTrap(e.target.value)}>
                    <option value="">Select a trap</option>
                    {traps.map(trap => (
                        <option key={trap.id_trap} value={trap.id_trap}>
                            {trap.trap_name}
                        </option>
                    ))}
                </select>
            </label>
            <label>
                Collection Date:
                <input
                    type="date"
                    value={collectionDate}
                    onChange={(e) => setCollectionDate(e.target.value)}
                    max={today}
                    required
                />
            </label>
            <button type="submit">Create Observation</button>
        </form>
    );
};

export default CreateObservationForm;

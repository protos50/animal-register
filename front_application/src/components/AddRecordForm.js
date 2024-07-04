import React, { useState, useEffect } from 'react';
import axios from 'axios';

const AddRecordForm = ({ table, relatedTable, onSubmit }) => {
    const [scientificName, setScientificName] = useState('');
    const [relatedData, setRelatedData] = useState([]);
    const [selectedRelatedId, setSelectedRelatedId] = useState('');

    // Mapeo de tablas para nombres relacionados correctos
    const tableMap = {
        families: 'family',
        subfamilies: 'subfamily',
        tribes: 'tribe',
        genera: 'genus',
        species: 'species'
    };


    useEffect(() => {
        if (relatedTable) {
            console.log(`Fetching data for related table: ${relatedTable}`);
            axios.get(`/api/${relatedTable}`)
                .then(response => {
                    console.log(`Received data for ${relatedTable}:`, response.data);
                    setRelatedData(response.data);
                })
                .catch(error => console.error(`Error fetching ${relatedTable}:`, error));
        }
    }, [relatedTable]);

    const handleSubmit = (event) => {
        event.preventDefault();

        const relatedField = `id_${tableMap[relatedTable]}`;
        const newRecord = {
            [relatedField]: parseInt(selectedRelatedId, 10),
            scientific_name: scientificName
        };

        console.log('Submitting new record:', newRecord);
        onSubmit(newRecord);
    };

    return (
        <form onSubmit={handleSubmit}>
            <label>
                Scientific Name:
                <input
                    type="text"
                    value={scientificName}
                    onChange={(e) => setScientificName(e.target.value)}
                    required
                />
            </label>
            {relatedTable && (
                <label>
                    {tableMap[relatedTable].charAt(0).toUpperCase() + tableMap[relatedTable].slice(1)}:
                    <select
                        value={selectedRelatedId}
                        onChange={(e) => setSelectedRelatedId(e.target.value)}
                        required
                    >
                        <option value="">Select a {tableMap[relatedTable]}</option>
                        {relatedData.map((item) => (
                            <option
                                key={item[`id_${tableMap[relatedTable]}`]}
                                value={item[`id_${tableMap[relatedTable]}`]}
                            >
                                {item[tableMap[relatedTable]]}
                            </option>
                        ))}
                    </select>
                </label>
            )}
            <button type="submit">Add {table.slice(0, -1)}</button>
        </form>
    );
};

export default AddRecordForm;

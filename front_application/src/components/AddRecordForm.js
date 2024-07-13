import React, { useState, useEffect } from 'react';
import axios from 'axios';

const AddRecordForm = ({ table, relatedTable, onSubmit }) => {
    const [recordName, setRecordName] = useState('');
    const [relatedData, setRelatedData] = useState([]);
    const [selectedRelatedId, setSelectedRelatedId] = useState('');

    // Mapeo de tablas para nombres relacionados correctos
    const tableMap = {
        families: { label: 'Family', nameField: 'scientific_name' },
        subfamilies: { label: 'Subfamily', nameField: 'scientific_name' },
        tribes: { label: 'Tribe', nameField: 'scientific_name' },
        genera: { label: 'Genus', nameField: 'scientific_name' },
        species: { label: 'Species', nameField: 'scientific_name' },
        localities: { label: 'Locality', nameField: 'locality_name' },
        departments: { label: 'Department', nameField: 'department_name' },
        provinces: { label: 'Province', nameField: 'province_name' },
        countries: { label: 'Country', nameField: 'country_name' }
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

        // Creamos un nuevo registro básico con el nombre
        const newRecord = {
            [tableMap[table].nameField]: recordName
        };

        // Si existe una tabla relacionada, agregamos el campo relacionado
        if (relatedTable) {
            const relatedField = `id_${tableMap[relatedTable].label.toLowerCase()}`;
            newRecord[relatedField] = parseInt(selectedRelatedId, 10);
        }

        console.log('Submitting new record:', newRecord);
        onSubmit(newRecord);
    };

    return (
        <form onSubmit={handleSubmit}>
            <label>
                {tableMap[table].label} Name:
                <input
                    type="text"
                    value={recordName}
                    onChange={(e) => setRecordName(e.target.value)}
                    required
                />
            </label>
            {relatedTable && (
                <label>
                    {tableMap[relatedTable].label}:
                    <select
                        value={selectedRelatedId}
                        onChange={(e) => setSelectedRelatedId(e.target.value)}
                        required
                    >
                        <option value="">Select a {tableMap[relatedTable].label}</option>
                        {relatedData.map((item) => (
                            <option
                                key={item[`id_${tableMap[relatedTable].label.toLowerCase()}`]}
                                value={item[`id_${tableMap[relatedTable].label.toLowerCase()}`]}
                            >
                                {item[tableMap[relatedTable].label.toLowerCase()]}
                            </option>
                        ))}
                    </select>
                </label>
            )}
            <button type="submit">Add {tableMap[table].label}</button>
        </form>
    );
};

export default AddRecordForm;

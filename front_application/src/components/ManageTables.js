import React, { useState } from 'react';
import AnimalTables from './AnimalTables';
import LocationTables from './LocationTables';
import PersonRoleTables from './PersonRoleTables';  
import CollectionTables from './CollectionTables';

const ManageTables = () => {
  const [activeTable, setActiveTable] = useState('animal'); // Estado para controlar la tabla activa

  const handleToggleTable = (table) => {
    setActiveTable(table);
  };

  const viewNames = {
    animal: 'Animal Records',
    location: 'Location Records',
    person: 'Person Records',
    collection: 'Collection Information Records',
  };
  
  return (
    <div>
      <div>
        <button onClick={() => handleToggleTable('animal')}>View and Add Animal Records</button>
        <button onClick={() => handleToggleTable('location')}>View and Add Location Records</button>
        <button onClick={() => handleToggleTable('person')}>View and Add Person Records</button>
        <button onClick={() => handleToggleTable('collection')}>View and Add Collection Information Records</button>
      </div>
  
      <h3>Selected View: {viewNames[activeTable]}</h3>
  
      {activeTable === 'animal' && <AnimalTables />}
      {activeTable === 'location' && <LocationTables />}
      {activeTable === 'person' && <PersonRoleTables />}
      {activeTable === 'collection' && <CollectionTables />}
    </div>
  );
  
  };

export default ManageTables;

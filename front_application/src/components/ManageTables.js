import React, { useState } from 'react';
import AnimalTables from './AnimalTables';
import LocationTables from './LocationTables';

const ManageTables = () => {
  const [activeTable, setActiveTable] = useState('animal'); // Estado para controlar la tabla activa

  const handleToggleTable = (table) => {
    setActiveTable(table);
  };

  return (
    <div>
      <div>
        <button onClick={() => handleToggleTable('animal')}>View and Add Animal Records</button>
        <button onClick={() => handleToggleTable('location')}>View and Add Location Records</button>
      </div>

      <h3>Selected View: {activeTable === 'animal' ? 'Animal Records' : 'Location Records'}</h3>

      {activeTable === 'animal' && <AnimalTables />}
      {activeTable === 'location' && <LocationTables />}
    </div>
  );
};

export default ManageTables;

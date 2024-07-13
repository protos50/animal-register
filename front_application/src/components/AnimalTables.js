import React from 'react';
import axios from 'axios';
import TableView from './TableView';
import AddRecordForm from './AddRecordForm';

const AnimalTables = () => {
    const [selectedTable, setSelectedTable] = React.useState('');
    const [relatedTable, setRelatedTable] = React.useState('');
  
    const handleTableSelection = (event) => {
      const table = event.target.value;
      setSelectedTable(table);
      switch (table) {
          case 'species':
              setRelatedTable('genera');
              break;
          case 'genera':
              setRelatedTable('tribes');
              break;
          case 'tribes':
              setRelatedTable('subfamilies');
              break;
          case 'subfamilies':
              setRelatedTable('families');
              break;
          case 'families':
              setRelatedTable(null);
              break;
          default:
              setRelatedTable(null);
      }
  };
  
    const handleAddRecord = (newRecord) => {
      axios.post(`/api/${selectedTable}`, newRecord)
        .then(response => {
          console.log(`Added new ${selectedTable.slice(0, -1)}:`, response.data);
          alert(`Successfully added new ${selectedTable.slice(0, -1)}!`);
        })
        .catch(error => {
          console.error(`Error adding new ${selectedTable.slice(0, -1)}:`, error);
          alert(`Failed to add new ${selectedTable.slice(0, -1)}.`);
        });
    };
  
    return (
      <div>
        <select onChange={handleTableSelection}>
          <option value="">Select a table</option>
          <option value="families">Families</option>
          <option value="subfamilies">Subfamilies</option>
          <option value="tribes">Tribes</option>
          <option value="genera">Genera</option>
          <option value="species">Species</option>
        </select>
  
        {selectedTable && (
          <div>
            <TableView table={selectedTable} />
            <AddRecordForm table={selectedTable} relatedTable={relatedTable} onSubmit={handleAddRecord} />
          </div>
        )}
      </div>
    );

  };

  export default AnimalTables;
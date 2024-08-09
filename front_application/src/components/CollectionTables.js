import React from 'react';
import axios from 'axios';
import TableView from './TableView';
import AddRecordForm from './AddRecordForm';

const CollectionTables = () => {
    const [selectedTable, setSelectedTable] = React.useState('');
    const [relatedTable, setRelatedTable] = React.useState('');
  
    const handleTableSelection = (event) => {
      const table = event.target.value;
      setSelectedTable(table);
      switch (table) {
        case 'localities':
          setRelatedTable('departments');
          break;
        case 'departments':
          setRelatedTable('provinces');
          break;
        case 'provinces':
          setRelatedTable('countries');
          break;
        case 'countries':
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
            <option value="traps">Traps</option>
            <option value="preservation_methods">Preservation Methods</option>
          </select>
    
          {selectedTable && (
            <div>
              <TableView table={selectedTable} />
              <AddRecordForm table={selectedTable} relatedTable={relatedTable} onSubmit={handleAddRecord} />
            </div>
          )}
        </div>
      );
}

export default CollectionTables;
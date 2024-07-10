import * as React from 'react';
import Box from '@mui/material/Box';
import { DataGrid } from '@mui/x-data-grid';
import axios from 'axios';

const columns = [
  { field: 'id_observation', headerName: 'ID', width: 90 },
  { field: 'species', headerName: 'Species', width: 150 },
  { field: 'genus', headerName: 'Genus', width: 150 },
  { field: 'tribe', headerName: 'Tribe', width: 150 },
  { field: 'subfamily', headerName: 'Subfamily', width: 150 },
  { field: 'family', headerName: 'Family', width: 150 },
  { field: 'locality', headerName: 'Locality', width: 150 },
  { field: 'department', headerName: 'Department', width: 150 },
  { field: 'province', headerName: 'Province', width: 150 },
];


const Observations = () => {
  const [observations, setObservations] = React.useState([]);

  React.useEffect(() => {
    axios.get('/api/observations')
      .then(response => {
        console.log('Received observations:', response.data);
        setObservations(response.data.map((observation, index) => ({
          ...observation,
          id: index + 1, // Generate a unique id for each row
        })));
      })
      .catch(error => console.error('Error fetching observations:', error));
  }, []);

  return (  
    <Box sx={{ height: 400, width: '100%' }}>
      <DataGrid
        rows={observations}
        columns={columns}
        getRowId={(row) => row.id_observation} // Specify id_observation as the unique id
        pageSize={5}
        rowsPerPageOptions={[5]}
        checkboxSelection
        disableSelectionOnClick
        />
    </Box>
  );
};

export default Observations;

import React, { useState, useEffect } from 'react';
import axios from 'axios';
import { DataGrid } from '@mui/x-data-grid';
import { TablePagination } from '@mui/material';




const columns = [
  { field: 'id_observation', headerName: 'ID', width: 90 },
  { field: 'species', headerName: 'Species', width: 150 },
  { field: 'genus', headerName: 'Genus', width: 150 },
  { field: 'tribe', headerName: 'Tribe', width: 150 },
  { field: 'subfamily', headerName: 'Subfamily', width: 150 },
  { field: 'family', headerName: 'Family', width: 150 },
  { field: 'province', headerName: 'Province', width: 150 },
  { field: 'department', headerName: 'Department', width: 150 },
  { field: 'locality', headerName: 'Locality', width: 150 },
];

const Observations = () => {
  const [page, setPage] = useState(0);
  const [rowsPerPage, setRowsPerPage] = useState(100);
  const [totalRows, setTotalRows] = useState(2007);
  const [observations, setObservations] = useState([]);

  const fetchObservations = async () => {
    const offset = page * rowsPerPage;
    const limit = rowsPerPage;

    try {
      const response = await axios.get(`/api/observations?limit=${limit}&offset=${offset}`);
      setObservations(response.data);
      // Establecer el total de filas para la paginación
      //setTotalRows(parseInt(response.headers['x-total-count'], 10)); // Obtén esto del encabezado de la respuesta
    } catch (error) {
      console.error('Error fetching observations:', error);
    }
  };

  useEffect(() => {
    fetchObservations();
  }, [page, rowsPerPage]);

  const handleChangePage = (event, newPage) => {
    setPage(newPage);
  };

  const handleChangeRowsPerPage = (event) => {
    setRowsPerPage(parseInt(event.target.value, 10));
    setPage(0);
  };

  return (
    <div style={{ height: 500, width: '100%' }}>

        <h1>Observations</h1>
        <DataGrid
          rows={observations}
          getRowId={(row) => row.id_observation}
          columns={columns} // Define tus columnas aquí
          rowSelection={true}
          checkboxSelection={true}
          loading={observations.length === 0}
        />

        <TablePagination
          component="div"
          count={totalRows}
          page={page}
          onPageChange={handleChangePage}
          rowsPerPage={[rowsPerPage]}
          rowsPerPageOptions={[25, 50, 100, 500, 1000, 2000]}
          onRowsPerPageChange={handleChangeRowsPerPage}
        />

    </div>
  );
};

export default Observations;

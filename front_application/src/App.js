import React from 'react';
import { BrowserRouter as Router, Route, Routes, Link } from 'react-router-dom';
import { CssBaseline, Drawer, List, ListItem, ListItemText, Box, createTheme, ThemeProvider, ListItemButton } from '@mui/material';
import { styled } from '@mui/system';
import Observations from './components/Observations';
import CreateObservationForm from './components/CreateObservationForm';
import ManageTables from './components/ManageTables';
import DownloadButton from './components/DownloadButton';
import Welcome from './components/Welcome';

const theme = createTheme();

const Root = styled('div')(({ theme }) => ({
  display: 'flex',
}));

const StyledDrawer = styled(Drawer)(({ theme }) => ({
  width: 240,
  flexShrink: 0,
  '& .MuiDrawer-paper': {
    width: 240,
  },
}));

const Content = styled('main')(({ theme }) => ({
  flexGrow: 1,
  padding: theme.spacing(3),
}));

const App = () => {
  return (
    <ThemeProvider theme={theme}>
      <Router>
        <Root>
          <CssBaseline />
          <StyledDrawer variant="permanent">
            <Box sx={theme.mixins.toolbar} />
            <List>
            <ListItemButton button component={Link} to="/">
                <ListItemText primary="Home" />
              </ListItemButton>
              <ListItemButton button component={Link} to="/observations">
                <ListItemText primary="Observations" />
              </ListItemButton>
              <ListItemButton button component={Link} to="/download-observations-csv">
                <ListItemText primary="Export Observations as CSV" />
              </ListItemButton>
              <ListItemButton button component={Link} to="/create-observation">
                <ListItemText primary="Create Observation" />
              </ListItemButton>
              <ListItemButton button component={Link} to="/manage-tables">
                <ListItemText primary="Manage Tables" />
              </ListItemButton>
            </List>
          </StyledDrawer>
          <Content>
            <Box sx={theme.mixins.toolbar} />
            <Routes>
              <Route path="/" element={<Welcome />} />
              <Route path="/observations" element={<Observations />} />
              <Route path="/download-observations-csv" element={<DownloadButton />} /> 
              <Route path="/create-observation" element={<CreateObservationForm />} />
              <Route path="/manage-tables" element={<ManageTables />} />
            </Routes>
          </Content>
        </Root>
      </Router>
    </ThemeProvider>
  );
};

export default App;

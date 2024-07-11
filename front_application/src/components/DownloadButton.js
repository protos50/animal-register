import React from 'react';
import axios from 'axios';

const DownloadButton = () => {
    const handleDownload = async () => {
        try {
            const response = await axios.get('/api/download_observations_csv', {
                responseType: 'blob',
            });
            const url = window.URL.createObjectURL(new Blob([response.data]));
            const link = document.createElement('a');
            link.href = url;
            link.setAttribute('download', 'observations.csv');
            document.body.appendChild(link);
            link.click();
            link.parentNode.removeChild(link);
        } catch (error) {
            console.error('Error downloading the file', error);
        }
    };

    return (
        <div>

        <h2>Download Observations</h2>
        
        <button onClick={handleDownload}>
            Download Observations as CSV
        </button>

        </div>
    );
};

export default DownloadButton;

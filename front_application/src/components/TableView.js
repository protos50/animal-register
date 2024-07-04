import React, { useState, useEffect } from 'react';
import axios from 'axios';

const TableView = ({ table }) => {
    const [data, setData] = useState([]);

    useEffect(() => {
        axios.get(`/api/${table}`)
            .then(response => setData(response.data))
            .catch(error => console.error(`Error fetching ${table}:`, error));
    }, [table]);

    const renderTableHeaders = () => {
        if (data.length === 0) return null;
        return (
            <thead>
                <tr>
                    {Object.keys(data[0]).map((key) => (
                        <th key={key}>{key}</th>
                    ))}
                </tr>
            </thead>
        );
    };

    const renderTableRows = () => {
        return (
            <tbody>
                {data.map((row, index) => (
                    <tr key={index}>
                        {Object.values(row).map((value, i) => (
                            <td key={i}>{value}</td>
                        ))}
                    </tr>
                ))}
            </tbody>
        );
    };

    return (
        <div>
            <h2>{table.charAt(0).toUpperCase() + table.slice(1)}</h2>
            <table>
                {renderTableHeaders()}
                {renderTableRows()}
            </table>
        </div>
    );
};

export default TableView;

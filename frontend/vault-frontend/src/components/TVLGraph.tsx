import React from "react";
import type { TVLResponse } from "../types";
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';

interface TVLGraphProps {
  data: TVLResponse[];
}

const TVLGraph: React.FC<TVLGraphProps> = ({ data }) => {
  // Take every 10th data point for plotting
  const filteredData = data.filter((_, index) => index % 60 === 0);
  const chartData = filteredData.map((item, index) => ({
    index,
    value: item.tvl,
  }));

  return (
    <div
      style={{ width: '100%', height: 380, minWidth: 480 }}
      className="w-full bg-gray-100 rounded-2xl p-4 shadow"
    >
      <ResponsiveContainer width="100%" height="100%">
        <LineChart data={chartData} margin={{ top: 10, right: 20, bottom: 30, left: 48 }}>
          <CartesianGrid stroke="#eee" strokeDasharray="5 5" />
          <XAxis dataKey="index" tick={{ fontWeight: 700 }} label={{ value: 'Minutes', position: 'insideBottom', offset: -5, style: { fontWeight: 700 } }} />
          <YAxis width={48} tick={{ fontWeight: 700 }} />
          <Tooltip />
          <Line type="monotone" dataKey="value" stroke="#8884d8" dot={{ r: 3 }} />
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
};

export default TVLGraph;

import React from "react";
import type { TVLResponse } from "../types";

interface TVLCardProps {
  tvl?: TVLResponse;
}

const TVLCard: React.FC<TVLCardProps> = ({ tvl }) => {
  if (!tvl) return <p>Loading...</p>;

  return (
    <div className="bg-white/80 backdrop-blur rounded-xl p-4 shadow-sm w-56 h-28 flex items-center justify-center border border-gray-200">
      <p className="text-gray-800 tracking-tight" style={{ fontWeight: 700 }}>
        TVL: ${tvl.tvl.toLocaleString()}
      </p>
    </div>
  );
};

export default TVLCard;

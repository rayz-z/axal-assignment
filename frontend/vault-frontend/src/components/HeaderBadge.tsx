import React from "react";

const HeaderBadge: React.FC = () => {
  return (
    <div className="inline-flex items-center gap-2 rounded-full px-4 py-2 bg-gradient-to-r from-indigo-600 to-blue-600 text-white shadow-sm">
      <span className="text-sm tracking-wide" style={{ fontWeight: 700 }}>Morpho Seamless USDC</span>
    </div>
  );
};

export default HeaderBadge;



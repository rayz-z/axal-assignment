import React from "react";

interface TVLWarningProps {
  warning: boolean;
}

const TVLWarning: React.FC<TVLWarningProps> = ({ warning }) => {
  if (!warning) {
    return null;
  }

  return (
    <div className="bg-amber-50 rounded-xl p-4 shadow-sm w-56 h-28 flex items-center justify-center border border-amber-200">
      <p className="text-amber-800 tracking-tight" style={{ fontWeight: 700 }}>Warning active</p>
    </div>
  );
};

export default TVLWarning;

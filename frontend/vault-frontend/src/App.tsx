import TVLGraph from "./components/TVLGraph";
import TVLCard from "./components/TVLCard";
import TVLWarning from "./components/TVLWarning";
import type { TVLResponse } from "./types";
import { useState } from "react";
import React, { useEffect } from "react";
import "./App.css";
import HeaderBadge from "./components/HeaderBadge";

const App: React.FC = () => {
  const [tvlData, setTvlData] = useState<TVLResponse[]>([]);
  const [hasWarningBeenTriggered, setHasWarningBeenTriggered] = useState(false);

  useEffect(() => {
    fetch("http://localhost:3000/")
      .then((res) => res.json())
      .then((data: TVLResponse[]) => setTvlData(data))
      .catch((err) => console.error("Error fetching TVL:", err));
  }, []);

  const latestTVL = tvlData.at(-1);

  // Track if warning has ever been triggered
  useEffect(() => {
    if (latestTVL?.warning) {
      setHasWarningBeenTriggered(true);
    }
  }, [latestTVL?.warning]);

  // will refresh as long as warning is still true
  useEffect(() => {
    if (latestTVL?.warning) {
      const refreshTimer = setTimeout(() => {
        window.location.reload();
      }, 5000); // refresh after 5s when warning is active
      return () => clearTimeout(refreshTimer);
    }
  }, [latestTVL?.warning]);

  return (
    <div className="parent">
      <div className="w-full">
        <TVLGraph data={tvlData} />
      </div>

      <div className="h-[380px] flex flex-col justify-between">
        <div className="flex justify-start">
          <HeaderBadge />
        </div>
        <div className="flex flex-col items-center gap-4">
          <TVLWarning warning={hasWarningBeenTriggered} />
          <TVLCard tvl={latestTVL ?? undefined} />
        </div>
      </div>
    </div>
  );
};

export default App;

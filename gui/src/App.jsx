import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import Dashboard from "./components/Dashboard";

const App = () => {
  const [stats, setStats] = useState(null);
  const [activities, setActivities] = useState([]);
  const [status, setStatus] = useState("Unknown");
  const [error, setError] = useState(null);

  const fetchData = async () => {
    try {
      const s = await invoke("get_dashboard_stats");
      setStats(s);
      setError(null);
    } catch (e) {
      console.error(e);
      setError(e.toString());
    }

    try {
      const a = await invoke("get_recent_activity");
      setActivities(a);
    } catch (e) {
      console.error(e);
    }

    try {
      const stat = await invoke("get_service_status");
      setStatus(stat);
    } catch (e) {
      console.error(e);
    }
  };

  useEffect(() => {
    fetchData();
    fetchData();
    const interval = setInterval(fetchData, 2000);
    return () => clearInterval(interval);
  }, []);

  if (error) {
    return (
      <div className="h-screen w-full bg-[#0b0c15] text-red-500 font-mono flex flex-col items-center justify-center p-10">
        <h1 className="text-2xl font-bold mb-4">SYSTEM ERROR</h1>
        <p className="border border-red-500/20 bg-red-500/10 p-4 rounded text-sm whitespace-pre-wrap max-w-2xl">
          {error}
        </p>
        <div className="mt-8 text-secondary text-xs">
          Please restart the application running as Administrator if this is a permission issue.
        </div>
      </div>
    );
  }

  return (
    <div className="h-screen w-full bg-[#0b0c15] text-white flex flex-col font-mono overflow-hidden selection:bg-accent selection:text-black">
      {/* Header */}
      <header className="h-14 border-b border-white/5 flex items-center justify-between px-6 bg-surface/30 backdrop-blur-md z-50">
        <div className="flex items-center gap-2">
          <div className="w-2 h-2 bg-accent rounded-full animate-pulse shadow-[0_0_8px_rgba(0,240,255,0.8)]"></div>
          <h1 className="text-xl font-black tracking-tighter" style={{ fontFamily: 'JetBrains Mono', letterSpacing: '-1px' }}>
            SYSTEM <span className="text-accent">//</span> MONITOR
          </h1>
        </div>

        <div className="flex items-center gap-4">
          <div className="flex items-center gap-2 px-3 py-1 bg-surface/50 rounded-full border border-white/5">
            <div className={`w-1.5 h-1.5 rounded-full ${status === 'Running' ? 'bg-green-500' : 'bg-red-500'}`}></div>
            <span className="text-[10px] font-bold text-secondary uppercase tracking-widest">{status}</span>
          </div>
          <span className="text-[10px] text-secondary font-mono opacity-50">LOCAL_NODE_01</span>
        </div>
      </header>

      {/* Main Content */}
      <main className="flex-1 p-6 relative overflow-hidden">
        <div className="absolute inset-0 bg-transparent opacity-20 pointer-events-none"
          style={{ backgroundImage: 'radial-gradient(circle at 50% 50%, #1a1b26 1px, transparent 1px)', backgroundSize: '24px 24px' }}>
        </div>

        <div className="relative z-10 h-full">
          <Dashboard stats={stats} activities={activities} />
        </div>
      </main>
    </div>
  );
};

export default App;

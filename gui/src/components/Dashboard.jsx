import React from 'react';

// --- SVGs & Icons ---
const IconCPU = () => (
  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" /></svg>
);
const IconActivity = () => (
  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
);

// --- Components ---
const Gauge = ({ value, max = 240, label, sublabel }) => {
  const radius = 60;
  const circumference = 2 * Math.PI * radius;
  const progress = Math.min(value / max, 1);
  const offset = circumference - progress * circumference;

  return (
    <div className="relative flex flex-col items-center justify-center p-6 bg-surface/40 rounded-3xl border border-white/5 shadow-2xl backdrop-blur-sm">
      <div className="relative w-48 h-48 flex items-center justify-center">
        {/* Glow */}
        <div className="absolute inset-0 bg-accent/20 blur-3xl rounded-full opacity-20"></div>

        <svg className="w-full h-full transform -rotate-90">
          <circle cx="96" cy="96" r={radius} stroke="#1e293b" strokeWidth="12" fill="none" />
          <circle
            cx="96" cy="96" r={radius}
            stroke="#00f0ff"
            strokeWidth="12"
            fill="none"
            strokeDasharray={circumference}
            strokeDashoffset={offset}
            strokeLinecap="round"
            className="transition-all duration-1000 ease-out"
            style={{ filter: "drop-shadow(0 0 4px #00f0ff)" }}
          />
        </svg>
        <div className="absolute inset-0 flex flex-col items-center justify-center text-center">
          <span className="text-5xl font-black text-white tracking-tighter" style={{ fontFamily: 'JetBrains Mono' }}>
            {value}
          </span>
          <span className="text-xs text-secondary mt-1 uppercase tracking-widest">{label}</span>
        </div>
      </div>
      <div className="mt-4 text-center">
        <div className="text-xs font-mono text-accent">Efficiency Nominal</div>
        <div className="text-[10px] text-secondary mt-1">Target: {max} min</div>
      </div>
    </div>
  );
};

const StatBox = ({ label, value, color = "text-white" }) => (
  <div className="bg-surface/30 p-4 rounded-xl border border-white/5 flex flex-col justify-between h-24">
    <span className="text-[10px] text-secondary uppercase tracking-widest font-bold">{label}</span>
    <span className={`text-3xl font-mono ${color}`}>{value}</span>
  </div>
);

const ActivityRow = ({ activity }) => {
  const isProd = activity.category?.toLowerCase() === 'productive';
  const isDist = activity.category?.toLowerCase() === 'distracting';

  return (
    <div className="flex items-center justify-between p-3 bg-surface/20 border-l-2 border-transparent hover:border-accent hover:bg-surface/40 transition-all duration-200 group rounded-r-lg mb-1">
      <div className="flex items-center gap-3 overflow-hidden">
        <div className={`p-2 rounded-lg ${isProd ? 'bg-accent/10 text-accent' : isDist ? 'bg-red-500/10 text-red-400' : 'bg-white/5 text-secondary'}`}>
          <IconActivity />
        </div>
        <div className="min-w-0">
          <div className="text-xs font-bold text-white truncate group-hover:text-primary transition-colors">{activity.app}</div>
          <div className="text-[10px] text-secondary truncate">{activity.title}</div>
        </div>
      </div>
      <div className="text-right pl-4">
        <div className="text-xs font-mono text-white/80">{Math.floor(activity.duration / 60)}m {activity.duration % 60}s</div>
        <div className={`text-[9px] uppercase font-bold tracking-wider ${isProd ? 'text-accent' : isDist ? 'text-red-500' : 'text-secondary'}`}>
          {activity.category}
        </div>
      </div>
    </div>
  );
};

const Dashboard = ({ stats, activities }) => {
  if (!stats) return <div className="h-full flex items-center justify-center text-secondary font-mono animate-pulse">Initializing System Link...</div>;

  return (
    <div className="grid grid-cols-1 lg:grid-cols-12 gap-6 h-full">
      {/* Left Column: Metrics */}
      <div className="lg:col-span-8 flex flex-col gap-6">
        {/* Header Stats */}
        <div className="grid grid-cols-4 gap-4">
          <StatBox label="Streak" value={stats.streak} color="text-accent" />
          <StatBox label="High Score" value={stats.best_streak} />
          <StatBox label="Distraction" value={`${stats.today_distraction_min}m`} color="text-red-400" />
          <StatBox label="Status" value={stats.today_productivity_min >= 120 ? "QUALIFIED" : "PENDING"} color={stats.today_productivity_min >= 120 ? "text-accent" : "text-yellow-500"} />
        </div>

        {/* Main Gauge Area */}
        <div className="flex-1 bg-surface/20 rounded-3xl border border-white/5 p-8 flex items-center justify-around relative overflow-hidden">
          <div className="absolute top-0 right-0 p-4 opacity-10 font-black text-9xl text-white select-none pointer-events-none">SYS</div>

          <div className="z-10">
            <h2 className="text-lg font-bold text-white mb-2 tracking-tight">Daily Performance</h2>
            <p className="text-secondary text-sm max-w-xs mb-6 leading-relaxed">
              Effective work is calculated by subtracting distraction penalty from raw productivity.
            </p>
            <div className={`inline-flex items-center gap-2 px-3 py-1 rounded-full text-xs font-bold ${stats.today_productivity_min >= 120 ? "bg-accent/10 text-accent border border-accent/20" : "bg-yellow-500/10 text-yellow-500 border border-yellow-500/20"}`}>
              <span className="w-2 h-2 rounded-full bg-current animate-pulse"></span>
              {stats.today_productivity_min >= 120 ? "TARGET MET" : "ACQUIRING..."}
            </div>
          </div>

          <Gauge value={stats.today_productivity_min} max={120} label="MINUTES" />
        </div>
      </div>

      {/* Right Column: Activity Feed */}
      <div className="lg:col-span-4 bg-surface/20 rounded-3xl border border-white/5 flex flex-col overflow-hidden">
        <div className="p-5 border-b border-white/5 bg-surface/30 flex justify-between items-center backdrop-blur-md">
          <h3 className="font-bold text-white text-sm tracking-wide flex items-center gap-2">
            <IconCPU />
            DETECTED PROCESSES
          </h3>
          <span className="text-[10px] font-mono text-accent">LIVE FEED</span>
        </div>
        <div className="flex-1 overflow-y-auto p-4 custom-scrollbar space-y-1">
          {activities.length === 0 ? (
            <div className="text-center text-secondary text-xs py-10">No signals detected.</div>
          ) : (
            activities.map((act, i) => <ActivityRow key={i} activity={act} />)
          )}
        </div>
      </div>
    </div>
  );
};

export default Dashboard;

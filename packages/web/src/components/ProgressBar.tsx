interface ProgressBarProps {
  value: number;
  max?: number;
  label?: string;
  size?: "sm" | "md" | "lg";
}

export default function ProgressBar({
  value,
  max = 100,
  label,
  size = "md",
}: ProgressBarProps) {
  const percentage = Math.min(Math.round((value / max) * 100), 100);

  const heightClass = {
    sm: "h-1.5",
    md: "h-2.5",
    lg: "h-4",
  }[size];

  return (
    <div className="w-full">
      {label && (
        <div className="mb-1 flex justify-between text-sm">
          <span className="text-gray-600">{label}</span>
          <span className="font-medium text-navy-500">{percentage}%</span>
        </div>
      )}
      <div className={`w-full rounded-full bg-gray-200 ${heightClass}`}>
        <div
          className={`rounded-full bg-saffron-500 ${heightClass} transition-all duration-300`}
          style={{ width: `${percentage}%` }}
        />
      </div>
    </div>
  );
}

import { Link } from "react-router-dom";
import type { Module } from "@/types";
import ProgressBar from "./ProgressBar";

interface ModuleCardProps {
  module: Module;
  languageId: string;
}

const skillIcons: Record<string, string> = {
  listening: "🎧",
  speaking: "🗣",
  reading: "📖",
  writing: "✍",
  grammar: "📝",
  vocabulary: "📚",
};

export default function ModuleCard({ module, languageId }: ModuleCardProps) {
  const progress =
    module.totalLessons > 0
      ? (module.completedLessons / module.totalLessons) * 100
      : 0;

  return (
    <div className="rounded-lg border border-gray-200 bg-white p-5 shadow-sm">
      <div className="mb-2 flex items-center gap-3">
        <span className="text-2xl" role="img" aria-label={module.skillType}>
          {skillIcons[module.skillType] || "📘"}
        </span>
        <div>
          <h3 className="font-semibold text-gray-900">{module.title}</h3>
          <span className="text-xs uppercase tracking-wide text-saffron-600">
            {module.skillType}
          </span>
        </div>
      </div>
      <p className="mb-3 text-sm text-gray-600">{module.description}</p>
      <div className="mb-3 text-sm text-gray-500">
        {module.completedLessons} / {module.totalLessons} lessons completed
      </div>
      <ProgressBar value={progress} size="sm" />
      <Link
        to={`/languages/${languageId}/curriculum`}
        className="mt-3 inline-block text-sm font-medium text-navy-500 hover:text-navy-600"
      >
        View Lessons →
      </Link>
    </div>
  );
}

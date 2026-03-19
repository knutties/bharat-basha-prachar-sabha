import type { Language } from "@/types";

interface LanguageCardProps {
  language: Language;
  onEnroll?: (languageId: string) => void;
  enrolled?: boolean;
}

export default function LanguageCard({
  language,
  onEnroll,
  enrolled = false,
}: LanguageCardProps) {
  return (
    <div className="rounded-lg border border-gray-200 bg-white p-6 shadow-sm transition-shadow hover:shadow-md">
      <div className="mb-3 text-center text-3xl font-bold text-navy-500">
        {language.scriptSample}
      </div>
      <h3 className="text-lg font-semibold text-gray-900">{language.name}</h3>
      <p className="text-sm text-gray-500">{language.nativeName}</p>
      <div className="mt-2 space-y-1 text-sm text-gray-600">
        <p>Script: {language.script}</p>
        <p>Region: {language.region}</p>
        <p>
          Grades {language.gradeRange.min}–{language.gradeRange.max}
        </p>
      </div>
      {onEnroll && (
        <button
          onClick={() => onEnroll(language.id)}
          disabled={enrolled || !language.available}
          className="mt-4 w-full rounded-md bg-saffron-500 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-saffron-600 disabled:cursor-not-allowed disabled:bg-gray-300"
        >
          {enrolled ? "Enrolled" : language.available ? "Enroll" : "Coming Soon"}
        </button>
      )}
    </div>
  );
}

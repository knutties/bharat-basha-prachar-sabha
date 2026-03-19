import { useState } from "react";
import LanguageCard from "@/components/LanguageCard";
import type { Language } from "@/types";

// Stub data for scaffold demonstration
const stubLanguages: Language[] = [
  {
    id: "hindi",
    name: "Hindi",
    nativeName: "हिन्दी",
    script: "Devanagari",
    scriptSample: "नमस्ते",
    region: "North India",
    gradeRange: { min: 1, max: 8 },
    description: "The most widely spoken language in India.",
    available: true,
  },
  {
    id: "tamil",
    name: "Tamil",
    nativeName: "தமிழ்",
    script: "Tamil",
    scriptSample: "வணக்கம்",
    region: "South India",
    gradeRange: { min: 1, max: 8 },
    description: "One of the oldest classical languages in the world.",
    available: true,
  },
  {
    id: "bengali",
    name: "Bengali",
    nativeName: "বাংলা",
    script: "Bengali-Assamese",
    scriptSample: "নমস্কার",
    region: "East India",
    gradeRange: { min: 1, max: 8 },
    description: "The language of Rabindranath Tagore.",
    available: true,
  },
  {
    id: "telugu",
    name: "Telugu",
    nativeName: "తెలుగు",
    script: "Telugu",
    scriptSample: "నమస్కారం",
    region: "South India",
    gradeRange: { min: 1, max: 8 },
    description: "Known as the 'Italian of the East'.",
    available: true,
  },
  {
    id: "marathi",
    name: "Marathi",
    nativeName: "मराठी",
    script: "Devanagari",
    scriptSample: "नमस्कार",
    region: "West India",
    gradeRange: { min: 1, max: 8 },
    description: "The official language of Maharashtra.",
    available: true,
  },
  {
    id: "kannada",
    name: "Kannada",
    nativeName: "ಕನ್ನಡ",
    script: "Kannada",
    scriptSample: "ನಮಸ್ಕಾರ",
    region: "South India",
    gradeRange: { min: 1, max: 8 },
    description: "A classical Dravidian language with rich literary tradition.",
    available: false,
  },
];

export default function LanguagesPage() {
  const [enrolledIds, setEnrolledIds] = useState<Set<string>>(new Set());

  function handleEnroll(languageId: string) {
    // In production this would call enrollInLanguage API
    setEnrolledIds((prev) => new Set([...prev, languageId]));
  }

  return (
    <div className="mx-auto max-w-7xl px-4 py-8">
      <div className="mb-8">
        <h1 className="text-2xl font-bold text-gray-900">
          Available Languages
        </h1>
        <p className="text-gray-600">
          Choose a language to start your mother-tongue learning journey.
        </p>
      </div>

      <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
        {stubLanguages.map((language) => (
          <LanguageCard
            key={language.id}
            language={language}
            onEnroll={handleEnroll}
            enrolled={enrolledIds.has(language.id)}
          />
        ))}
      </div>
    </div>
  );
}

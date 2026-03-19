import { useParams } from "react-router-dom";
import ModuleCard from "@/components/ModuleCard";
import type { Module } from "@/types";

// Stub data for scaffold demonstration
const stubModules: Module[] = [
  {
    id: "m1",
    curriculumId: "c1",
    title: "Alphabet and Phonetics",
    description: "Learn the basic letters and sounds of the language.",
    skillType: "reading",
    order: 1,
    lessons: [],
    totalLessons: 8,
    completedLessons: 8,
  },
  {
    id: "m2",
    curriculumId: "c1",
    title: "Basic Greetings",
    description: "Common greetings and everyday phrases.",
    skillType: "speaking",
    order: 2,
    lessons: [],
    totalLessons: 6,
    completedLessons: 4,
  },
  {
    id: "m3",
    curriculumId: "c1",
    title: "Numbers and Counting",
    description: "Learn to count and use numbers in context.",
    skillType: "vocabulary",
    order: 3,
    lessons: [],
    totalLessons: 5,
    completedLessons: 2,
  },
  {
    id: "m4",
    curriculumId: "c1",
    title: "Listening Comprehension",
    description: "Understand spoken language through stories and dialogues.",
    skillType: "listening",
    order: 4,
    lessons: [],
    totalLessons: 7,
    completedLessons: 0,
  },
  {
    id: "m5",
    curriculumId: "c1",
    title: "Basic Grammar",
    description: "Sentence structure, gender, and tense basics.",
    skillType: "grammar",
    order: 5,
    lessons: [],
    totalLessons: 10,
    completedLessons: 0,
  },
  {
    id: "m6",
    curriculumId: "c1",
    title: "Writing Practice",
    description: "Practice forming letters and writing simple words.",
    skillType: "writing",
    order: 6,
    lessons: [],
    totalLessons: 6,
    completedLessons: 0,
  },
];

export default function CurriculumPage() {
  const { id: languageId } = useParams<{ id: string }>();

  return (
    <div className="mx-auto max-w-7xl px-4 py-8">
      <div className="mb-8">
        <h1 className="text-2xl font-bold text-gray-900">Curriculum</h1>
        <p className="text-gray-600">
          Modules for language: <span className="font-medium">{languageId}</span>
        </p>
      </div>

      <div className="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
        {stubModules.map((mod) => (
          <ModuleCard
            key={mod.id}
            module={mod}
            languageId={languageId || ""}
          />
        ))}
      </div>
    </div>
  );
}

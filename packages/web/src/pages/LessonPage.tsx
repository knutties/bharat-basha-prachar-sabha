import { useParams, Link } from "react-router-dom";

export default function LessonPage() {
  const { id: lessonId } = useParams<{ id: string }>();

  return (
    <div className="mx-auto max-w-4xl px-4 py-8">
      {/* Lesson header */}
      <div className="mb-6">
        <Link
          to="/dashboard"
          className="mb-2 inline-block text-sm text-gray-500 hover:text-gray-700"
        >
          ← Back to Dashboard
        </Link>
        <h1 className="text-2xl font-bold text-gray-900">
          Lesson {lessonId}
        </h1>
        <p className="text-gray-600">Module: Basic Greetings</p>
      </div>

      {/* Lesson content area */}
      <div className="mb-8 rounded-lg border border-gray-200 bg-white p-8">
        <div className="flex min-h-[300px] items-center justify-center text-center">
          <div>
            <p className="mb-2 text-lg text-gray-400">Lesson Content Area</p>
            <p className="text-sm text-gray-400">
              This area will display lesson content including text, audio, video,
              and interactive exercises.
            </p>
          </div>
        </div>
      </div>

      {/* Navigation */}
      <div className="flex items-center justify-between">
        <button className="rounded-md border border-gray-300 px-4 py-2 text-sm text-gray-600 hover:bg-gray-50">
          ← Previous Lesson
        </button>
        <div className="text-sm text-gray-500">Lesson 1 of 6</div>
        <button className="rounded-md bg-saffron-500 px-4 py-2 text-sm font-medium text-white hover:bg-saffron-600">
          Next Lesson →
        </button>
      </div>
    </div>
  );
}

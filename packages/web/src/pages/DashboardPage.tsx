import { Link } from "react-router-dom";
import { useAuth } from "@/hooks/useAuth";
import ProgressBar from "@/components/ProgressBar";
import type { Enrollment } from "@/types";

// Stub data for scaffold demonstration
const stubEnrollments: Enrollment[] = [
  {
    id: "1",
    studentId: "s1",
    languageId: "hindi",
    languageName: "Hindi",
    grade: 6,
    status: "active",
    progress: 45,
    enrolledAt: "2026-01-15T00:00:00Z",
    lastActivityAt: "2026-03-18T00:00:00Z",
  },
  {
    id: "2",
    studentId: "s1",
    languageId: "tamil",
    languageName: "Tamil",
    grade: 6,
    status: "active",
    progress: 20,
    enrolledAt: "2026-02-01T00:00:00Z",
    lastActivityAt: "2026-03-17T00:00:00Z",
  },
];

export default function DashboardPage() {
  const { user } = useAuth();

  return (
    <div className="mx-auto max-w-7xl px-4 py-8">
      <div className="mb-8">
        <h1 className="text-2xl font-bold text-gray-900">
          Welcome back, {user?.name || "Student"}
        </h1>
        <p className="text-gray-600">Here is your learning progress.</p>
      </div>

      {/* Enrolled Languages */}
      <section className="mb-10">
        <h2 className="mb-4 text-lg font-semibold text-gray-900">
          Enrolled Languages
        </h2>
        <div className="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
          {stubEnrollments.map((enrollment) => (
            <div
              key={enrollment.id}
              className="rounded-lg border border-gray-200 bg-white p-5 shadow-sm"
            >
              <div className="mb-3 flex items-center justify-between">
                <h3 className="text-lg font-semibold text-navy-500">
                  {enrollment.languageName}
                </h3>
                <span className="rounded-full bg-green-100 px-2 py-0.5 text-xs font-medium text-green-700">
                  {enrollment.status}
                </span>
              </div>
              <p className="mb-3 text-sm text-gray-500">
                Grade {enrollment.grade}
              </p>
              <ProgressBar
                value={enrollment.progress}
                label="Overall Progress"
                size="md"
              />
              <Link
                to={`/languages/${enrollment.languageId}/curriculum`}
                className="mt-4 inline-block text-sm font-medium text-saffron-600 hover:text-saffron-700"
              >
                Continue Learning →
              </Link>
            </div>
          ))}
        </div>
      </section>

      {/* Recent Activity */}
      <section>
        <h2 className="mb-4 text-lg font-semibold text-gray-900">
          Recent Activity
        </h2>
        <div className="rounded-lg border border-gray-200 bg-white p-5">
          <ul className="space-y-3">
            <li className="flex items-center gap-3 text-sm text-gray-600">
              <span className="h-2 w-2 rounded-full bg-green-500" />
              Completed "Basic Greetings" in Hindi
              <span className="ml-auto text-xs text-gray-400">2 hours ago</span>
            </li>
            <li className="flex items-center gap-3 text-sm text-gray-600">
              <span className="h-2 w-2 rounded-full bg-saffron-500" />
              Started "Tamil Alphabet - Vowels" module
              <span className="ml-auto text-xs text-gray-400">Yesterday</span>
            </li>
            <li className="flex items-center gap-3 text-sm text-gray-600">
              <span className="h-2 w-2 rounded-full bg-green-500" />
              Scored 85% on Hindi Vocabulary Quiz
              <span className="ml-auto text-xs text-gray-400">2 days ago</span>
            </li>
          </ul>
        </div>
      </section>
    </div>
  );
}

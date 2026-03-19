import { useState, type FormEvent } from "react";
import { Link, useNavigate } from "react-router-dom";
import * as authApi from "@/api/auth";
import type { UserRole } from "@/types";

export default function RegisterPage() {
  const [role, setRole] = useState<UserRole>("student");
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [school, setSchool] = useState("");
  const [state, setState] = useState("");
  const [grade, setGrade] = useState(6);
  const [motherTongue, setMotherTongue] = useState("");
  const [languages, setLanguages] = useState("");
  const [error, setError] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const navigate = useNavigate();

  async function handleSubmit(e: FormEvent) {
    e.preventDefault();
    setError("");
    setIsSubmitting(true);
    try {
      if (role === "student") {
        const tokens = await authApi.registerStudent({
          email,
          password,
          name,
          grade,
          school,
          state,
          motherTongue,
        });
        localStorage.setItem("access_token", tokens.accessToken);
        localStorage.setItem("refresh_token", tokens.refreshToken);
      } else {
        const tokens = await authApi.registerTeacher({
          email,
          password,
          name,
          school,
          state,
          languages: languages.split(",").map((l) => l.trim()),
        });
        localStorage.setItem("access_token", tokens.accessToken);
        localStorage.setItem("refresh_token", tokens.refreshToken);
      }
      navigate("/dashboard");
    } catch {
      setError("Registration failed. Please check your details and try again.");
    } finally {
      setIsSubmitting(false);
    }
  }

  return (
    <div className="flex min-h-[80vh] items-center justify-center px-4 py-10">
      <div className="w-full max-w-lg">
        <h1 className="mb-6 text-center text-2xl font-bold text-gray-900">
          Create an Account
        </h1>

        {/* Role Selection */}
        <div className="mb-6 flex justify-center gap-4">
          {(["student", "teacher"] as const).map((r) => (
            <button
              key={r}
              type="button"
              onClick={() => setRole(r)}
              className={`rounded-md px-6 py-2 text-sm font-medium capitalize ${
                role === r
                  ? "bg-saffron-500 text-white"
                  : "bg-gray-100 text-gray-700 hover:bg-gray-200"
              }`}
            >
              {r}
            </button>
          ))}
        </div>

        {error && (
          <div className="mb-4 rounded-md bg-red-50 p-3 text-sm text-red-600">
            {error}
          </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label htmlFor="name" className="mb-1 block text-sm font-medium text-gray-700">
              Full Name
            </label>
            <input
              id="name"
              type="text"
              required
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-saffron-500 focus:outline-none focus:ring-1 focus:ring-saffron-500"
            />
          </div>

          <div>
            <label htmlFor="reg-email" className="mb-1 block text-sm font-medium text-gray-700">
              Email
            </label>
            <input
              id="reg-email"
              type="email"
              required
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-saffron-500 focus:outline-none focus:ring-1 focus:ring-saffron-500"
            />
          </div>

          <div>
            <label htmlFor="reg-password" className="mb-1 block text-sm font-medium text-gray-700">
              Password
            </label>
            <input
              id="reg-password"
              type="password"
              required
              minLength={8}
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-saffron-500 focus:outline-none focus:ring-1 focus:ring-saffron-500"
            />
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div>
              <label htmlFor="school" className="mb-1 block text-sm font-medium text-gray-700">
                School
              </label>
              <input
                id="school"
                type="text"
                required
                value={school}
                onChange={(e) => setSchool(e.target.value)}
                className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-saffron-500 focus:outline-none focus:ring-1 focus:ring-saffron-500"
              />
            </div>
            <div>
              <label htmlFor="state" className="mb-1 block text-sm font-medium text-gray-700">
                State
              </label>
              <input
                id="state"
                type="text"
                required
                value={state}
                onChange={(e) => setState(e.target.value)}
                className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-saffron-500 focus:outline-none focus:ring-1 focus:ring-saffron-500"
              />
            </div>
          </div>

          {role === "student" && (
            <>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label htmlFor="grade" className="mb-1 block text-sm font-medium text-gray-700">
                    Grade
                  </label>
                  <select
                    id="grade"
                    value={grade}
                    onChange={(e) => setGrade(Number(e.target.value))}
                    className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-saffron-500 focus:outline-none focus:ring-1 focus:ring-saffron-500"
                  >
                    {Array.from({ length: 8 }, (_, i) => i + 1).map((g) => (
                      <option key={g} value={g}>
                        Grade {g}
                      </option>
                    ))}
                  </select>
                </div>
                <div>
                  <label htmlFor="motherTongue" className="mb-1 block text-sm font-medium text-gray-700">
                    Mother Tongue
                  </label>
                  <input
                    id="motherTongue"
                    type="text"
                    required
                    value={motherTongue}
                    onChange={(e) => setMotherTongue(e.target.value)}
                    className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-saffron-500 focus:outline-none focus:ring-1 focus:ring-saffron-500"
                    placeholder="e.g., Tamil"
                  />
                </div>
              </div>
            </>
          )}

          {role === "teacher" && (
            <div>
              <label htmlFor="languages" className="mb-1 block text-sm font-medium text-gray-700">
                Languages (comma-separated)
              </label>
              <input
                id="languages"
                type="text"
                required
                value={languages}
                onChange={(e) => setLanguages(e.target.value)}
                className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-saffron-500 focus:outline-none focus:ring-1 focus:ring-saffron-500"
                placeholder="e.g., Hindi, Tamil, Bengali"
              />
            </div>
          )}

          <button
            type="submit"
            disabled={isSubmitting}
            className="w-full rounded-md bg-saffron-500 py-2 text-sm font-medium text-white hover:bg-saffron-600 disabled:bg-gray-300"
          >
            {isSubmitting ? "Creating account..." : "Create Account"}
          </button>
        </form>

        <p className="mt-6 text-center text-sm text-gray-600">
          Already have an account?{" "}
          <Link
            to="/login"
            className="font-medium text-saffron-600 hover:text-saffron-700"
          >
            Sign in
          </Link>
        </p>
      </div>
    </div>
  );
}

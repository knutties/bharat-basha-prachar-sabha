import { Link } from "react-router-dom";
import { useAuth } from "@/hooks/useAuth";
import type { ReactNode } from "react";

export default function Layout({ children }: { children: ReactNode }) {
  const { user, isAuthenticated, logout } = useAuth();

  return (
    <div className="flex min-h-screen flex-col">
      {/* Navbar */}
      <header className="border-b border-gray-200 bg-white">
        <nav className="mx-auto flex max-w-7xl items-center justify-between px-4 py-3">
          <Link to="/" className="flex items-center gap-2">
            <span className="text-xl font-bold text-navy-500">
              Bharat Basha
            </span>
            <span className="text-sm text-saffron-500">Prachar Sabha</span>
          </Link>

          <div className="flex items-center gap-6">
            <Link
              to="/languages"
              className="text-sm text-gray-600 hover:text-navy-500"
            >
              Languages
            </Link>

            {isAuthenticated ? (
              <>
                <Link
                  to="/dashboard"
                  className="text-sm text-gray-600 hover:text-navy-500"
                >
                  Dashboard
                </Link>
                <div className="flex items-center gap-3">
                  <span className="text-sm text-gray-700">{user?.name}</span>
                  <button
                    onClick={logout}
                    className="rounded-md bg-gray-100 px-3 py-1.5 text-sm text-gray-700 hover:bg-gray-200"
                  >
                    Logout
                  </button>
                </div>
              </>
            ) : (
              <div className="flex items-center gap-3">
                <Link
                  to="/login"
                  className="text-sm text-gray-600 hover:text-navy-500"
                >
                  Login
                </Link>
                <Link
                  to="/register"
                  className="rounded-md bg-saffron-500 px-4 py-1.5 text-sm font-medium text-white hover:bg-saffron-600"
                >
                  Register
                </Link>
              </div>
            )}
          </div>
        </nav>
      </header>

      {/* Main content */}
      <main className="flex-1">{children}</main>

      {/* Footer */}
      <footer className="border-t border-gray-200 bg-navy-500 py-8 text-white">
        <div className="mx-auto max-w-7xl px-4">
          <div className="grid grid-cols-1 gap-8 md:grid-cols-3">
            <div>
              <h3 className="mb-2 font-semibold">Bharat Basha Prachar Sabha</h3>
              <p className="text-sm text-gray-300">
                Preserving and promoting Indian mother tongues through
                school-integrated learning.
              </p>
            </div>
            <div>
              <h4 className="mb-2 text-sm font-semibold">Quick Links</h4>
              <ul className="space-y-1 text-sm text-gray-300">
                <li>
                  <Link to="/languages" className="hover:text-white">
                    Languages
                  </Link>
                </li>
                <li>
                  <Link to="/register" className="hover:text-white">
                    Get Started
                  </Link>
                </li>
              </ul>
            </div>
            <div>
              <h4 className="mb-2 text-sm font-semibold">Contact</h4>
              <p className="text-sm text-gray-300">
                info@bharatbasha.org
              </p>
            </div>
          </div>
          <div className="mt-8 border-t border-navy-400 pt-4 text-center text-xs text-gray-400">
            &copy; {new Date().getFullYear()} Bharat Basha Prachar Sabha. All
            rights reserved.
          </div>
        </div>
      </footer>
    </div>
  );
}

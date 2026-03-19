import { Link } from "react-router-dom";

const features = [
  {
    title: "Mother Tongue Learning",
    description:
      "Learn your mother tongue with structured curricula aligned to school standards.",
    icon: "📚",
  },
  {
    title: "Earn School Credits",
    description:
      "Complete modules and assessments to earn credits recognized by your school.",
    icon: "🎓",
  },
  {
    title: "Multiple Indian Languages",
    description:
      "Choose from a wide range of Indian languages with native script support.",
    icon: "🗺",
  },
  {
    title: "Skill-Based Learning",
    description:
      "Build listening, speaking, reading, and writing skills progressively.",
    icon: "🎯",
  },
];

const sampleLanguages = [
  { name: "Hindi", script: "देवनागरी", sample: "नमस्ते" },
  { name: "Tamil", script: "தமிழ்", sample: "வணக்கம்" },
  { name: "Bengali", script: "বাংলা", sample: "নমস্কার" },
  { name: "Telugu", script: "తెలుగు", sample: "నమస్కారం" },
  { name: "Marathi", script: "मराठी", sample: "नमस्कार" },
  { name: "Kannada", script: "ಕನ್ನಡ", sample: "ನಮಸ್ಕಾರ" },
];

export default function HomePage() {
  return (
    <div>
      {/* Hero Section */}
      <section className="bg-gradient-to-br from-navy-500 to-navy-700 py-20 text-white">
        <div className="mx-auto max-w-7xl px-4 text-center">
          <h1 className="mb-4 text-4xl font-bold md:text-5xl">
            Learn Your Mother Tongue,
            <br />
            <span className="text-saffron-400">Earn School Credits</span>
          </h1>
          <p className="mx-auto mb-8 max-w-2xl text-lg text-gray-300">
            Bharat Basha Prachar Sabha helps Indian school students learn and
            preserve their mother tongues through structured, credit-bearing
            courses integrated with the school curriculum.
          </p>
          <div className="flex justify-center gap-4">
            <Link
              to="/register"
              className="rounded-lg bg-saffron-500 px-6 py-3 font-medium text-white hover:bg-saffron-600"
            >
              Get Started
            </Link>
            <Link
              to="/languages"
              className="rounded-lg border border-white/30 px-6 py-3 font-medium text-white hover:bg-white/10"
            >
              Explore Languages
            </Link>
          </div>
        </div>
      </section>

      {/* Features */}
      <section className="py-16">
        <div className="mx-auto max-w-7xl px-4">
          <h2 className="mb-10 text-center text-3xl font-bold text-gray-900">
            Why Bharat Basha?
          </h2>
          <div className="grid grid-cols-1 gap-8 md:grid-cols-2 lg:grid-cols-4">
            {features.map((feature) => (
              <div
                key={feature.title}
                className="rounded-lg border border-gray-100 p-6 text-center shadow-sm"
              >
                <div className="mb-3 text-3xl">{feature.icon}</div>
                <h3 className="mb-2 font-semibold text-gray-900">
                  {feature.title}
                </h3>
                <p className="text-sm text-gray-600">{feature.description}</p>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* Supported Languages */}
      <section className="bg-gray-50 py-16">
        <div className="mx-auto max-w-7xl px-4">
          <h2 className="mb-10 text-center text-3xl font-bold text-gray-900">
            Supported Languages
          </h2>
          <div className="grid grid-cols-2 gap-4 md:grid-cols-3 lg:grid-cols-6">
            {sampleLanguages.map((lang) => (
              <div
                key={lang.name}
                className="rounded-lg bg-white p-4 text-center shadow-sm"
              >
                <div className="mb-1 text-2xl font-bold text-navy-500">
                  {lang.sample}
                </div>
                <div className="font-medium text-gray-900">{lang.name}</div>
                <div className="text-xs text-gray-500">{lang.script}</div>
              </div>
            ))}
          </div>
          <div className="mt-8 text-center">
            <Link
              to="/languages"
              className="text-sm font-medium text-saffron-600 hover:text-saffron-700"
            >
              View all languages →
            </Link>
          </div>
        </div>
      </section>
    </div>
  );
}

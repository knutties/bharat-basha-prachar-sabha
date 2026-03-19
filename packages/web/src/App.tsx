import { Routes, Route } from "react-router-dom";
import Layout from "@/components/Layout";
import HomePage from "@/pages/HomePage";
import LoginPage from "@/pages/LoginPage";
import RegisterPage from "@/pages/RegisterPage";
import DashboardPage from "@/pages/DashboardPage";
import LanguagesPage from "@/pages/LanguagesPage";
import CurriculumPage from "@/pages/CurriculumPage";
import LessonPage from "@/pages/LessonPage";

function App() {
  return (
    <Layout>
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/login" element={<LoginPage />} />
        <Route path="/register" element={<RegisterPage />} />
        <Route path="/dashboard" element={<DashboardPage />} />
        <Route path="/languages" element={<LanguagesPage />} />
        <Route path="/languages/:id/curriculum" element={<CurriculumPage />} />
        <Route path="/lessons/:id" element={<LessonPage />} />
      </Routes>
    </Layout>
  );
}

export default App;

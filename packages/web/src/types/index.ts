export type UserRole = "student" | "teacher" | "admin";

export type SkillType = "listening" | "speaking" | "reading" | "writing" | "grammar" | "vocabulary";

export type LessonStatus = "locked" | "available" | "in_progress" | "completed";

export type EnrollmentStatus = "active" | "paused" | "completed" | "dropped";

export interface User {
  id: string;
  email: string;
  name: string;
  role: UserRole;
  avatarUrl?: string;
  createdAt: string;
  updatedAt: string;
}

export interface StudentProfile {
  userId: string;
  grade: number;
  school: string;
  state: string;
  motherTongue: string;
  enrolledLanguages: string[];
  totalCredits: number;
}

export interface Language {
  id: string;
  name: string;
  nativeName: string;
  script: string;
  scriptSample: string;
  region: string;
  gradeRange: { min: number; max: number };
  description: string;
  iconUrl?: string;
  available: boolean;
}

export interface Curriculum {
  id: string;
  languageId: string;
  grade: number;
  title: string;
  description: string;
  modules: Module[];
  totalLessons: number;
  estimatedHours: number;
}

export interface Module {
  id: string;
  curriculumId: string;
  title: string;
  description: string;
  skillType: SkillType;
  order: number;
  lessons: LessonSummary[];
  totalLessons: number;
  completedLessons: number;
}

export interface LessonSummary {
  id: string;
  title: string;
  order: number;
  status: LessonStatus;
  durationMinutes: number;
}

export interface Lesson {
  id: string;
  moduleId: string;
  title: string;
  description: string;
  skillType: SkillType;
  order: number;
  durationMinutes: number;
  content: LessonContent;
  status: LessonStatus;
}

export interface LessonContent {
  type: "text" | "audio" | "video" | "interactive" | "quiz";
  body: string;
  mediaUrl?: string;
  exercises?: Exercise[];
}

export interface Exercise {
  id: string;
  type: "multiple_choice" | "fill_blank" | "match" | "translate" | "speak" | "write";
  prompt: string;
  options?: string[];
  correctAnswer: string;
  hints?: string[];
}

export interface Enrollment {
  id: string;
  studentId: string;
  languageId: string;
  languageName: string;
  grade: number;
  status: EnrollmentStatus;
  progress: number;
  enrolledAt: string;
  lastActivityAt: string;
}

export interface LearningPath {
  enrollmentId: string;
  languageId: string;
  currentModuleId: string;
  currentLessonId: string;
  completedModules: string[];
  completedLessons: string[];
  nextLessonId?: string;
  overallProgress: number;
}

export interface Assessment {
  id: string;
  moduleId: string;
  title: string;
  type: "quiz" | "test" | "project" | "oral";
  totalMarks: number;
  passingMarks: number;
  durationMinutes: number;
  questions: Exercise[];
}

export interface AssessmentResult {
  assessmentId: string;
  studentId: string;
  score: number;
  totalMarks: number;
  passed: boolean;
  completedAt: string;
  feedback?: string;
}

export interface Credit {
  id: string;
  studentId: string;
  languageId: string;
  languageName: string;
  grade: number;
  credits: number;
  awardedAt: string;
  certificateUrl?: string;
}

export interface ProgressRecord {
  lessonId: string;
  studentId: string;
  status: LessonStatus;
  score?: number;
  timeSpentMinutes: number;
  completedAt?: string;
}

export interface AuthTokens {
  accessToken: string;
  refreshToken: string;
  expiresIn: number;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface RegisterStudentRequest {
  email: string;
  password: string;
  name: string;
  grade: number;
  school: string;
  state: string;
  motherTongue: string;
}

export interface RegisterTeacherRequest {
  email: string;
  password: string;
  name: string;
  school: string;
  state: string;
  languages: string[];
}

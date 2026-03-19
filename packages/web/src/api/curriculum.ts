import apiClient from "./client";
import type {
  Language,
  Curriculum,
  Module,
  Lesson,
  Enrollment,
  LearningPath,
  ProgressRecord,
} from "@/types";

export async function listLanguages(): Promise<Language[]> {
  const response = await apiClient.get<Language[]>("/languages");
  return response.data;
}

export async function getCurriculum(
  languageId: string,
  grade?: number,
): Promise<Curriculum> {
  const params = grade ? { grade } : {};
  const response = await apiClient.get<Curriculum>(
    `/languages/${languageId}/curriculum`,
    { params },
  );
  return response.data;
}

export async function getModule(moduleId: string): Promise<Module> {
  const response = await apiClient.get<Module>(`/modules/${moduleId}`);
  return response.data;
}

export async function getLesson(lessonId: string): Promise<Lesson> {
  const response = await apiClient.get<Lesson>(`/lessons/${lessonId}`);
  return response.data;
}

export async function enrollInLanguage(
  languageId: string,
): Promise<Enrollment> {
  const response = await apiClient.post<Enrollment>(
    `/languages/${languageId}/enroll`,
  );
  return response.data;
}

export async function getLearningPath(
  enrollmentId: string,
): Promise<LearningPath> {
  const response = await apiClient.get<LearningPath>(
    `/enrollments/${enrollmentId}/path`,
  );
  return response.data;
}

export async function recordProgress(
  lessonId: string,
  data: Partial<ProgressRecord>,
): Promise<ProgressRecord> {
  const response = await apiClient.post<ProgressRecord>(
    `/lessons/${lessonId}/progress`,
    data,
  );
  return response.data;
}

import apiClient from "./client";
import type {
  AuthTokens,
  LoginRequest,
  RegisterStudentRequest,
  RegisterTeacherRequest,
  User,
} from "@/types";

export async function login(data: LoginRequest): Promise<AuthTokens> {
  const response = await apiClient.post<AuthTokens>("/auth/login", data);
  return response.data;
}

export async function registerStudent(
  data: RegisterStudentRequest,
): Promise<AuthTokens> {
  const response = await apiClient.post<AuthTokens>(
    "/auth/register/student",
    data,
  );
  return response.data;
}

export async function registerTeacher(
  data: RegisterTeacherRequest,
): Promise<AuthTokens> {
  const response = await apiClient.post<AuthTokens>(
    "/auth/register/teacher",
    data,
  );
  return response.data;
}

export async function getProfile(): Promise<User> {
  const response = await apiClient.get<User>("/auth/profile");
  return response.data;
}

export async function refreshToken(token: string): Promise<AuthTokens> {
  const response = await apiClient.post<AuthTokens>("/auth/refresh", {
    refreshToken: token,
  });
  return response.data;
}

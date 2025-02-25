import { Api } from '@/data/common'
import axios from '@/lib/axios'

export interface AuthResponse {
  userId: string
  email: string
  accessToken: string
  refreshToken: string
}

export const signUp = async (req: {
  name: string
  email: string
  password: string
}): Promise<AuthResponse> => {
  const res = await axios.post<Api<AuthResponse>>('/api/v1/auth/signup', req)
  return res.data.data
}

export const signIn = async (req: { email: string; password: string }): Promise<AuthResponse> => {
  const res = await axios.post<Api<AuthResponse>>('/api/v1/auth/signin', req)
  return res.data.data
}

export const refresh = async (): Promise<AuthResponse> => {
  const res = await axios.get<Api<AuthResponse>>('/api/v1/auth/refresh')
  return res.data.data
}

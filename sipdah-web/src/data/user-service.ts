import { Api } from '@/data/common'
import axios from '@/lib/axios'

export interface UserResponse {
  id: string
  email: string
  name: string
  phoneNumber: string | null
  photoUrl: string | null
  createdAt: string
  updatedAt: string
}

export const getCurrentUser = async (): Promise<UserResponse> => {
  const res = await axios.get<Api<UserResponse>>('/api/v1/user')
  return res.data.data
}

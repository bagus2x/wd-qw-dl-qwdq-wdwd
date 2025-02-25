import defaultAxios, { AxiosError, AxiosRequestConfig, AxiosResponse } from 'axios'
import { BACKEND_BASE_URL } from '@/lib/constants'
import { toast } from 'sonner'
import { Api } from '@/data/common'
import { AuthResponse } from '@/data/auth-service'

const axios = defaultAxios.create({
  baseURL: BACKEND_BASE_URL,
  withCredentials: true,
})

export default axios

export function isAxiosError<ResponseType = unknown>(
  error: unknown
): error is AxiosError<ResponseType> {
  return defaultAxios.isAxiosError(error)
}

interface FailedQueueItem {
  resolve: (token: string) => void
  reject: (error: unknown) => void
}

let isRefreshing = false
let failedQueue: FailedQueueItem[] = []

const processQueue = (error: unknown | null, token: string | null) => {
  failedQueue.forEach((prom) => {
    if (error) {
      prom.reject(error)
    } else if (token) {
      prom.resolve(token)
    }
  })
  failedQueue = []
}

axios.interceptors.response.use(
  (response: AxiosResponse) => response,
  async (err: unknown) => {
    if (!isAxiosError(err) || !err.response || err.response.status !== 401) {
      return Promise.reject(err)
    }

    const originalRequest = err.config as AxiosRequestConfig & { _retry?: boolean }

    if (originalRequest._retry) {
      return Promise.reject(err)
    }

    if (isRefreshing) {
      return new Promise<string>((resolve, reject) => {
        failedQueue.push({ resolve, reject })
      })
        .then((token) => {
          originalRequest.headers = {
            ...originalRequest.headers,
            Authorization: `Bearer ${token}`,
          }
          return axios(originalRequest)
        })
        .catch((error) => Promise.reject(error))
    }

    originalRequest._retry = true
    isRefreshing = true

    return new Promise((resolve, reject) => {
      defaultAxios
        .get<Api<AuthResponse>>(`${BACKEND_BASE_URL}/api/v1/auth/refresh`, {
          withCredentials: true,
        })
        .then(({ data }) => {
          axios.defaults.headers.common['Authorization'] = `Bearer ${data.data.accessToken}`
          originalRequest.headers = {
            ...originalRequest.headers,
            Authorization: `Bearer ${data.data.accessToken}`,
          }
          processQueue(null, data.data.accessToken)
          resolve(axios(originalRequest))
        })
        .catch((error) => {
          processQueue(error, null)
          toast.error('Token expired', { richColors: true })
          reject(error)
        })
        .finally(() => {
          isRefreshing = false
        })
    })
  }
)

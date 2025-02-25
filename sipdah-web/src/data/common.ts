export interface Api<T = null> {
  data: T
  status: number
  message: string
}

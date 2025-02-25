import { useMutation } from '@tanstack/react-query'

import { signIn, signUp } from '@/data/auth-service'

export const useSignUp = () => {
  return useMutation({
    mutationFn: signUp,
  })
}

export const useSignIn = () => {
  return useMutation({
    mutationFn: signIn,
  })
}

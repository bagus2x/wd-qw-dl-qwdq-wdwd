import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'

import { signIn, signOut, signUp } from '@/data/auth-service'
import { getCookie } from '@/lib/cookie'

export const authHooksKeys = {
  is_signed_in: ['is_signed_in'],
}

export const useSignUp = () => {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: signUp,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: authHooksKeys['is_signed_in'] })
    },
  })
}

export const useSignIn = () => {
  return useMutation({
    mutationFn: signIn,
  })
}

export const useSignOut = () => {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: signOut,
    onSuccess: () => {
      queryClient.clear()
    },
  })
}

export const useIsSignedIn = () => {
  const { data } = useQuery({
    queryKey: authHooksKeys['is_signed_in'],
    queryFn: () => !!getCookie('is_signed_in'),
    initialData: !!getCookie('is_signed_in'),
    refetchInterval: 1,
  })
  return !!data
}
